//! This module implements forwarding methods for standard `NSApplicationDelegate` calls. It also
//! creates a custom `NSApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

use std::sync::Once;

use objc::class;
use objc::declare::ClassDecl;
use objc::runtime::Class;

/// Used for injecting a custom NSApplication. Currently does nothing.
pub(crate) fn register_app_class() -> *const Class {
    static mut APP_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();
    const CLASS_NAME: &str = "RSTApplication";

    if let Some(c) = Class::get(CLASS_NAME) {
        unsafe { APP_CLASS = c };
    } else {
        INIT.call_once(|| unsafe {
            let superclass = class!(NSApplication);
            let decl = ClassDecl::new(CLASS_NAME, superclass).unwrap();
            APP_CLASS = decl.register();
        });
    }

    unsafe { APP_CLASS }
}
