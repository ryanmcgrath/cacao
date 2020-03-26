//! Everything useful for the `WindowDelegate`. Handles injecting an `NSWindowDelegate` subclass
//! into the Objective C runtime, which loops back to give us lifecycle methods.

use std::rc::Rc;
use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel, sel_impl};

use crate::foundation::id;
use crate::utils::load;
use crate::window::{WindowDelegate, WINDOW_DELEGATE_PTR};

/// Called when an `NSWindowDelegate` receives a `windowWillClose:` event.
/// Good place to clean up memory and what not.
extern fn will_close<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    {
        let window = window.borrow();
        (*window).will_close();
    }

    Rc::into_raw(window);
}

/// Called when an `NSWindowDelegate` receives a `windowWillMove:` event.
extern fn will_move<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    {
        let window = window.borrow();
        (*window).will_move();
    }

    Rc::into_raw(window);
}

/// Called when an `NSWindowDelegate` receives a `windowDidMove:` event.
extern fn did_move<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    {
        let window = window.borrow();
        (*window).did_move();
    }

    Rc::into_raw(window);
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn did_change_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    {
        let window = window.borrow();
        (*window).did_change_screen();
    }

    Rc::into_raw(window);
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn did_change_screen_profile<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    {
        let window = window.borrow();
        (*window).did_change_screen_profile();
    }

    Rc::into_raw(window);
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeBackingProperties:` event.
extern fn did_change_backing_properties<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    {
        let window = window.borrow();
        (*window).did_change_backing_properties();
    }

    Rc::into_raw(window);
}

/// Injects an `NSWindow` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_class() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSWindow);
        let decl = ClassDecl::new("RSTWindow", superclass).unwrap();
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}

/// Injects an `NSWindowDelegate` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_class_with_delegate<T: WindowDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSWindow);
        let mut decl = ClassDecl::new("RSTWindowWithDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(WINDOW_DELEGATE_PTR);

        // Subclassed methods

        // NSWindowDelegate methods
        decl.add_method(sel!(windowWillClose:), will_close::<T> as extern fn(&Object, _, _));

        // Moving Windows
        decl.add_method(sel!(windowWillMove:), will_move::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidMove:), did_move::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidChangeScreen:), did_change_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidChangeScreenProfile:), did_change_screen_profile::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidChangeBackingProperties:), did_change_backing_properties::<T> as extern fn(&Object, _, _));
        
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
