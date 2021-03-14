//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::dragdrop::DragInfo;
use crate::foundation::{id, NSString, NSUInteger, NO, YES};
use crate::input::{TextFieldDelegate, TEXTFIELD_DELEGATE_PTR};
use crate::utils::load;

/// Called when editing this text field has ended (e.g. user pressed enter).
extern "C" fn text_did_end_editing<T: TextFieldDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    view.text_did_end_editing({
        let s = NSString::wrap(unsafe { msg_send![this, stringValue] });
        s.to_str().to_string()
    });
}

extern "C" fn text_did_begin_editing<T: TextFieldDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    view.text_did_begin_editing({
        let s = NSString::wrap(unsafe { msg_send![this, stringValue] });
        s.to_str().to_string()
    });
}

extern "C" fn text_did_change<T: TextFieldDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    view.text_did_change({
        let s = NSString::wrap(unsafe { msg_send![this, stringValue] });
        s.to_str().to_string()
    });
}

extern "C" fn text_should_begin_editing<T: TextFieldDelegate>(
    this: &mut Object,
    _: Sel,
    info: id,
) -> bool {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    view.text_should_begin_editing({
        let s = NSString::wrap(unsafe { msg_send![this, stringValue] });
        s.to_str().to_string()
    })
}

extern "C" fn text_should_end_editing<T: TextFieldDelegate>(
    this: &mut Object,
    _: Sel,
    info: id,
) -> bool {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    view.text_should_end_editing({
        let s = NSString::wrap(unsafe { msg_send![this, stringValue] });
        s.to_str().to_string()
    })
}

/// Injects an `NSTextField` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSTextField);
        let decl = ClassDecl::new("RSTTextInputField", superclass).unwrap();
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}

/// Injects an `NSTextField` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: TextFieldDelegate>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSTextField);
        // let superclass = class!(NSView);
        let mut decl = ClassDecl::new("RSTTextInputFieldWithDelegate", superclass).unwrap();

        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(TEXTFIELD_DELEGATE_PTR);

        decl.add_method(
            sel!(textDidEndEditing:),
            text_did_end_editing::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(textDidBeginEditing:),
            text_did_begin_editing::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(textDidChange:),
            text_did_change::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(textShouldBeginEditing:),
            text_should_begin_editing::<T> as extern "C" fn(&mut Object, Sel, *mut Object) -> bool,
        );
        decl.add_method(
            sel!(textShouldEndEditing:),
            text_should_end_editing::<T> as extern "C" fn(&mut Object, Sel, *mut Object) -> bool,
        );

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
