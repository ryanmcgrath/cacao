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

use crate::foundation::{id, YES, NO, NSUInteger};
use crate::dragdrop::DragInfo;
use crate::view::{VIEW_DELEGATE_PTR, ViewDelegate};
use crate::utils::load;

/// Injects an `NSView` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_image_view_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSImageView);
        let decl = ClassDecl::new("RSTImageView", superclass).unwrap();

        //decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);
    
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
