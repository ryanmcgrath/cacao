//! This module implements forwarding methods for standard `UIApplicationDelegate` calls. It also
//! creates a custom `UIApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

use objc::runtime::Class;

use crate::foundation::load_or_register_class_with_optional_generated_suffix;

/// Used for injecting a custom UIApplication. Currently does nothing.
pub(crate) fn register_app_class() -> *const Class {
    let should_generate_suffix = false;

    load_or_register_class_with_optional_generated_suffix("UIApplication", "RSTApplication", should_generate_suffix, |decl| {})
}
