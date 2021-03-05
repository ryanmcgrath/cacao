//! Handles the Objective-C functionality for the Toolbar module.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel, sel_impl, msg_send};

use crate::foundation::{load_or_register_class, id, BOOL, NSArray, NSString};
use crate::macos::toolbar::{TOOLBAR_PTR, ToolbarDelegate};
use crate::utils::load;

/// Retrieves and passes the allowed item identifiers for this toolbar.
extern fn allowed_item_identifiers<T: ToolbarDelegate>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    let identifiers: NSArray = toolbar.allowed_item_identifiers().iter().map(|identifier| {
        identifier.to_nsstring()
    }).collect::<Vec<id>>().into();

    identifiers.into()
}

/// Retrieves and passes the default item identifiers for this toolbar.
extern fn default_item_identifiers<T: ToolbarDelegate>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    let identifiers: NSArray = toolbar.default_item_identifiers()
        .iter()
        .map(|identifier| identifier.to_nsstring())
        .collect::<Vec<id>>()
        .into();

    identifiers.into()
}

/// Retrieves and passes the default item identifiers for this toolbar.
extern fn selectable_item_identifiers<T: ToolbarDelegate>(this: &Object, _: Sel, _: id) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);

    let identifiers: NSArray = toolbar.selectable_item_identifiers()
        .iter()
        .map(|identifier| identifier.to_nsstring())
        .collect::<Vec<id>>()
        .into();

    identifiers.into()
}

/// Loads the controller, grabs whatever item is for this identifier, and returns what the
/// Objective-C runtime needs.
extern fn item_for_identifier<T: ToolbarDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    identifier: id,
    _: BOOL
) -> id {
    let toolbar = load::<T>(this, TOOLBAR_PTR);
    let identifier = NSString::from_retained(identifier);
    
    let item = toolbar.item_for(identifier.to_str());
    unsafe {
        msg_send![&*item.objc, self]
    }
    //&mut *item.objc
}

/// Registers a `NSToolbar` subclass, and configures it to hold some ivars for various things we need
/// to store. We use it as our delegate as well, just to cut down on moving pieces.
pub(crate) fn register_toolbar_class<T: ToolbarDelegate>(instance: &T) -> *const Class {
    load_or_register_class("NSObject", instance.subclass_name(), |decl| unsafe {
        // For callbacks
        decl.add_ivar::<usize>(TOOLBAR_PTR);

        // Add callback methods
        decl.add_method(
            sel!(toolbarAllowedItemIdentifiers:),
            allowed_item_identifiers::<T> as extern fn(&Object, _, _) -> id
        );
        decl.add_method(
            sel!(toolbarDefaultItemIdentifiers:),
            default_item_identifiers::<T> as extern fn(&Object, _, _) -> id
        );
        decl.add_method(
            sel!(toolbarSelectableItemIdentifiers:),
            selectable_item_identifiers::<T> as extern fn(&Object, _, _) -> id
        );
        decl.add_method(
            sel!(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:),
            item_for_identifier::<T> as extern fn(&Object, _, _, _, _) -> id
        );
    })
}
