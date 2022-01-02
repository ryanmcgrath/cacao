//! Implements a parent trait for the various sub-traits we use throughout Cacao. The methods
//! defined on here provide access handlers for common properties that the sub-traits need to
//! enable modifying.

use objc::runtime::Object;

use crate::foundation::id;

/// Types that implement this should provide access to their underlying root node type (e.g, the
/// view or control). Traits that have this as their super-trait can rely on this to ensure access
/// without needing to derive or do extra work elsewhere.
#[allow(unused_variables)]
pub trait ObjcAccess {
    /// Used for mutably interacting with the underlying Objective-C instance.
    /// Setters should use this.
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F);

    /// Used for checking backing properties of the underlying Objective-C instance, without
    /// needing a mutable borrow.
    ///
    /// Getters should use this.
    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R;
}
