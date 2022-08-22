//! A wrapper for `NSLayoutConstraint` that's more general in nature. You can think of this as an
//! escape hatch, if you need it (we use it for things like width and height, which aren't handled
//! by an axis).

use core_graphics::base::CGFloat;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, NO, YES};

#[cfg(all(feature = "appkit", target_os = "macos"))]
use super::LayoutConstraintAnimatorProxy;

/// A wrapper for `NSLayoutConstraint`. This both acts as a central path through which to activate
/// constraints, as well as a wrapper for layout constraints that are not axis bound (e.g, width or
/// height).
#[derive(Clone, Debug)]
pub struct LayoutConstraint {
    /// A shared pointer to the underlying view. Provided your view isn't dropped, this will always
    /// be valid.
    pub constraint: ShareId<Object>,

    /// The offset used in computing this constraint.
    pub offset: f64,

    /// The multiplier used in computing this constraint.
    pub multiplier: f64,

    /// The priority used in computing this constraint.
    pub priority: f64,

    /// An animator proxy that can be used inside animation contexts.
    /// This is currently only supported on macOS with the `appkit` feature.
    #[cfg(all(feature = "appkit", target_os = "macos"))]
    pub animator: LayoutConstraintAnimatorProxy
}

impl LayoutConstraint {
    /// An internal method for wrapping existing constraints.
    pub(crate) fn new(object: id) -> Self {
        LayoutConstraint {
            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: LayoutConstraintAnimatorProxy::new(object),

            constraint: unsafe { ShareId::from_ptr(object) },
            offset: 0.0,
            multiplier: 0.0,
            priority: 0.0
        }
    }

    /// Sets the offset for this constraint.
    pub fn offset<F: Into<f64>>(self, offset: F) -> Self {
        let offset: f64 = offset.into();
        unsafe {
            let o = offset as CGFloat;
            let _: () = msg_send![&*self.constraint, setConstant: o];
        }

        LayoutConstraint {
            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: self.animator,

            constraint: self.constraint,
            offset: offset,
            multiplier: self.multiplier,
            priority: self.priority
        }
    }

    /// Sets the offset of a borrowed constraint.
    pub fn set_offset<F: Into<f64>>(&self, offset: F) {
        let offset: f64 = offset.into();

        unsafe {
            let o = offset as CGFloat;
            let _: () = msg_send![&*self.constraint, setConstant: o];
        }
    }

    /// Set whether this constraint is active or not. If you're doing this across a batch of
    /// constraints, it's often more performant to batch-deactivate with
    /// `LayoutConstraint::deactivate()`.
    pub fn set_active(&self, active: bool) {
        unsafe {
            let _: () = msg_send![&*self.constraint, setActive:match active {
                true => YES,
                false => NO
            }];
        }
    }

    /// Call this with your batch of constraints to activate them.
    // If you're astute, you'll note that, yes... this is kind of hacking around some
    // borrowing rules with how objc_id::Id/objc_id::ShareId works. In this case, to
    // support the way autolayout constraints work over in the cocoa runtime, we need to be
    // able to clone these and pass them around... while also getting certain references to
    // them.
    //
    // I regret nothing, lol. If you have a better solution I'm all ears.
    pub fn activate(constraints: &[LayoutConstraint]) {
        let ids: Vec<&Object> = constraints.into_iter().map(|constraint| &*constraint.constraint).collect();
        unsafe {
            let constraints: id = msg_send![class!(NSArray), arrayWithObjects:ids.as_ptr() count:ids.len()];
            let _: () = msg_send![class!(NSLayoutConstraint), activateConstraints: constraints];
        }
    }

    pub fn deactivate(constraints: &[LayoutConstraint]) {
        let ids: Vec<&Object> = constraints.into_iter().map(|constraint| &*constraint.constraint).collect();
        unsafe {
            let constraints: id = msg_send![class!(NSArray), arrayWithObjects:ids.as_ptr() count:ids.len()];
            let _: () = msg_send![class!(NSLayoutConstraint), deactivateConstraints: constraints];
        }
    }
}
