//! Implements an NSToolbar, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Once;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSString};

use objc_id::ShareId;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::constants::TOOLBAR_PTR;
use crate::toolbar::traits::ToolbarController;
use crate::utils::{load, str_from};

/// A wrapper for `NSToolbar`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSToolbar` and associated delegate live.
pub struct Toolbar<T> {
    internal_callback_ptr: *const RefCell<T>,
    pub identifier: String,
    pub objc_controller: ShareId<Object>,
    pub controller: Rc<RefCell<T>>
}

impl<T> Toolbar<T> where T: ToolbarController + 'static {
    /// Creates a new `NSToolbar` instance, configures it appropriately, injects an `NSObject`
    /// delegate wrapper, and retains the necessary Objective-C runtime pointers.
    pub fn new<S: Into<String>>(identifier: S, controller: T) -> Self {
        let identifier = identifier.into();
        let controller = Rc::new(RefCell::new(controller));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&controller);
            Rc::into_raw(cloned)
        };

        let objc_controller = unsafe {
            let delegate_class = register_delegate_class::<T>();
            let identifier = NSString::alloc(nil).init_str(&identifier);
            let alloc: id = msg_send![delegate_class, alloc];
            let toolbar: id = msg_send![alloc, initWithIdentifier:identifier];

            (&mut *toolbar).set_ivar(TOOLBAR_PTR, internal_callback_ptr as usize);
            let _: () = msg_send![toolbar, setDelegate:toolbar];

            ShareId::from_ptr(toolbar)
        };

        Toolbar {
            internal_callback_ptr: internal_callback_ptr,
            identifier: identifier,
            objc_controller: objc_controller,
            controller: controller
        }
    }
}

impl<T> Drop for Toolbar<T> {
    /// A bit of extra cleanup for delegate callback pointers.
    fn drop(&mut self) {
        unsafe {
            let _ = Rc::from_raw(self.internal_callback_ptr);
        }
    }
}

/// Loops back to the delegate.
extern fn allowed_item_identifiers<T: ToolbarController>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    unsafe {
        let identifiers = {
            let t = toolbar.borrow();

            (*t).allowed_item_identifiers().iter().map(|identifier| {
                NSString::alloc(nil).init_str(identifier)
            }).collect::<Vec<id>>()
        };

        Rc::into_raw(toolbar);
        NSArray::arrayWithObjects(nil, &identifiers)
    }
}

/// Loops back to the delegate.
extern fn default_item_identifiers<T: ToolbarController>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    unsafe {
        let identifiers = {
            let t = toolbar.borrow();
            
            (*t).default_item_identifiers().iter().map(|identifier| {
                NSString::alloc(nil).init_str(identifier)
            }).collect::<Vec<id>>()
        };

        Rc::into_raw(toolbar);
        NSArray::arrayWithObjects(nil, &identifiers)
    }
}

/// Loops back to the delegate.
extern fn item_for_identifier<T: ToolbarController>(this: &Object, _: Sel, _: id, identifier: id, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);
    let identifier = str_from(identifier);
    
    let mut item = {
        let t = toolbar.borrow();
        let item = (*t).item_for(identifier);
        item
    };

    Rc::into_raw(toolbar);
    &mut *item.inner
}

/// Registers an `NSObject` subclass, and configures it to hold some ivars for various things we need
/// to store.
fn register_delegate_class<T: ToolbarController>() -> *const Class {
    static mut TOOLBAR_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSToolbar);
        let mut decl = ClassDecl::new("RSTToolbar", superclass).unwrap();
        
        // For callbacks
        decl.add_ivar::<usize>(TOOLBAR_PTR);

        // Add callback methods
        decl.add_method(sel!(toolbarAllowedItemIdentifiers:), allowed_item_identifiers::<T> as extern fn(&Object, _, _) -> id);
        decl.add_method(sel!(toolbarDefaultItemIdentifiers:), default_item_identifiers::<T> as extern fn(&Object, _, _) -> id);
        decl.add_method(sel!(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:), item_for_identifier::<T> as extern fn(&Object, _, _, _, _) -> id);

        TOOLBAR_CLASS = decl.register();
    });

    unsafe { TOOLBAR_CLASS }
}
