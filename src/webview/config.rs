//! A wrapper for `WKWebViewConfiguration`. It aims to (mostly) cover
//! the important pieces of configuring and updating a WebView configuration.

use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, NSInteger, NSString, NO, YES};
use crate::webview::enums::InjectAt;

/// A wrapper for `WKWebViewConfiguration`. Holds (retains) pointers for the Objective-C runtime
/// where everything lives.
#[derive(Debug)]
pub struct WebViewConfig {
    pub objc: Id<Object, Owned>,
    pub handlers: Vec<String>,
    pub protocols: Vec<String>
}

impl Default for WebViewConfig {
    /// Initializes a default `WebViewConfig`.
    fn default() -> Self {
        let config = unsafe { msg_send_id![class!(WKWebViewConfiguration), new] };

        WebViewConfig {
            objc: config,
            handlers: vec![],
            protocols: vec![]
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
        let at: NSInteger = at.into();

        unsafe {
            let alloc: id = msg_send![class!(WKUserScript), alloc];
            let user_script: id = msg_send![
                alloc,
                initWithSource: &*source,
                injectionTime: at,
                forMainFrameOnly: match main_frame_only {
                    true => YES,
                    false => NO
                },
            ];

            let content_controller: id = msg_send![&*self.objc, userContentController];
            let _: () = msg_send![content_controller, addUserScript: user_script];
        }
    }

    /// Register the given protocol to the underlying `WKWebView`.
    /// Example; protocol_name: `demo` will allow request to `demo://`
    pub fn add_custom_protocol(&mut self, protocol_name: &str) {
        self.protocols.push(protocol_name.to_string());
    }

    /// Enables access to the underlying inspector view for `WKWebView`.
    pub fn enable_developer_extras(&mut self) {
        let key = NSString::new("developerExtrasEnabled");

        unsafe {
            let yes: id = msg_send![class!(NSNumber), numberWithBool: YES];
            let preferences: id = msg_send![&*self.objc, preferences];
            let _: () = msg_send![preferences, setValue: yes, forKey: &*key];
        }
    }
}
