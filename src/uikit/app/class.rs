//! This module implements forwarding methods for standard `UIApplicationDelegate` calls. It also
//! creates a custom `UIApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

use std::sync::Once;

use objc::class;
use objc::declare::ClassDecl;
use objc::runtime::{Class};

/// Used for injecting a custom UIApplication. Currently does nothing.
pub(crate) fn register_app_class() -> *const Class {
    static mut APP_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UIApplication);
        let decl = ClassDecl::new("RSTApplication", superclass).unwrap();
        APP_CLASS = decl.register();
    });

    unsafe {
        APP_CLASS
    }
}
