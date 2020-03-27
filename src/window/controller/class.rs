//! Everything useful for the `WindowController`. Handles injecting an `NSWindowController` subclass
//! into the Objective C runtime, which loops back to give us lifecycle methods.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::Class;
use objc::class;

use crate::window::{WindowDelegate, WINDOW_DELEGATE_PTR};

/// Injects an `NSWindowController` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_controller_class<T: WindowDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSWindowController);
        let mut decl = ClassDecl::new("RSTWindowController", superclass).unwrap();
        decl.add_ivar::<usize>(WINDOW_DELEGATE_PTR);
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
