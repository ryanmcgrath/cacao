use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Bool, Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::foundation::{id, load_or_register_class, NSString, NSUInteger};
use crate::input::{TextFieldDelegate, TEXTFIELD_DELEGATE_PTR};
use crate::utils::load;

/// Called when editing this text field has ended (e.g. user pressed enter).
extern "C" fn text_did_end_editing<T: TextFieldDelegate>(this: &Object, _: Sel, _info: id) {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    let s = NSString::retain(unsafe { msg_send![this, text] });
    view.text_did_end_editing(s.to_str());
}

extern "C" fn text_did_begin_editing<T: TextFieldDelegate>(this: &Object, _: Sel, _info: id) {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    let s = NSString::retain(unsafe { msg_send![this, text] });
    view.text_did_begin_editing(s.to_str());
}

extern "C" fn text_did_change<T: TextFieldDelegate>(this: &Object, _: Sel, _info: id) {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    let s = NSString::retain(unsafe { msg_send![this, text] });
    view.text_did_change(s.to_str());
}

extern "C" fn text_should_begin_editing<T: TextFieldDelegate>(this: &Object, _: Sel, _info: id) -> Bool {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    let s = NSString::retain(unsafe { msg_send![this, text] });

    Bool::new(view.text_should_begin_editing(s.to_str()))
}

extern "C" fn text_should_end_editing<T: TextFieldDelegate>(this: &Object, _: Sel, _info: id) -> Bool {
    let view = load::<T>(this, TEXTFIELD_DELEGATE_PTR);
    let s = NSString::retain(unsafe { msg_send![this, text] });
    Bool::new(view.text_should_end_editing(s.to_str()))
}

/// Injects an `UITextField` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> &'static Class {
    static mut VIEW_CLASS: Option<&'static Class> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UITextField);
        let decl = ClassDecl::new("RSTTextInputField", superclass).unwrap();
        VIEW_CLASS = Some(decl.register());
    });

    unsafe { VIEW_CLASS.unwrap() }
}

/// Injects an `UITextField` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: TextFieldDelegate>(instance: &T) -> &'static Class {
    load_or_register_class("UITextField", instance.subclass_name(), |decl| unsafe {
        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(TEXTFIELD_DELEGATE_PTR);

        decl.add_method(
            sel!(textFieldDidEndEditing:),
            text_did_end_editing::<T> as extern "C" fn(_, _, _)
        );
        decl.add_method(
            sel!(textFieldDidBeginEditing:),
            text_did_begin_editing::<T> as extern "C" fn(_, _, _)
        );
        decl.add_method(
            sel!(textFieldDidChangeSelection:),
            text_did_change::<T> as extern "C" fn(_, _, _)
        );
        decl.add_method(
            sel!(textFieldShouldBeginEditing:),
            text_should_begin_editing::<T> as extern "C" fn(_, _, _) -> _
        );
        decl.add_method(
            sel!(textFieldShouldEndEditing:),
            text_should_end_editing::<T> as extern "C" fn(_, _, _) -> _
        );
    })
}
