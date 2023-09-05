use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{msg_send, msg_send_id, sel};

use crate::foundation::id;
use crate::layout::constraint::LayoutConstraint;

/// A wrapper for `NSLayoutAnchorY`, used to handle values for how a given view should
/// layout along the y-axis.
#[derive(Clone, Debug)]
pub enum LayoutAnchorY {
    /// Represents an uninitialized anchor (e.g, for a view that's not created yet).
    Uninitialized,

    /// Represents a top anchor.
    Top(Id<Object, Shared>),

    /// Represents a bottom anchor.
    Bottom(Id<Object, Shared>),

    /// Represents a center anchor for the Y axis.
    Center(Id<Object, Shared>)
}

impl Default for LayoutAnchorY {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl LayoutAnchorY {
    /// Given a view, returns an anchor for the top anchor.
    pub(crate) fn top(view: id) -> Self {
        Self::Top(unsafe { msg_send_id![view, topAnchor].unwrap() })
    }

    /// Given a view, returns an anchor for the bottom anchor.
    pub(crate) fn bottom(view: id) -> Self {
        Self::Bottom(unsafe { msg_send_id![view, bottomAnchor].unwrap() })
    }

    /// Given a view, returns an anchor for the center Y anchor.
    pub(crate) fn center(view: id) -> Self {
        Self::Center(unsafe { msg_send_id![view, centerYAnchor].unwrap() })
    }

    /// Boilerplate for handling constraint construction and panic'ing with some more helpful
    /// messages. The goal here is to make AutoLayout slightly easier to debug when things go
    /// wrong.
    fn constraint_with<F>(&self, anchor_to: &LayoutAnchorY, handler: F) -> LayoutConstraint
    where
        F: Fn(&Id<Object, Shared>, &Id<Object, Shared>) -> id
    {
        match (self, anchor_to) {
            (Self::Top(from), Self::Top(to))
            | (Self::Top(from), Self::Bottom(to))
            | (Self::Top(from), Self::Center(to))
            | (Self::Bottom(from), Self::Bottom(to))
            | (Self::Bottom(from), Self::Top(to))
            | (Self::Bottom(from), Self::Center(to))
            | (Self::Center(from), Self::Center(to))
            | (Self::Center(from), Self::Top(to))
            | (Self::Center(from), Self::Bottom(to)) => LayoutConstraint::new(handler(from, to)),

            (Self::Uninitialized, Self::Uninitialized) => {
                panic!("Attempted to create constraints with uninitialized \"from\" and \"to\" y anchors.");
            },

            (Self::Uninitialized, _) => {
                panic!("Attempted to create constraints with an uninitialized \"from\" y anchor.");
            },

            (_, Self::Uninitialized) => {
                panic!("Attempted to create constraints with an uninitialized \"to\" y anchor.");
            }
        }
    }

    /// Return a constraint equal to another vertical anchor.
    pub fn constraint_equal_to(&self, anchor_to: &LayoutAnchorY) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![from, constraintEqualToAnchor:&**to]
        })
    }

    /// Return a constraint greater than or equal to another vertical anchor.
    pub fn constraint_greater_than_or_equal_to(&self, anchor_to: &LayoutAnchorY) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![from, constraintGreaterThanOrEqualToAnchor:&**to]
        })
    }

    /// Return a constraint less than or equal to another vertical anchor.
    pub fn constraint_less_than_or_equal_to(&self, anchor_to: &LayoutAnchorY) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![from, constraintLessThanOrEqualToAnchor:&**to]
        })
    }
}
