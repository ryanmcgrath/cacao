//! A wrapper for `WKWebViewConfiguration`. It aims to (mostly) cover
//! the important pieces of configuring and updating a WebView configuration.

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::NSString;

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::webview::process_pool::register_process_pool;

/// Whether a script should be injected at the start or end of the document load.
pub enum InjectAt {
    Start = 0,
    End = 1
}

/// A wrapper for `WKWebViewConfiguration`. Holds (retains) pointers for the Objective-C runtime 
/// where everything lives.
pub struct WebViewConfig {
    pub inner: Id<Object>
}

impl Default for WebViewConfig {
    fn default() -> Self {
        let inner = unsafe {
            let cls = class!(WKWebViewConfiguration);
            let inner: id = msg_send![cls, new];

            // For debug builds, we want to enable this as it allows the inspector to be used.
            if cfg!(debug_assertions) {
                let key = NSString::alloc(nil).init_str("developerExtrasEnabled");
                let yes: id = msg_send![class!(NSNumber), numberWithBool:YES];
                let preferences: id = msg_send![inner, preferences];
                let _: () = msg_send![preferences, setValue:yes forKey:key];
            }

            Id::from_ptr(inner)
        };

        WebViewConfig {
            inner: inner
        }
    }
}
