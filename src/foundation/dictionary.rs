//! A wrapper for `NSDictionary`, which aims to make dealing with the class throughout this
//! framework a tad bit simpler.

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, nil, YES, NO, NSString};

/// A wrapper for `NSDictionary`. Behind the scenes we actually wrap `NSMutableDictionary`, and
/// rely on Rust doing the usual borrow-checking guards that it does so well.
#[derive(Debug)]
pub struct NSDictionary(pub Id<Object>);

impl Default for NSDictionary {
    fn default() -> Self {
        NSDictionary::new()
    }
}

impl NSDictionary {
    pub fn new() -> Self {
        NSDictionary(unsafe {
            Id::from_ptr(msg_send![class!(NSMutableDictionary), new])
        })
    }

    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
