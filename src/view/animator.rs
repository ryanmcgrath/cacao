use core_graphics::base::CGFloat;

use crate::id_shim::ShareId;
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};

use crate::foundation::id;

/// A wrapper for an animation proxy object in Cocoa that supports basic animations.
#[derive(Clone, Debug)]
pub struct ViewAnimatorProxy(pub ShareId<Object>);

impl ViewAnimatorProxy {
    pub fn new(proxy: id) -> Self {
        Self(unsafe { ShareId::from_ptr(msg_send![proxy, animator]) })
    }

    /// Sets the alpha value for the view being animated.
    pub fn set_alpha(&self, value: CGFloat) {
        unsafe {
            let _: () = msg_send![&*self.0, setAlphaValue: value];
        }
    }
}
