//! Implements an NSToolbar, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use std::sync::Once;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSString};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl};

use crate::toolbar::item::ToolbarItem;
use crate::utils::str_from;

static TOOLBAR_PTR: &str = "rstToolbarPtr";

/// A trait that you can implement to have your struct/etc act as an `NSToolbarDelegate`.
pub trait ToolbarDelegate {
    /// What items are allowed in this toolbar.
    fn allowed_item_identifiers(&self) -> Vec<&'static str>;

    /// The default items in this toolbar.
    fn default_item_identifiers(&self) -> Vec<&'static str>;

    /// For a given `identifier`, return the `ToolbarItem` that should be displayed.
    fn item_for(&self, _identifier: &str) -> ToolbarItem;
}

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
pub struct Toolbar {
    pub inner: Id<Object>,
    pub objc_delegate: Id<Object>,
    pub delegate: Box<dyn ToolbarDelegate>
}

impl Toolbar {
    /// Creates a new `NSToolbar` instance, configures it appropriately, injects an `NSObject`
    /// delegate wrapper, and retains the necessary Objective-C runtime pointers.
    pub fn new<D: ToolbarDelegate + 'static>(identifier: &str, delegate: D) -> Self {
        let inner = unsafe {
            let identifier = NSString::alloc(nil).init_str(identifier);
            let alloc: id = msg_send![Class::get("NSToolbar").unwrap(), alloc];
            let toolbar: id = msg_send![alloc, initWithIdentifier:identifier];
            Id::from_ptr(toolbar)
        };

        let toolbar_delegate = Box::new(delegate);

        let objc_delegate = unsafe {
            let delegate_class = register_delegate_class::<D>();
            let objc_delegate: id = msg_send![delegate_class, new];
            let delegate_ptr: *const D = &*toolbar_delegate;
            (&mut *objc_delegate).set_ivar(TOOLBAR_PTR, delegate_ptr as usize);
            let _: () = msg_send![&*inner, setDelegate:objc_delegate];
            Id::from_ptr(objc_delegate)
        };

        Toolbar {
            inner: inner,
            objc_delegate: objc_delegate,
            delegate: toolbar_delegate
        }
    }
}

/// Loops back to the delegate.
extern fn allowed_item_identifiers<D: ToolbarDelegate>(this: &Object, _: Sel, _: id) -> id {
    unsafe {
        let ptr: usize = *this.get_ivar(TOOLBAR_PTR);
        let toolbar = ptr as *mut D;
        let identifiers = (*toolbar).allowed_item_identifiers().iter().map(|identifier| {
            NSString::alloc(nil).init_str(identifier)
        }).collect::<Vec<id>>();

        NSArray::arrayWithObjects(nil, &identifiers)
    }
}

/// Loops back to the delegate.
extern fn default_item_identifiers<D: ToolbarDelegate>(this: &Object, _: Sel, _: id) -> id {
    unsafe {
        let ptr: usize = *this.get_ivar(TOOLBAR_PTR);
        let toolbar = ptr as *mut D;
        let identifiers = (*toolbar).default_item_identifiers().iter().map(|identifier| {
            NSString::alloc(nil).init_str(identifier)
        }).collect::<Vec<id>>();

        NSArray::arrayWithObjects(nil, &identifiers)
    }
}

/// Loops back to the delegate.
extern fn item_for_identifier<D: ToolbarDelegate>(this: &Object, _: Sel, _: id, identifier: id, _: id) -> id {
    unsafe {
        let ptr: usize = *this.get_ivar(TOOLBAR_PTR);
        let toolbar = ptr as *mut D;
        let identifier = str_from(identifier);
        let mut item = (*toolbar).item_for(identifier);
        &mut *item.inner
    }
}

/// Registers an `NSObject` subclass, and configures it to hold some ivars for various things we need
/// to store.
fn register_delegate_class<D: ToolbarDelegate>() -> *const Class {
    static mut TOOLBAR_DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("RSTToolbarDelegate", superclass).unwrap();
        
        // For callbacks
        decl.add_ivar::<usize>(TOOLBAR_PTR);

        // Add callback methods
        decl.add_method(sel!(toolbarAllowedItemIdentifiers:), allowed_item_identifiers::<D> as extern fn(&Object, _, _) -> id);
        decl.add_method(sel!(toolbarDefaultItemIdentifiers:), default_item_identifiers::<D> as extern fn(&Object, _, _) -> id);
        decl.add_method(sel!(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:), item_for_identifier::<D> as extern fn(&Object, _, _, _, _) -> id);

        TOOLBAR_DELEGATE_CLASS = decl.register();
    });

    unsafe { TOOLBAR_DELEGATE_CLASS }
}
