//! This module handles providing a special subclass of `NSApplication`.
//!
//! Now, I know what you're thinking: this is dumb.
//!
//! However, there are rare cases where this is beneficial, and by default we're doing nothing...
//! so consider this a placeholder that we might use in the future for certain things.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::Class;

/// Used for injecting a custom NSApplication. Currently does nothing.
pub(crate) fn register_app_class() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSApplication").unwrap();
        let decl = ClassDecl::new("RSTApplication", superclass).unwrap();
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
