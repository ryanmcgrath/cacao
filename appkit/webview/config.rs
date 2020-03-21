//! A wrapper for `WKWebViewConfiguration`. It aims to (mostly) cover
//! the important pieces of configuring and updating a WebView configuration.

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, YES, NO, NSString};
use crate::webview::enums::InjectAt;

/// A wrapper for `WKWebViewConfiguration`. Holds (retains) pointers for the Objective-C runtime 
/// where everything lives.
pub struct WebViewConfig {
    pub objc: Id<Object>,
    pub handlers: Vec<String>
}

impl Default for WebViewConfig {
    /// Initializes a default `WebViewConfig`.
    fn default() -> Self {
        let config = unsafe {
            let config: id = msg_send![class!(WKWebViewConfiguration), new];
            Id::from_ptr(config)
        };

        WebViewConfig {
            objc: config,
            handlers: vec![]
        }
    }
}

impl WebViewConfig {
    /// Pushes the specified handler name onto the stack, queuing it for initialization with the
    /// `WKWebView`.
    pub fn add_handler(&mut self, name: &str) {
        self.handlers.push(name.to_string());
    }

    /// Adds the given user script to the underlying `WKWebView` user content controller.
    pub fn add_user_script(&mut self, script: &str, at: InjectAt, main_frame_only: bool) {
        let source = NSString::new(script);
        
        unsafe {
            let alloc: id = msg_send![class!(WKUserScript), alloc]; 
            let user_script: id = msg_send![alloc, initWithSource:source injectionTime:inject_at forMainFrameOnly:match main_frame_only {
                true => YES,
                false => NO
            }];

            let content_controller: id = msg_send![&*self.objc, userContentController];
            let _: () = msg_send![content_controller, addUserScript:user_script];
        }
    }

    /// Enables access to the underlying inspector view for `WKWebView`.
    pub fn enable_developer_extras(&mut self) {
        let key = NSString::new("developerExtrasEnabled");

        unsafe {
            let yes: id = msg_send![class!(NSNumber), numberWithBool:YES];
            let preferences: id = msg_send![&*self.objc, preferences];
            let _: () = msg_send![preferences, setValue:yes forKey:key];
        }
    }

    pub(crate) fn attach_handlers(&self, target: id) {

    }

    pub fn into_inner(self) -> id {
        &mut *self.objc
    }
}
