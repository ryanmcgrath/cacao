//! A wrapper for `NSLayoutAnchorX`, which is typically used to handle values for how a 
//! given view should layout along the x-axis. Of note: the only thing that can't be protected
//! against is mixing/matching incorrect left/leading and right/trailing anchors. Be careful!

use cocoa::base::id;

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::layout::constraint::LayoutConstraint;

/// A wrapper for `NSLayoutAnchor`. You should never be creating this yourself - it's more of a
/// factory/helper for creating `LayoutConstraint` objects based on your views.
#[derive(Clone, Debug, Default)]
pub struct LayoutAnchorX(pub Option<ShareId<Object>>);

impl LayoutAnchorX {
    /// An internal method for wrapping existing anchors.
    pub(crate) fn new(object: id) -> Self {
        LayoutAnchorX(Some(unsafe {
            ShareId::from_ptr(object)
        }))
    }

    /// Return a constraint equal to another horizontal anchor.
    pub fn constraint_equal_to(&self, anchor_to: &LayoutAnchorX) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintEqualToAnchor:&*to.clone()]
            }),

            _ => { panic!("Attempted to create horizontal constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint greater than or equal to another horizontal anchor.
    pub fn constraint_greater_than_or_equal_to(&self, anchor_to: &LayoutAnchorX) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintGreaterThanOrEqualToAnchor:&*to]
            }),

            _ => { panic!("Attempted to create horizontal constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint less than or equal to another horizontal anchor.
    pub fn constraint_less_than_or_equal_to(&self, anchor_to: &LayoutAnchorX) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintLessThanOrEqualToAnchor:&*to]
            }),

            _ => { panic!("Attempted to create horizontal constraints with an uninitialized anchor!"); }
        }
    }
}
