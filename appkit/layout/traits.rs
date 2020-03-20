//! Various traits related to controllers opting in to autolayout routines and support for view
//! heirarchies.

use objc::runtime::Object;
use objc_id::ShareId;

/// A trait that view wrappers must conform to. Enables managing the subview tree.
pub trait Layout {
    /// Returns a reference to the backing Objective-C layer. This is optional, as we try to keep
    /// the general lazy-loading approach Cocoa has. This may change in the future, and in general
    /// this shouldn't affect your code too much (if at all).
    fn get_backing_node(&self) -> ShareId<Object>;

    /// This trait should implement adding a view to the subview tree for a given view.
    fn add_subview<V: Layout>(&self, _view: &V);
}
