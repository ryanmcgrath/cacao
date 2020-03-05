//! Implements a WebView, which wraps a number of different classes/delegates/controllers into one
//! useful interface. This encompasses...
//!
//! - `WKWebView`
//! - `WKUIDelegate`
//! - `WKScriptMessageHandler`
//! - `NSViewController`
//!
//! ...yeah.

use std::rc::Rc;
use std::cell::RefCell;

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::NSString;

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::view::ViewController;
use crate::webview::{WEBVIEW_VAR, WEBVIEW_CONFIG_VAR, WEBVIEW_CONTROLLER_PTR, WebViewController};
use crate::webview::controller::register_controller_class;
use crate::webview::config::{WebViewConfig, InjectAt};

#[derive(Default)]
pub struct WebViewInner {
    pub config: WebViewConfig,
    pub controller: Option<ShareId<Object>>
}

impl WebViewInner {
    pub fn configure<T: ViewController + WebViewController + 'static>(&mut self, controller: &T) {
        self.controller = Some(unsafe {
            let view_controller: id = msg_send![register_controller_class::<T>(), new];
            (&mut *view_controller).set_ivar(WEBVIEW_CONFIG_VAR, &*self.config.inner);
            (&mut *view_controller).set_ivar(WEBVIEW_CONTROLLER_PTR, controller as *const T as usize);
            ShareId::from_ptr(view_controller)
        });
    }

    // Builder pattern?
    //        let webview: id = msg_send![view_controller, view];
    //        let _: () = msg_send![webview, setUIDelegate:view_controller];

    pub fn add_user_script(&self, script: &str, inject_at: InjectAt, main_frame_only: bool) {
        unsafe {
            let source = NSString::alloc(nil).init_str(script);

            let cls = class!(WKUserScript);
            let alloc: id = msg_send![cls, alloc]; 
            let user_script: id = msg_send![alloc, initWithSource:source injectionTime:inject_at forMainFrameOnly:match main_frame_only {
                true => YES,
                false => NO
            }];

            let content_controller: id = msg_send![&*self.config.inner, userContentController];
            let _: () = msg_send![content_controller, addUserScript:user_script];
        }
    }

    pub fn add_handler(&self, name: &str) {
        if let Some(controller) = &self.controller {
            unsafe {
                let name = NSString::alloc(nil).init_str(name);
                let content_controller: id = msg_send![&*self.config.inner, userContentController];
                let _: () = msg_send![content_controller, addScriptMessageHandler:controller.clone() name:name]; 
            }
        }
    }

    pub fn load_url(&self, url: &str) {
        if let Some(controller) = &self.controller {
            unsafe {
                // This is weird, I know, but it has to be done due to a lifecycle "quirk" in AppKit.
                // In short: `loadView` isn't called unless the view is actually accessed, and you
                // could theoretically call this without having had it done. We use the `loadView`
                // method because we *want* the lazy loading aspect, but for this call to work we
                // need things to be done.
                //
                // We can't create the `WKWebView` before `loadView` as it copies
                // `WKWebViewConfiguration` on initialization, and we defer that for API reasons.
                let _view: id = msg_send![*controller, view];

                let url_string = NSString::alloc(nil).init_str(url);
                let u: id = msg_send![class!(NSURL), URLWithString:url_string];
                let request: id = msg_send![class!(NSURLRequest), requestWithURL:u];
                let webview: id = *controller.get_ivar(WEBVIEW_VAR);
                let _: () = msg_send![webview, loadRequest:request];
            }
        }
    }
}

#[derive(Default)]
pub struct WebView(Rc<RefCell<WebViewInner>>);

impl WebView {
    pub fn configure<T: ViewController + WebViewController + 'static>(&self, controller: &T) {
        {
            let mut webview = self.0.borrow_mut();
            webview.configure(controller);
        }

        controller.did_load();
    }

    pub fn get_handle(&self) -> Option<ShareId<Object>> {
        let webview = self.0.borrow();
        webview.controller.clone()
    }

    pub fn load_url(&self, url: &str) {
        let webview = self.0.borrow();
        webview.load_url(url);
    }
    
    pub fn add_user_script(&self, script: &str, inject_at: InjectAt, main_frame_only: bool) {
        let webview = self.0.borrow();
        webview.add_user_script(script, inject_at, main_frame_only);
    }

    pub fn add_handler(&self, name: &str) {
        let webview = self.0.borrow();
        webview.add_handler(name);
    }
}
