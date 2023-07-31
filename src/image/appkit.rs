//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use objc::runtime::Class;

use crate::foundation::load_or_register_class;

/// Injects an `NSView` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_image_view_class() -> *const Class {
    load_or_register_class("NSImageView", "RSTImageView", |decl| unsafe {
        //decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(_, _) -> _);
    })
}
