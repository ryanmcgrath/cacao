use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::{fmt, slice, str};

use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::id;

const UTF8_ENCODING: usize = 4;

/// A wrapper for `NSString`.
///
/// We can make a few safety guarantees in this module as the UTF8 code on the Foundation
/// side is fairly battle tested.
#[derive(Debug)]
pub struct NSString<'a> {
    /// A reference to the backing `NSString`.
    pub objc: Id<Object, Owned>,
    phantom: PhantomData<&'a ()>
}

impl<'a> NSString<'a> {
    /// Creates a new `NSString`.
    pub fn new(s: &str) -> Self {
        NSString {
            objc: unsafe {
                msg_send_id![
                    msg_send_id![class!(NSString), alloc],
                    initWithBytes: s.as_ptr(),
                    length: s.len(),
                    encoding: UTF8_ENCODING,
                ]
            },

            phantom: PhantomData
        }
    }

    /// Creates a new `NSString` without copying the bytes for the passed-in string.
    pub fn no_copy(s: &'a str) -> Self {
        NSString {
            objc: unsafe {
                let nsstring = msg_send_id![class!(NSString), alloc];
                msg_send_id![
                    nsstring,
                    initWithBytesNoCopy: s.as_ptr(),
                    length: s.len(),
                    encoding: UTF8_ENCODING,
                    freeWhenDone: false,
                ]
            },

            phantom: PhantomData
        }
    }

    /// In cases where we're vended an `NSString` by the system, this can be used to wrap and
    /// retain it.
    pub fn retain(object: id) -> Self {
        NSString {
            objc: unsafe { Id::retain(object).unwrap() },
            phantom: PhantomData
        }
    }

    pub fn from_id(objc: Id<Object, Owned>) -> Self {
        Self {
            objc,
            phantom: PhantomData
        }
    }

    /// Utility method for checking whether an `NSObject` is an `NSString`.
    pub fn is(obj: id) -> bool {
        unsafe { msg_send![obj, isKindOfClass: class!(NSString)] }
    }

    /// Helper method for returning the UTF8 bytes for this `NSString`.
    fn bytes(&self) -> *const u8 {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.objc, UTF8String];
            bytes as *const u8
        }
    }

    /// Helper method for grabbing the proper byte length for this `NSString` (the UTF8 variant).
    fn bytes_len(&self) -> usize {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
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
}

impl fmt::Display for NSString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Deref for NSString<'_> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.objc
    }
}

impl DerefMut for NSString<'_> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.objc
    }
}
