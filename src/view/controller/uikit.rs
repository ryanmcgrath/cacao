use std::sync::Once;
use std::unreachable;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::foundation::load_or_register_class;
use crate::foundation::{to_bool, BOOL};
use crate::utils::load;
use crate::view::{ViewDelegate, VIEW_DELEGATE_PTR};

/// Called when the view controller receives a `viewWillAppear:` message.
extern "C" fn will_appear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewWillAppear: animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_appear(to_bool(animated));
}

/// Called when the view controller receives a `viewDidAppear:` message.
extern "C" fn did_appear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewDidAppear: animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_appear(to_bool(animated));
}

/// Called when the view controller receives a `viewWillDisappear:` message.
extern "C" fn will_disappear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewWillDisappear: animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_disappear(to_bool(animated));
}

/// Called when the view controller receives a `viewDidDisappear:` message.
extern "C" fn did_disappear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewDidDisappear: animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_disappear(to_bool(animated));
}

/// Registers an `NSViewDelegate`.
pub(crate) fn register_view_controller_class<T: ViewDelegate + 'static>(instance: &T) -> *const Class {
    load_or_register_class("UIViewController", instance.subclass_name(), |decl| unsafe {
        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);

        decl.add_method(sel!(viewWillAppear:), will_appear::<T> as extern "C" fn(_, _, _));
        decl.add_method(sel!(viewDidAppear:), did_appear::<T> as extern "C" fn(_, _, _));
        decl.add_method(sel!(viewWillDisappear:), will_disappear::<T> as extern "C" fn(_, _, _));
        decl.add_method(sel!(viewDidDisappear:), did_disappear::<T> as extern "C" fn(_, _, _));
    })
}
