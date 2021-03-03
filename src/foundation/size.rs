use std::ffi::CStr;
use std::os::raw::c_char;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::core_graphics::base::CGFloat;
use crate::foundation::{id, to_bool, NSInteger, BOOL, NO, YES};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl NSSize {
    /// Creates a new `NSSize`.
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }

    /// Utility method for checking whether an `NSObject` is an `NSSize`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe { msg_send![obj, isKindOfClass: class!(NSSize)] };
        to_bool(result)
    }
}
