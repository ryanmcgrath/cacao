use std::sync::Once;
use std::unreachable;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{BOOL};
use crate::view::{VIEW_DELEGATE_PTR, ViewDelegate};
use crate::utils::{load, as_bool};

/// Called when the view controller receives a `viewWillAppear:` message.
extern "C" fn will_appear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewWillAppear:animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_appear(as_bool(animated));
}

/// Called when the view controller receives a `viewDidAppear:` message.
extern "C" fn did_appear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewDidAppear:animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_appear(as_bool(animated));
}

/// Called when the view controller receives a `viewWillDisappear:` message.
extern "C" fn will_disappear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewWillDisappear:animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.will_disappear(as_bool(animated));
}

/// Called when the view controller receives a `viewDidDisappear:` message.
extern "C" fn did_disappear<T: ViewDelegate>(this: &mut Object, _: Sel, animated: BOOL) {
    unsafe {
        let _: () = msg_send![super(this, class!(UIViewController)), viewDidDisappear:animated];
    }

    let controller = load::<T>(this, VIEW_DELEGATE_PTR);
    controller.did_disappear(as_bool(animated));
}

/// Registers an `NSViewDelegate`.
pub(crate) fn register_view_controller_class<T: ViewDelegate + 'static>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UIViewController);
        let mut decl = ClassDecl::new("RSTViewController", superclass).unwrap();

        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);

        decl.add_method(sel!(viewWillAppear:), will_appear::<T> as extern "C" fn(&mut Object, _, BOOL));
        decl.add_method(sel!(viewDidAppear:), did_appear::<T> as extern "C" fn(&mut Object, _, BOOL));
        decl.add_method(sel!(viewWillDisappear:), will_disappear::<T> as extern "C" fn(&mut Object, _, BOOL));
        decl.add_method(sel!(viewDidDisappear:), did_disappear::<T> as extern "C" fn(&mut Object, _, BOOL));

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
