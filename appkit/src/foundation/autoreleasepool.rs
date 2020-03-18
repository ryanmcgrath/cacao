//! A lightweight wrapper around `NSAutoreleasePool`.

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

pub struct AutoReleasePool(pub Id<Object>);

impl AutoReleasePool {
    pub fn new() -> Self {
        AutoReleasePool(unsafe {
            Id::from_retained_ptr(msg_send![class!(NSAutoreleasePool), new])
        })
    }

    pub fn drain(&self) {
        let _: () = unsafe { msg_send![&*self.0, drain] };
    }
}
