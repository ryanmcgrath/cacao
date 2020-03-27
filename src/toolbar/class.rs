//! Handles the Objective-C functionality for the Toolbar module.

use std::rc::Rc;
use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel, sel_impl};

use crate::foundation::{id, NSArray, NSString};
use crate::constants::TOOLBAR_PTR;
use crate::toolbar::traits::ToolbarController;
use crate::utils::load;

/// Retrieves and passes the allowed item identifiers for this toolbar.
extern fn allowed_item_identifiers<T: ToolbarController>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    let identifiers: NSArray = {
        let t = toolbar.borrow();
        (*t).allowed_item_identifiers().iter().map(|identifier| {
            NSString::new(identifier).into_inner()
        }).collect::<Vec<id>>().into()
    };

    Rc::into_raw(toolbar); 
    identifiers.into_inner()
}

/// Retrieves and passes the default item identifiers for this toolbar.
extern fn default_item_identifiers<T: ToolbarController>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    let identifiers: NSArray = {
        let t = toolbar.borrow();
        
        (*t).default_item_identifiers().iter().map(|identifier| {
            NSString::new(identifier).into_inner()
        }).collect::<Vec<id>>().into()
    };

    Rc::into_raw(toolbar);
    identifiers.into_inner()
}

/// Loads the controller, grabs whatever item is for this identifier, and returns what the
/// Objective-C runtime needs.
extern fn item_for_identifier<T: ToolbarController>(this: &Object, _: Sel, _: id, identifier: id, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);
    let identifier = NSString::wrap(identifier).to_str();
    
    let mut item = {
        let t = toolbar.borrow();
        let item = (*t).item_for(identifier);
        item
    };

    Rc::into_raw(toolbar);
    &mut *item.inner
}

/// Registers a `NSToolbar` subclass, and configures it to hold some ivars for various things we need
/// to store. We use it as our delegate as well, just to cut down on moving pieces.
pub(crate) fn register_toolbar_class<T: ToolbarController>() -> *const Class {
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
