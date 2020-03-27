//! WebViewController is a combination of various delegates and helpers for an underlying
//! `WKWebView`. It allows you to do things such as handle opening a file (for uploads or
//! in-browser-processing), handling navigation actions or JS message callbacks, and so on.

use crate::webview::WebView;
use crate::webview::actions::{NavigationAction, NavigationResponse, OpenPanelParameters};
use crate::webview::enums::{NavigationPolicy, NavigationResponsePolicy};

/// You can implement this on structs to handle callbacks from the underlying `WKWebView`.
pub trait WebViewDelegate {   
    /// Called when the View is ready to work with. You're passed a `ViewHandle` - this is safe to
    /// store and use repeatedly, but it's not thread safe - any UI calls must be made from the
    /// main thread!
    fn did_load(&mut self, _webview: WebView) {}

    /// Called when this is about to be added to the view heirarchy.
    fn will_appear(&self) {}

    /// Called after this has been added to the view heirarchy.
    fn did_appear(&self) {}

    /// Called when this is about to be removed from the view heirarchy.
    fn will_disappear(&self) {}

    /// Called when this has been removed from the view heirarchy.
    fn did_disappear(&self) {}

    /// Called when a JS message is passed by the browser process. For instance, if you added
    /// `notify` as a callback, and in the browser you called
    /// `webkit.messageHandlers.notify.postMessage({...})` it would wind up here, with `name` being
    /// `notify` and `body` being your arguments.
    ///
    /// Note that at the moment, you really should handle bridging JSON/stringification yourself.
    fn on_message(&self, _name: &str, _body: &str) {}

    /// Given a callback handler, you can decide what policy should be taken for a given browser
    /// action. By default, this is `NavigationPolicy::Allow`.
    fn policy_for_navigation_action<F: Fn(NavigationPolicy)>(&self, _action: NavigationAction, handler: F) {
        handler(NavigationPolicy::Allow);
    }

    /// Given a callback handler, you can decide what policy should be taken for a given browser
    /// response. By default, this is `NavigationResponsePolicy::Allow`.
    fn policy_for_navigation_response<F: Fn(NavigationResponsePolicy)>(&self, _response: NavigationResponse, handler: F) {
        handler(NavigationResponsePolicy::Allow);
    }

    /// Given a callback handler and some open panel parameters (e.g, if the user is clicking an
    /// upload field that pre-specifies supported options), you should create a `FileSelectPanel`
    /// and thread the callbacks accordingly.
    fn run_open_panel<F: Fn(Option<Vec<String>>) + 'static>(&self, _parameters: OpenPanelParameters, handler: F) {
        handler(None);
    }

    /// Given a callback handler and a suggested filename, you should create a `FileSavePanel`
    /// and thread the callbacks accordingly.
    ///
    /// Note that this specific callback is only
    /// automatically fired if you're linking in to the `webview_downloading` feature, which
    /// is not guaranteed to be App Store compatible. If you want a version that can go in the App
    /// Store, you'll likely need to write some JS in the webview to handle triggering
    /// downloading/saving. This is due to Apple not allowing the private methods on `WKWebView` to
    /// be open, which... well, complain to them, not me. :)
    fn run_save_panel<F: Fn(bool, Option<String>) + 'static>(&self, _suggested_filename: &str, handler: F) {
        handler(false, None);
    }
}
