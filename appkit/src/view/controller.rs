//! Hoists a basic `NSViewController`. We use `NSViewController` rather than plain `NSView` as
//! we're interested in the lifecycle methods and events.

use std::sync::Once;

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSRect};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::constants::VIEW_CONTROLLER_PTR;
use crate::geometry::Rect;
use crate::view::ViewController;
use crate::view::class::register_view_class;

/// Loads and configures ye old NSView for this controller.
extern fn load_view<T: ViewController>(this: &mut Object, _: Sel) {
    unsafe {
        let zero: NSRect = Rect::zero().into();
        let view: id = msg_send![register_view_class::<T>(), new];
        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
        let _: () = msg_send![view, setFrame:zero];
        let _: () = msg_send![this, setView:view]; 
    }
}

/// Registers an `NSViewController`.
pub fn register_controller_class<T: ViewController + 'static>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSViewController);
        let mut decl = ClassDecl::new("RSTViewController", superclass).unwrap();

        decl.add_ivar::<usize>(VIEW_CONTROLLER_PTR);

        // NSViewController
        decl.add_method(sel!(loadView), load_view::<T> as extern fn(&mut Object, _));

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
