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
pub(crate) fn register_view_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();
    const CLASS_NAME: &str = "RSTView";

    if let Some(c) = Class::get(CLASS_NAME) {
        unsafe { VIEW_CLASS = c };
    } else {
        INIT.call_once(|| unsafe {
            let superclass = class!(UIView);
            let mut decl = ClassDecl::new(CLASS_NAME, superclass).unwrap();
            VIEW_CLASS = decl.register();
        });
    }

    unsafe { VIEW_CLASS }
}

/// Injects an `NSView` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: ViewDelegate>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();
    const CLASS_NAME: &str = "RSTViewWithDelegate";

    if let Some(c) = Class::get(CLASS_NAME) {
        unsafe { VIEW_CLASS = c };
    } else {
        INIT.call_once(|| unsafe {
            let superclass = class!(UIView);
            let mut decl = ClassDecl::new(CLASS_NAME, superclass).unwrap();
            decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);
            VIEW_CLASS = decl.register();
        });
    }

    unsafe { VIEW_CLASS }
}
