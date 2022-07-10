//! A lightweight wrapper over some networking components, like `NSURLRequest` and co.
//!
/// At the moment, this is mostly used for inspection of objects returned from system
/// calls, as `NSURL` is pervasive in some filesystem references. Over time this may grow to
/// include a proper networking stack, but the expectation for v0.1 is that most apps will want to
/// use their standard Rust networking libraries (however... odd... the async story may be).

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, NSString};

/// A wrapper around `NSURLRequest`.
#[derive(Debug)]
pub struct URLRequest(ShareId<Object>);

impl URLRequest {
    /// Wraps and retains an `NSURLRequest`.
    pub fn with(request: id) -> Self {
        URLRequest(unsafe {
            ShareId::from_ptr(request)
        })
    }

    /// Returns the underlying request URL as an owned `String`.
    pub fn absolute_url(&self) -> String {
        NSString::from_retained(unsafe {
            let url: id = msg_send![&*self.0, URL];
            msg_send![url, absoluteString]
        }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use objc::{class, msg_send, sel, sel_impl};

    use crate::foundation::{id, NSString};
    use crate::networking::URLRequest;

    #[test]
    fn test_urlrequest() {
        let endpoint = "https://rymc.io/";

        let url = unsafe {
            let url = NSString::new(endpoint);
            let url: id = msg_send![class!(NSURL), URLWithString:&*url];
            URLRequest::with(msg_send![class!(NSURLRequest), requestWithURL:url])
        };

        assert_eq!(&url.absolute_url(), endpoint);
    }
}
