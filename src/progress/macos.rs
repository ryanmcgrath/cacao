use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{class, sel, sel_impl};

/// Injects an `NSView` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_progress_indicator_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSProgressIndicator);
        let decl = ClassDecl::new("RSTProgressIndicator", superclass).unwrap();
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
