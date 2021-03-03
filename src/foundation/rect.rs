use std::ffi::CStr;
use std::os::raw::c_char;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::core_graphics::base::CGFloat;
use crate::foundation::{id, to_bool, NSInteger, NSPoint, NSSize, BOOL, NO, YES};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

impl NSRect {
    /// Creates a new `NSRect`.
    pub fn new(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat) -> Self {
        Self {
            origin: NSPoint::new(x, y),
            size: NSSize::new(w, h),
        }
    }

    /// Utility method for checking whether an `NSObject` is an `NSRect`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe { msg_send![obj, isKindOfClass: class!(NSRect)] };
        to_bool(result)
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    fn NSInsetRect(rect: NSRect, x: CGFloat, y: CGFloat) -> NSRect;
}
