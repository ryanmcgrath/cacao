//! Hoists a basic `NSViewController`.

use std::sync::Once;

use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::foundation::load_or_register_class;
use crate::utils::load;
use crate::view::{ViewDelegate, VIEW_DELEGATE_PTR};

/// Called when the view controller receives a `viewWillAppear` message.
extern "C" fn will_appear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    unsafe {
        let _: () = msg_send![super(this, class!(NSViewController)), viewWillAppear];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_appear(false);
}

/// Called when the view controller receives a `viewDidAppear` message.
extern "C" fn did_appear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    unsafe {
        let _: () = msg_send![super(this, class!(NSViewController)), viewDidAppear];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_appear(false);
}

/// Called when the view controller receives a `viewWillDisappear` message.
extern "C" fn will_disappear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    unsafe {
        let _: () = msg_send![super(this, class!(NSViewController)), viewWillDisappear];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_disappear(false);
}

/// Called when the view controller receives a `viewDidDisappear` message.
extern "C" fn did_disappear<T: ViewDelegate>(this: &mut Object, _: Sel) {
    unsafe {
        let _: () = msg_send![super(this, class!(NSViewController)), viewDidDisappear];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_disappear(false);
}

/// Registers an `NSViewDelegate`.
pub(crate) fn register_view_controller_class<T: ViewDelegate + 'static>(instance: &T) -> &'static Class {
    load_or_register_class("NSViewController", instance.subclass_name(), |decl| unsafe {
        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);

        decl.add_method(sel!(viewWillAppear), will_appear::<T> as extern "C" fn(_, _));
        decl.add_method(sel!(viewDidAppear), did_appear::<T> as extern "C" fn(_, _));
        decl.add_method(sel!(viewWillDisappear), will_disappear::<T> as extern "C" fn(_, _));
        decl.add_method(sel!(viewDidDisappear), did_disappear::<T> as extern "C" fn(_, _));
    })
}
