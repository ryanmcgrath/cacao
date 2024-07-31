use objc::runtime::Class;

use crate::foundation::load_or_register_class;

/// Injects an `NSView` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_image_view_class() -> &'static Class {
    load_or_register_class("UIImageView", "RSTImageView", |decl| {})
}
