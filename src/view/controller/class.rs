//! Hoists a basic `NSViewController`.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel, sel_impl};

use crate::view::{VIEW_DELEGATE_PTR, ViewDelegate};
use crate::utils::load;

/// Called when the view controller receives a `viewWillAppear` message.
extern fn will_appear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_appear();
}

/// Called when the view controller receives a `viewDidAppear` message.
extern fn did_appear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_appear();
}

/// Called when the view controller receives a `viewWillDisappear` message.
extern fn will_disappear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_disappear();
}

/// Called when the view controller receives a `viewDidDisappear` message.
extern fn did_disappear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_disappear();
}

/// Registers an `NSViewDelegate`.
pub(crate) fn register_view_controller_class<T: ViewDelegate + 'static>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSViewController);
        let mut decl = ClassDecl::new("RSTViewController", superclass).unwrap();

        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);

        // NSViewDelegate
        decl.add_method(sel!(viewWillAppear), will_appear::<T> as extern fn(&mut Object, _));
        decl.add_method(sel!(viewDidAppear), did_appear::<T> as extern fn(&mut Object, _));
        decl.add_method(sel!(viewWillDisappear), will_disappear::<T> as extern fn(&mut Object, _));
        decl.add_method(sel!(viewDidDisappear), did_disappear::<T> as extern fn(&mut Object, _));

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
