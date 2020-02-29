//! Everything useful for the `WindowController`. Handles injecting an `NSWindowController` subclass
//! into the Objective C runtime, which loops back to give us lifecycle methods.

use std::sync::Once;

use cocoa::base::id;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel, sel_impl};

use crate::window::WindowController;

static WINDOW_CONTROLLER_PTR: &str = "rstWindowController";

/// Called when an `NSWindow` receives a `windowWillClose:` event.
/// Good place to clean up memory and what not.
extern fn will_close<T: WindowController>(this: &Object, _: Sel, _: id) {
    unsafe {
        let window_ptr: usize = *this.get_ivar(WINDOW_CONTROLLER_PTR);
        let window = window_ptr as *const T;
        (*window).will_close();
    }
}

/// Injects an `NSWindowController` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_controller_class<T: WindowController + 'static>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSWindowController);
        let mut decl = ClassDecl::new("RSTWindowController", superclass).unwrap();

        decl.add_ivar::<usize>(WINDOW_CONTROLLER_PTR);

        // Subclassed methods

        // NSWindowDelegate methods
        decl.add_method(sel!(windowWillClose:), will_close::<T> as extern fn(&Object, _, _));
        
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
