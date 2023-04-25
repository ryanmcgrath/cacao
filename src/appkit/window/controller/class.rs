//! Everything useful for the `WindowController`. Handles injecting an `NSWindowController` subclass
//! into the Objective C runtime, which loops back to give us lifecycle methods.

use objc::runtime::Class;

use crate::appkit::window::{WindowDelegate, WINDOW_DELEGATE_PTR};
use crate::foundation::load_or_register_class;

/// Injects an `NSWindowController` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_controller_class<T: WindowDelegate>() -> *const Class {
    load_or_register_class("NSWindowController", "RSTWindowController", |decl| unsafe {
        decl.add_ivar::<usize>(WINDOW_DELEGATE_PTR);
    })
}
