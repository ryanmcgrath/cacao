use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, sel};
use objc::rc::{Id, Owned};

use crate::dragdrop::DragInfo;
use crate::foundation::{id, NSUInteger};
use crate::utils::load;
use crate::view::{ViewDelegate, VIEW_DELEGATE_PTR};

/// Injects an `NSView` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> &'static Class {
    load_or_register_class("UIView", "RSTView", |decl| unsafe {})
}

/// Injects an `NSView` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: ViewDelegate>() -> &'static Class {
    load_or_register_class("UIView", "RSTViewWithDelegate", |decl| unsafe {
        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);
    })
}
