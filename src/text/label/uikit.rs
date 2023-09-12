use std::sync::Once;

use objc::declare::ClassBuilder;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel};

use crate::foundation::{id, NSUInteger};
use crate::text::label::{LabelDelegate, LABEL_DELEGATE_PTR};

/// Injects an `UILabel` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> &'static Class {
    static mut VIEW_CLASS: Option<&'static Class> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UILabel);
        let decl = ClassBuilder::new("RSTTextField", superclass).unwrap();
        VIEW_CLASS = Some(decl.register());
    });

    unsafe { VIEW_CLASS.unwrap() }
}

/// Injects an `UILabel` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: LabelDelegate>() -> &'static Class {
    static mut VIEW_CLASS: Option<&'static Class> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UIView);
        let mut decl = ClassBuilder::new("RSTTextFieldWithDelegate", superclass).unwrap();

        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(LABEL_DELEGATE_PTR);

        VIEW_CLASS = Some(decl.register());
    });

    unsafe { VIEW_CLASS.unwrap() }
}
