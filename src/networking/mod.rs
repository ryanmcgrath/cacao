//! A lightweight wrapper over some networking components, like `NSURLRequest` and co.
//! This is currently not meant to be exhaustive.

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, NSString};

#[derive(Debug)]
pub struct URLRequest {
    pub inner: Id<Object>
}

impl URLRequest {
    pub fn with(inner: id) -> Self {
        URLRequest {
            inner: unsafe { Id::from_ptr(inner) }
        }
    }

    pub fn url(&self) -> String {
        NSString::wrap(unsafe {
            let url: id = msg_send![&*self.inner, URL];
            msg_send![url, absoluteString]
        }).to_string()
    }
}
