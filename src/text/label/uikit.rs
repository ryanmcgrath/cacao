use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{class, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, NSUInteger, NO, YES};
use crate::text::label::{LabelDelegate, LABEL_DELEGATE_PTR};

/// Injects an `UILabel` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UILabel);
        let decl = ClassDecl::new("RSTTextField", superclass).unwrap();
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}

/// Injects an `UILabel` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: LabelDelegate>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UIView);
        let mut decl = ClassDecl::new("RSTTextFieldWithDelegate", superclass).unwrap();

        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(LABEL_DELEGATE_PTR);

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
