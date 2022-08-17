use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::id;
use crate::layout::constraint::LayoutConstraint;

/// A wrapper for `NSLayoutAnchorX`, used to handle values for how a given view should
/// layout along the x-axis.
///
/// Of note: mismatches of incorrect left/leading and right/trailing anchors are detected at
/// runtime, and will panic - this is by design, as your UI needs to work. Be careful!
#[derive(Clone, Debug)]
pub enum LayoutAnchorX {
    /// Represents an uninitialized anchor (e.g, for a view that's not created yet).
    Uninitialized,

    /// Represents a leading anchor; side depends on system orientation.
    Leading(ShareId<Object>),

    /// Represents a left anchor.
    Left(ShareId<Object>),

    /// Represents a trailing anchor; side depends on system orientation.
    Trailing(ShareId<Object>),

    /// Represents a right anchor.
    Right(ShareId<Object>),

    /// Represents a center anchor on the X axis.
    Center(ShareId<Object>),
}

impl Default for LayoutAnchorX {
    /// Returns an uninitialized anchor by default.
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl LayoutAnchorX {
    /// Given a view, returns an anchor for the leading anchor.
    pub(crate) fn leading(view: id) -> Self {
        Self::Leading(unsafe { ShareId::from_ptr(msg_send![view, leadingAnchor]) })
    }

    /// Given a view, returns an anchor for the left anchor.
    pub(crate) fn left(view: id) -> Self {
        Self::Left(unsafe { ShareId::from_ptr(msg_send![view, leftAnchor]) })
    }

    /// Given a view, returns an anchor for the trailing anchor.
    pub(crate) fn trailing(view: id) -> Self {
        Self::Trailing(unsafe { ShareId::from_ptr(msg_send![view, trailingAnchor]) })
    }

    /// Given a view, returns an anchor for the right anchor.
    pub(crate) fn right(view: id) -> Self {
        Self::Right(unsafe { ShareId::from_ptr(msg_send![view, rightAnchor]) })
    }

    /// Given a view, returns an anchor for the right anchor.
    pub(crate) fn center(view: id) -> Self {
        Self::Center(unsafe { ShareId::from_ptr(msg_send![view, centerXAnchor]) })
    }

    /// Boilerplate for handling constraint construction and panic'ing with some more helpful
    /// messages. The goal here is to make AutoLayout slightly easier to debug when things go
    /// wrong.
    fn constraint_with<F>(&self, anchor_to: &LayoutAnchorX, handler: F) -> LayoutConstraint
    where
        F: Fn(&ShareId<Object>, &ShareId<Object>) -> id,
    {
        match (self, anchor_to) {
            // The anchors that can connect to each other. These blocks could be condensed, but are
            // kept separate for readability reasons.
            (Self::Leading(from), Self::Leading(to))
            | (Self::Leading(from), Self::Trailing(to))
            | (Self::Leading(from), Self::Center(to)) => LayoutConstraint::new(handler(from, to)),

            (Self::Trailing(from), Self::Trailing(to))
            | (Self::Trailing(from), Self::Leading(to))
            | (Self::Trailing(from), Self::Center(to)) => LayoutConstraint::new(handler(from, to)),

            (Self::Left(from), Self::Left(to)) | (Self::Left(from), Self::Right(to)) | (Self::Left(from), Self::Center(to)) => {
                LayoutConstraint::new(handler(from, to))
            },

            (Self::Right(from), Self::Right(to))
            | (Self::Right(from), Self::Left(to))
            | (Self::Right(from), Self::Center(to)) => LayoutConstraint::new(handler(from, to)),

            (Self::Center(from), Self::Center(to))
            | (Self::Center(from), Self::Leading(to))
            | (Self::Center(from), Self::Trailing(to))
            | (Self::Center(from), Self::Left(to))
            | (Self::Center(from), Self::Right(to)) => LayoutConstraint::new(handler(from, to)),

            // These anchors explicitly cannot be attached to each other, as it results in
            // undefined/unexpected layout behavior when a system has differing ltr/rtl setups.
            (Self::Leading(_), Self::Left(_)) | (Self::Left(_), Self::Leading(_)) => {
                panic!(
                    r#"
                    Attempted to attach a "leading" constraint to a "left" constraint. This will
                    result in undefined behavior for LTR and RTL system settings, and Cacao blocks this.

                    Use either left/right or leading/trailing.
                "#
                );
            },

            (Self::Leading(_), Self::Right(_)) | (Self::Right(_), Self::Leading(_)) => {
                panic!(
                    r#"
                    Attempted to attach a "leading" constraint to a "right" constraint. This will
                    result in undefined behavior for LTR and RTL system settings, and Cacao blocks this.

                    Use either left/right or leading/trailing.
                "#
                );
            },

            (Self::Trailing(_), Self::Left(_)) | (Self::Left(_), Self::Trailing(_)) => {
                panic!(
                    r#"
                    Attempted to attach a "trailing" constraint to a "left" constraint. This will
                    result in undefined behavior for LTR and RTL system settings, and Cacao blocks this.

                    Use either left/right or leading/trailing.
                "#
                );
            },

            (Self::Trailing(_), Self::Right(_)) | (Self::Right(_), Self::Trailing(_)) => {
                panic!(
                    r#"
                    Attempted to attach a "trailing" constraint to a "right" constraint. This will
                    result in undefined behavior for LTR and RTL system settings, and Cacao blocks this.

                    Use either left/right or leading/trailing.
                "#
                );
            },

            // If anything is attempted with an uninitialized anchor, then block it.
            (Self::Uninitialized, Self::Uninitialized) => {
                panic!("Attempted to create constraints with an uninitialized \"from\" and \"to\" X anchor.");
            },

            (Self::Uninitialized, _) => {
                panic!("Attempted to create constraints with an uninitialized \"from\" X anchor.");
            },

            (_, Self::Uninitialized) => {
                panic!("Attempted to create constraints with an uninitialized \"to\" X anchor.");
            },
        }
    }

    /// Return a constraint equal to another horizontal anchor.
    pub fn constraint_equal_to(&self, anchor_to: &LayoutAnchorX) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![*from, constraintEqualToAnchor:&**to]
        })
    }

    /// Return a constraint greater than or equal to another horizontal anchor.
    pub fn constraint_greater_than_or_equal_to(&self, anchor_to: &LayoutAnchorX) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![*from, constraintGreaterThanOrEqualToAnchor:&**to]
        })
    }

    /// Return a constraint less than or equal to another horizontal anchor.
    pub fn constraint_less_than_or_equal_to(&self, anchor_to: &LayoutAnchorX) -> LayoutConstraint {
        self.constraint_with(anchor_to, |from, to| unsafe {
            msg_send![*from, constraintLessThanOrEqualToAnchor:&**to]
        })
    }
}
