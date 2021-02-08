use std::{slice, str};
use std::os::raw::c_char;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, BOOL, YES, NO};

const UTF8_ENCODING: usize = 4;

/// A wrapper for `NSString`.
///
/// We can make a few safety guarantees in this module as the UTF8 code on the Foundation 
/// side is fairly battle tested.
#[derive(Debug)]
pub struct NSString(pub Id<Object>);

impl NSString {
    /// Creates a new `NSString`. Note that `NSString` lives on the heap, so this allocates
    /// accordingly.
    pub fn new(s: &str) -> Self {
        NSString(unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            //msg_send![nsstring, initWithBytesNoCopy:s.as_ptr() length:s.len() encoding:4 freeWhenDone:NO]
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr() length:s.len() encoding:UTF8_ENCODING])
        })
    }

    /// In cases where we're vended an `NSString` by the system, this can be used to wrap and
    /// retain it.
    pub fn wrap(object: id) -> Self {
        NSString(unsafe {
            Id::from_ptr(object)
        })
    }

    /// Utility method for checking whether an `NSObject` is an `NSString`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe { msg_send![obj, isKindOfClass:class!(NSString)] };

        match result {
            YES => true,
            NO => false
        }
    }

    /// Helper method for returning the UTF8 bytes for this `NSString`.
    fn bytes(&self) -> *const u8 {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.0, UTF8String];
            bytes as *const u8
        }
    }

    /// Helper method for grabbing the proper byte length for this `NSString` (the UTF8 variant).
    fn bytes_len(&self) -> usize {
        unsafe {
            msg_send![&*self.0, lengthOfBytesUsingEncoding:UTF8_ENCODING]
        } 
    }

    /// A utility method for taking an `NSString` and bridging it to a Rust `&str`.
    pub fn to_str(&self) -> &str {
        let bytes = self.bytes();
        let len = self.bytes_len();
        
        unsafe {
            let bytes = slice::from_raw_parts(bytes, len);
            str::from_utf8(bytes).unwrap()
        }
    }

    /// A utility method for taking an `NSString` and getting an owned `String` from it.
    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    /// Consumes and returns the underlying `NSString` instance.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
