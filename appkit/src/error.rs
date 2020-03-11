//! A wrapper for `NSError`, which can be (and is) bubbled up for certain calls in this library. It
//! attempts to be thread safe where possible, and extract the "default" usable information out of
//! an `NSError`. This might not be what you need, though, so if it's missing something... well,
//! it's up for discussion.

use std::error;
use std::fmt;

use cocoa::base::id;
use objc::{msg_send, sel, sel_impl};

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
    pub fn new(error: id) -> Box<Self> {
        let (code, domain, description) = unsafe {
            let code: usize = msg_send![error, code];
            let domain: id = msg_send![error, domain];
            let description: id = msg_send![error, localizedDescription];

            (code, domain, description)
        };

        Box::new(AppKitError {
            code: code,
            domain: str_from(domain).to_string(),
            description: str_from(description).to_string()
        })
    }
}

impl fmt::Display for AppKitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for AppKitError {}
