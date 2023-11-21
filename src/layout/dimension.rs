use objc::foundation::CGFloat;
use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, nil, NSInteger};
use crate::layout::constraint::LayoutConstraint;

use super::attributes::{LayoutAttribute, LayoutRelation};

/// A wrapper for `NSLayoutAnchor`. You should never be creating this yourself - it's more of a
/// factory/helper for creating `LayoutConstraint` objects based on your views.
#[derive(Clone, Debug, Default)]
pub struct LayoutAnchorDimension2(pub Option<Id<Object, Shared>>);

/// A wrapper for `NSLayoutAnchorDimension`, which is typically used to handle `width` and `height`
/// values for how a given view should layout.
#[derive(Clone, Debug)]
pub enum LayoutAnchorDimension {
    /// Represents an uninitialized anchor (e.g, for a view that's not created yet).
    Uninitialized,

    /// Represents a Width anchor.
    Width(Id<Object, Shared>),

    /// Represents a Height anchor.
    Height(Id<Object, Shared>)
}

impl Default for LayoutAnchorDimension {
    /// Returns an Uninitialized anchor dimension by default.
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl LayoutAnchorDimension {
    /// Given a view, returns an anchor for the width anchor.
    pub(crate) fn width(view: id) -> Self {
        Self::Width(unsafe { msg_send_id![view, widthAnchor] })
    }

    /// Given a view, returns an anchor for the height anchor.
    pub(crate) fn height(view: id) -> Self {
        Self::Height(unsafe { msg_send_id![view, heightAnchor] })
    }

    /// Return a constraint equal to a constant value.
    pub fn constraint_equal_to_constant(&self, constant: f64) -> LayoutConstraint {
        if let Self::Width(obj) | Self::Height(obj) = self {
            return LayoutConstraint::new(unsafe {
                let value = constant as CGFloat;
                msg_send![obj, constraintEqualToConstant: value]
            });
        }

        panic!("Attempted to create a constant constraint with an uninitialized anchor.");
    }

    /// Return a constraint greater than or equal to a constant value.
    pub fn constraint_greater_than_or_equal_to_constant(&self, constant: f64) -> LayoutConstraint {
        if let Self::Width(obj) | Self::Height(obj) = self {
            return LayoutConstraint::new(unsafe {
                let value = constant as CGFloat;
                msg_send![obj, constraintGreaterThanOrEqualToConstant: value]
            });
        }

        panic!("Attempted to create a constraint (>=) with an uninitialized anchor.");
    }

    /// Return a constraint greater than or equal to a constant value.
    pub fn constraint_less_than_or_equal_to_constant(&self, constant: f64) -> LayoutConstraint {
        if let Self::Width(obj) | Self::Height(obj) = self {
            return LayoutConstraint::new(unsafe {
                let value = constant as CGFloat;
                msg_send![obj, constraintLessThanOrEqualToConstant: value]
            });
        }

        panic!("Attempted to create a constraint (<=) with an uninitialized anchor.");
    }

    /// Boilerplate for handling constraint construction and panic'ing with some more helpful
    /// messages. The goal here is to make AutoLayout slightly easier to debug when things go
    /// wrong.
    fn constraint_with<F>(&self, anchor_to: &LayoutAnchorDimension, handler: F) -> LayoutConstraint
    where
        F: Fn(&Id<Object, Shared>, &Id<Object, Shared>) -> id
    {
        match (self, anchor_to) {
            (Self::Width(from), Self::Width(to))
            | (Self::Width(from), Self::Height(to))
            | (Self::Height(from), Self::Width(to))
            | (Self::Height(from), Self::Height(to)) => LayoutConstraint::new(handler(from, to)),

            (Self::Uninitialized, Self::Uninitialized) => {
                panic!("Attempted to create constraints with an uninitialized \"from\" and \"to\" dimension anchor.");
            },

            (Self::Uninitialized, _) => {
                panic!("Attempted to create constraints with an uninitialized \"from\" dimension anchor.");
            },

            (_, Self::Uninitialized) => {
                panic!("Attempted to create constraints with an uninitialized \"to\" dimension anchor.");
            }
        }
    }

    /// Return a constraint equal to another dimension anchor.
    pub fn constraint_equal_to(&self, anchor_to: &LayoutAnchorDimension) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![from, constraintEqualToAnchor:&**to]
        })
    }

    /// Return a constraint greater than or equal to another dimension anchor.
    pub fn constraint_greater_than_or_equal_to(&self, anchor_to: &LayoutAnchorDimension) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![from, constraintGreaterThanOrEqualToAnchor:&**to]
        })
    }

    /// Return a constraint less than or equal to another dimension anchor.
    pub fn constraint_less_than_or_equal_to(&self, anchor_to: &LayoutAnchorDimension) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![from, constraintLessThanOrEqualToAnchor:&**to]
        })
    }
}
