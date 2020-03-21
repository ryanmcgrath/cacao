//! A wrapper library for `NSString`, which we use throughout the framework. This is abstracted out
//! for a few reasons, but namely:
//!
//! - It's used often, so we want a decent enough API.
//! - Playing around with performance for this type is ideal, as it's a lot of heap allocation.
//!
//! End users should never need to interact with this.

use std::{slice, str};
use std::os::raw::c_char;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::id;

const UTF8_ENCODING: usize = 4;

/// Wraps an underlying `NSString`. 
#[derive(Debug)]
pub struct NSString(pub Id<Object>);

impl NSString {
    pub fn new(s: &str) -> Self {
        NSString(unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            //msg_send![nsstring, initWithBytesNoCopy:s.as_ptr() length:s.len() encoding:4 freeWhenDone:NO]
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr() length:s.len() encoding:UTF8_ENCODING])
        })
    }

    pub fn wrap(object: id) -> Self {
        NSString(unsafe {
            Id::from_ptr(object)
        })
    }

    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }

    /// A utility method for taking an `NSString` and bridging it to a Rust `&str`.
    pub fn to_str(self) -> &'static str {
        unsafe {
            let bytes = {
                let bytes: *const c_char = msg_send![&*self.0, UTF8String];
                bytes as *const u8
            };

            let len = msg_send![&*self.0, lengthOfBytesUsingEncoding:UTF8_ENCODING];
            let bytes = slice::from_raw_parts(bytes, len);
            str::from_utf8(bytes).unwrap()
        }
    }
}
