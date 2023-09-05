use core_graphics::base::CGFloat;

use objc::rc::{Id, Shared};
use objc::runtime::{Class, Object};
use objc::{msg_send, msg_send_id, sel};

use crate::foundation::id;

/// A wrapper for an animation proxy object in Cocoa that supports basic animations.
#[derive(Clone, Debug)]
pub struct ViewAnimatorProxy(pub Id<Object, Shared>);

impl ViewAnimatorProxy {
    pub fn new(proxy: id) -> Self {
        Self(unsafe { msg_send_id![proxy, animator] })
    }

    /// Sets the alpha value for the view being animated.
    pub fn set_alpha(&self, value: CGFloat) {
        unsafe {
            let _: () = msg_send![&*self.0, setAlphaValue: value];
        }
    }
}

// TODO: Safety
unsafe impl Send for ViewAnimatorProxy {}
unsafe impl Sync for ViewAnimatorProxy {}
