//! A wrapper for `NSError`.
//!
//! It attempts to be thread safe where possible, and extract the "default" usable information out of
//! an `NSError`. This might not be what you need, though, so if it's missing something... well,
//! it's up for discussion.

use std::error;
use std::fmt;

use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, NSInteger, NSString};

/// A wrapper around pieces of data extracted from `NSError`. This could be improved: right now, it
/// allocates `String` instances when theoretically it could be avoided, and we might be erasing
/// certain parts of the `NSError` object that are useful.
#[derive(Clone, Debug)]
pub struct Error {
    /// Represents the code. Some of these can be... archaic.
    pub code: usize,

    /// Represents the domain of the error.
    pub domain: String,

    /// Maps over to `[NSError localizedDescription]`.
    pub description: String
}

impl Error {
    /// Given an `NSError` (i.e, an id reference) we'll pull out the relevant information and
    /// configure this. We pull out the information as it makes the error thread safe this way,
    /// which is... easier, in some cases.
    pub fn new(error: id) -> Self {
        let (code, domain, description) = unsafe {
            let code: usize = msg_send![error, code];
            let domain = NSString::wrap(msg_send![error, domain]);
            let description = NSString::wrap(msg_send![error, localizedDescription]);

            (code, domain, description)
        };

        Error {
            code: code,
            domain: domain.to_str().to_string(),
            description: description.to_str().to_string()
        }
    }

    /// Returns a boxed `Error`.
    pub fn boxed(error: id) -> Box<Self> {
        Box::new(Error::new(error))
    }

    /// Used for cases where we need to return an `NSError` back to the system (e.g, top-level
    /// error handling). We just create a new `NSError` so the `Error` crate can be mostly
    /// thread safe.
    pub fn into_nserror(self) -> id {
        unsafe {
            let domain = NSString::new(&self.domain);
            let code = self.code as NSInteger;
            msg_send![class!(NSError), errorWithDomain:domain code:code userInfo:nil]
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for Error {}
