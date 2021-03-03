use std::ffi::CStr;
use std::os::raw::c_char;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::core_graphics::base::CGFloat;
use crate::foundation::{id, to_bool, NSInteger, BOOL, NO, YES};
use core_graphics::display::CGPoint;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl NSPoint {
    /// Creates a new `NSPoint`.
    pub fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }

    /// Utility method for checking whether an `NSObject` is an `NSPoint`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe { msg_send![obj, isKindOfClass: class!(NSPoint)] };
        to_bool(result)
    }
}
