//! A wrapper for `NSNumber`.
//!
//! There are a few places where we have to interact with this type (e.g, `NSUserDefaults`) and so
//! this type exists to wrap those unsafe operations.

use std::ffi::CStr;
use std::os::raw::c_char;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, BOOL, YES, NO, NSInteger};

/// Wrapper for a retained `NSNumber` object.
#[derive(Debug)]
pub struct NSNumber(pub Id<Object>);

impl NSNumber {
    pub fn bool(value: bool) -> Self {
        NSNumber(unsafe {
            Id::from_ptr(msg_send![class!(NSNumber), numberWithBool:match value {
                true => YES,
                false => NO
            }])
        })
    }

    pub fn integer(value: i64) -> Self {
        NSNumber(unsafe {
            Id::from_ptr(msg_send![class!(NSNumber), numberWithInteger:value as NSInteger])
        })
    }

    pub fn float(value: f64) -> Self {
        NSNumber(unsafe {
            Id::from_ptr(msg_send![class!(NSNumber), numberWithDouble:value])
        })
    }

    pub fn objc_type(&self) -> &str {
        unsafe {
            let t: *const c_char = msg_send![&*self.0, objCType];
            let slice = CStr::from_ptr(t);
            slice.to_str().unwrap()
        }
    }

    pub fn as_i64(&self) -> i64 {
        unsafe {
            let i: NSInteger = msg_send![&*self.0, integerValue];
            i as i64
        }
    }

    pub fn as_f64(&self) -> f64 {
        unsafe {
            msg_send![&*self.0, doubleValue]
        }
    }

    /// If we're vended an NSNumber from a method (e.g, `NSUserDefaults` querying) we might want to
    /// wrap it while we figure out what to do with it. This does that.
    pub fn wrap(data: id) -> Self {
        NSNumber(unsafe {
            Id::from_ptr(data)
        })
    }

    /// A helper method for determining if a given `NSObject` is an `NSNumber`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe {
            msg_send![obj, isKindOfClass:class!(NSNumber)]
        };

        match result {
            YES => true,
            NO => false,
            _ => unreachable!()
        }
    }
    
    /// Consumes and returns the underlying `NSNumber`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
