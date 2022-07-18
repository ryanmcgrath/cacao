use core_graphics::base::CGFloat;

use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::id;

/// A wrapper for an animation proxy object in Cocoa that supports basic animations.
#[derive(Clone, Debug)]
pub struct LayoutConstraintAnimatorProxy(pub ShareId<Object>);

impl LayoutConstraintAnimatorProxy {
    /// Wraps and returns a proxy for animation of layout constraint values.
    pub fn new(proxy: id) -> Self {
        Self(unsafe { ShareId::from_ptr(msg_send![proxy, animator]) })
    }

    /// Sets the constant (usually referred to as `offset` in Cacao) value for the constraint being animated.
    pub fn set_offset(&self, value: CGFloat) {
        unsafe {
            let _: () = msg_send![&*self.0, setConstant: value];
        }
    }
}
