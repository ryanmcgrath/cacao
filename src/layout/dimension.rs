//! A wrapper for `NSLayoutAnchorDimension`, which is typically used to handle `width` and `height`
//! values for how a given view should layout.

use core_graphics::base::CGFloat;

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::id;
use crate::layout::constraint::LayoutConstraint;

/// A wrapper for `NSLayoutAnchor`. You should never be creating this yourself - it's more of a
/// factory/helper for creating `LayoutConstraint` objects based on your views.
#[derive(Clone, Debug, Default)]
pub struct LayoutAnchorDimension(pub Option<ShareId<Object>>);

impl LayoutAnchorDimension {
    /// An internal method for wrapping existing anchors.
    pub(crate) fn new(object: id) -> Self {
        LayoutAnchorDimension(Some(unsafe {
            ShareId::from_ptr(object)
        }))
    }

    /// Return a constraint equal to a constant value.
    pub fn constraint_equal_to_constant(&self, constant: f64) -> LayoutConstraint {
        match &self.0 {
            Some(from) => LayoutConstraint::new(unsafe {
                let value = constant as CGFloat;
                msg_send![*from, constraintEqualToConstant:value]
            }),

            _ => { panic!("Attempted to create constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint equal to another dimension anchor.
    pub fn constraint_equal_to(&self, anchor_to: &LayoutAnchorDimension) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintEqualToAnchor:&**to]
            }),

            _ => { panic!("Attempted to create constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint greater than or equal to another dimension anchor.
    pub fn constraint_greater_than_or_equal_to(&self, anchor_to: &LayoutAnchorDimension) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintGreaterThanOrEqualToAnchor:&**to]
            }),

            _ => { panic!("Attempted to create constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint less than or equal to another dimension anchor.
    pub fn constraint_less_than_or_equal_to(&self, anchor_to: &LayoutAnchorDimension) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintLessThanOrEqualToAnchor:&**to]
            }),

            _ => { panic!("Attempted to create constraints with an uninitialized anchor!"); }
        }
    }
}
