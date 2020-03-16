//! A wrapper for `NSError`, which can be (and is) bubbled up for certain calls in this library. It
//! attempts to be thread safe where possible, and extract the "default" usable information out of
//! an `NSError`. This might not be what you need, though, so if it's missing something... well,
//! it's up for discussion.

use std::error;
use std::fmt;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSInteger, NSString};
use objc::{class, msg_send, sel, sel_impl};

use crate::utils::str_from;

/// A wrapper around pieces of data extracted from `NSError`. This could be improved: right now, it
/// allocates `String` instances when theoretically it could be avoided, and we might be erasing
/// certain parts of the `NSError` object that are useful.
#[derive(Clone, Debug)]
pub struct AppKitError {
    pub code: usize,
    pub domain: String,
    pub description: String
}

impl AppKitError {
    /// Given an `NSError` (i.e, an id reference) we'll pull out the relevant information and
    /// configure this. We pull out the information as it makes the error thread safe this way,
    /// which is... easier, in some cases.
    pub fn new(error: id) -> Self {
        let (code, domain, description) = unsafe {
            let code: usize = msg_send![error, code];
            let domain: id = msg_send![error, domain];
            let description: id = msg_send![error, localizedDescription];

            (code, domain, description)
        };

        AppKitError {
            code: code,
            domain: str_from(domain).to_string(),
            description: str_from(description).to_string()
        }
    }

    pub fn boxed(error: id) -> Box<Self> {
        Box::new(AppKitError::new(error))
    }

    /// Used for cases where we need to return an `NSError` back to the system (e.g, top-level
    /// error handling). We just create a new `NSError` so the `AppKitError` crate can be mostly
    /// thread safe.
    pub fn into_nserror(self) -> id {
        unsafe {
            let domain = NSString::alloc(nil).init_str(&self.domain);
            let code = self.code as NSInteger;
            msg_send![class!(NSError), errorWithDomain:domain code:code userInfo:nil]
        }
    }
}

impl fmt::Display for AppKitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for AppKitError {}
