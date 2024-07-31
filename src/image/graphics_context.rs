use std::ffi::c_void;

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id};

#[derive(Debug)]
pub(crate) struct GraphicsContext(pub Id<Object, Shared>);

impl GraphicsContext {
    pub(crate) fn current() -> Self {
        Self(unsafe { msg_send_id![class!(NSGraphicsContext), currentContext] })
    }

    pub(crate) fn save(&self) {
        unsafe { msg_send![&self.0, saveGraphicsState] }
    }

    pub(crate) fn restore(&self) {
        unsafe { msg_send![&self.0, restoreGraphicsState] }
    }

    pub(crate) fn cg_context(&self) -> *mut c_void {
        unsafe { msg_send![&self.0, CGContext] }
    }
}
