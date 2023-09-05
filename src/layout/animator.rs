use core_graphics::base::CGFloat;

use objc::rc::{Id, Shared};
use objc::runtime::{Class, Object};
use objc::{msg_send, msg_send_id, sel};

use crate::foundation::id;

/// A wrapper for an animation proxy object in Cocoa that supports basic animations.
#[derive(Clone, Debug)]
pub struct LayoutConstraintAnimatorProxy(pub Id<Object, Shared>);

impl LayoutConstraintAnimatorProxy {
    /// Wraps and returns a proxy for animation of layout constraint values.
    pub fn new(proxy: id) -> Self {
        Self(unsafe { msg_send_id![proxy, animator].unwrap() })
    }

    /// Sets the constant (usually referred to as `offset` in Cacao) value for the constraint being animated.
    pub fn set_offset(&self, value: CGFloat) {
        unsafe {
            let _: () = msg_send![&*self.0, setConstant: value];
        }
    }
}

// TODO: Safety
unsafe impl Send for LayoutConstraintAnimatorProxy {}
unsafe impl Sync for LayoutConstraintAnimatorProxy {}
