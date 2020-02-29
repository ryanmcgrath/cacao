//! A lightweight wrapper over some networking components, like `NSURLRequest` and co.
//! This is currently not meant to be exhaustive.

use cocoa::base::id;
use objc_id::Id;

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;

use crate::utils::str_from;

pub struct URLRequest {
    pub inner: Id<Object>
}

impl URLRequest {
    pub fn with(inner: id) -> Self {
        URLRequest {
            inner: unsafe { Id::from_ptr(inner) }
        }
    }

    pub fn url(&self) -> &'static str {
        unsafe {
            let url: id = msg_send![&*self.inner, URL];
            let path: id = msg_send![url, absoluteString];
            str_from(path)
        }
    }
}
