//! Everything useful for the `WindowDelegate`. Handles injecting an `NSWindowDelegate` subclass
//! into the Objective C runtime, which loops back to give us lifecycle methods.

use std::sync::Once;

use core_graphics::base::CGFloat;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel, sel_impl};

use crate::foundation::{id, BOOL, YES, NO, NSUInteger};
use crate::utils::{load, CGSize};
use crate::window::{WindowDelegate, WINDOW_DELEGATE_PTR};

/// Called when an `NSWindowDelegate` receives a `windowWillClose:` event.
/// Good place to clean up memory and what not.
extern fn should_close<T: WindowDelegate>(this: &Object, _: Sel, _: id) -> BOOL {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    match window.should_close() {
        true => YES,
        false => NO
    }
}

/// Called when an `NSWindowDelegate` receives a `windowWillClose:` event.
/// Good place to clean up memory and what not.
extern fn will_close<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_close();
}

/// Called when an `NSWindowDelegate` receives a `windowWillMove:` event.
extern fn will_move<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_move();
}

/// Called when an `NSWindowDelegate` receives a `windowDidMove:` event.
extern fn did_move<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_move();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn did_change_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn did_change_screen_profile<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_screen_profile();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn will_resize<T: WindowDelegate>(this: &Object, _: Sel, _: id, size: CGSize) -> CGSize {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    let s = window.will_resize(size.width as f64, size.height as f64);
        
    CGSize { 
        width: s.0 as CGFloat,
        height: s.1 as CGFloat
    }
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn did_resize<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_resize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn will_start_live_resize<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_start_live_resize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn did_end_live_resize<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_end_live_resize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn will_miniaturize<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_miniaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn did_miniaturize<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_miniaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern fn did_deminiaturize<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_deminiaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn will_enter_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_enter_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn did_enter_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_enter_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn content_size_for_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id, size: CGSize) -> CGSize {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    let (width, height) = window.content_size_for_full_screen(
        size.width as f64,
        size.height as f64
    );

    CGSize {
        width: width as CGFloat,
        height: height as CGFloat
    }
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn options_for_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id, options: NSUInteger) -> NSUInteger {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    let desired_opts = window.presentation_options_for_full_screen();
        
    if desired_opts.is_none() { 
        options
    } else {
        let mut opts: NSUInteger = 0;
        for opt in desired_opts.unwrap() {
            opts = opts << NSUInteger::from(opt);
        }

        opts
    }
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn will_exit_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_exit_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn did_exit_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_exit_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn did_fail_to_enter_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_fail_to_enter_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern fn did_fail_to_exit_full_screen<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_fail_to_exit_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeBackingProperties:` event.
extern fn did_change_backing_properties<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_backing_properties();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeBackingProperties:` event.
extern fn did_change_occlusion_state<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_occlusion_state();
}

/// Called when an `NSWindowDelegate` receives a `windowDidUpdate:` event.
extern fn did_update<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_update();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern fn did_become_main<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_become_main();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern fn did_resign_main<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_resign_main();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern fn did_become_key<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_become_key();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern fn did_resign_key<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_resign_key();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern fn did_expose<T: WindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_expose();
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

        // NSWindowDelegate methods
        decl.add_method(sel!(windowShouldClose:), should_close::<T> as extern fn(&Object, _, _) -> BOOL);
        decl.add_method(sel!(windowWillClose:), will_close::<T> as extern fn(&Object, _, _));

        // Sizing
        decl.add_method(sel!(windowWillResize:toSize:), will_resize::<T> as extern fn(&Object, _, _, CGSize) -> CGSize);
        decl.add_method(sel!(windowDidResize:), did_resize::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowWillStartLiveResize:), will_start_live_resize::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidEndLiveResize:), did_end_live_resize::<T> as extern fn(&Object, _, _));

        // Minimizing
        decl.add_method(sel!(windowWillMiniaturize:), will_miniaturize::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidMiniaturize:), did_miniaturize::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidDeminiaturize:), did_deminiaturize::<T> as extern fn(&Object, _, _));

        // Full Screen
        decl.add_method(sel!(window:willUseFullScreenContentSize:), content_size_for_full_screen::<T> as extern fn(&Object, _, _, CGSize) -> CGSize);
        decl.add_method(sel!(window:willUseFullScreenPresentationOptions:), options_for_full_screen::<T> as extern fn(&Object, _, _, NSUInteger) -> NSUInteger);
        decl.add_method(sel!(windowWillEnterFullScreen:), will_enter_full_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidEnterFullScreen:), did_enter_full_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowWillExitFullScreen:), will_exit_full_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidExitFullScreen:), did_exit_full_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidFailToEnterFullScreen:), did_fail_to_enter_full_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidFailToExitFullScreen:), did_fail_to_exit_full_screen::<T> as extern fn(&Object, _, _));

        // Key status
        decl.add_method(sel!(windowDidBecomeKey:), did_become_key::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidResignKey:), did_resign_key::<T> as extern fn(&Object, _, _));

        // Main status
        decl.add_method(sel!(windowDidBecomeMain:), did_become_main::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidResignMain:), did_resign_main::<T> as extern fn(&Object, _, _));

        // Moving Windows
        decl.add_method(sel!(windowWillMove:), will_move::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidMove:), did_move::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidChangeScreen:), did_change_screen::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidChangeScreenProfile:), did_change_screen_profile::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidChangeBackingProperties:), did_change_backing_properties::<T> as extern fn(&Object, _, _));

        // Random
        decl.add_method(sel!(windowDidChangeOcclusionState:), did_change_occlusion_state::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidExpose:), did_expose::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(windowDidUpdate:), did_update::<T> as extern fn(&Object, _, _));
        
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
