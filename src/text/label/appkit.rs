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
use objc::{class, sel, sel_impl};
use objc_id::Id;

use crate::dragdrop::DragInfo;
use crate::foundation::{id, NSUInteger, NO, YES};
use crate::text::label::{LabelDelegate, LABEL_DELEGATE_PTR};
use crate::utils::load;

/// Injects an `NSTextField` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();
    const CLASS_NAME: &str = "RSTTextField";

    if let Some(c) = Class::get(CLASS_NAME) {
        unsafe { VIEW_CLASS = c };
    } else {
        INIT.call_once(|| unsafe {
            let superclass = class!(NSTextField);
            let decl = ClassDecl::new(CLASS_NAME, superclass).unwrap();
            VIEW_CLASS = decl.register();
        });
    }

    unsafe { VIEW_CLASS }
}

/// Injects an `NSTextField` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: LabelDelegate>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();
    const CLASS_NAME: &str = "RSTTextFieldWithDelegate";

    if let Some(c) = Class::get(CLASS_NAME) {
        unsafe { VIEW_CLASS = c };
    } else {
        INIT.call_once(|| unsafe {
            let superclass = class!(NSView);
            let mut decl = ClassDecl::new(CLASS_NAME, superclass).unwrap();

            // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
            // move.
            decl.add_ivar::<usize>(LABEL_DELEGATE_PTR);

            VIEW_CLASS = decl.register();
        });
    }

    unsafe { VIEW_CLASS }
}
