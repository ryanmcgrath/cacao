//! Hoists a basic NSView. In our current particular use case,
//! this is primarily used as the ContentView for a window. From there,
//! we configure an NSToolbar and WKWebview on top of them.

use std::sync::Once;
use std::ffi::c_void;

use block::Block;

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSArray, NSInteger};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::view::ViewController;
use crate::view::class::register_view_class;
use crate::webview::action::{NavigationAction, NavigationResponse};
use crate::webview::{WEBVIEW_VAR, WEBVIEW_CONFIG_VAR, WEBVIEW_CONTROLLER_PTR};
use crate::webview::traits::WebViewController;
use crate::utils::str_from;

/// Loads and configures ye old WKWebView/View for this controller.
extern fn load_view<T: ViewController + WebViewController>(this: &mut Object, _: Sel) {
    unsafe {
        let configuration: id = *this.get_ivar(WEBVIEW_CONFIG_VAR);

        // Technically private!
        #[cfg(feature = "enable-webview-downloading")]
        let process_pool: id = msg_send![configuration, processPool];
        #[cfg(feature = "enable-webview-downloading")]
        let _: () = msg_send![process_pool, _setDownloadDelegate:&*this];

        let zero = NSRect::new(NSPoint::new(0., 0.), NSSize::new(1000., 600.));
        let webview_alloc: id = msg_send![class!(WKWebView), alloc];
        let webview: id = msg_send![webview_alloc, initWithFrame:zero configuration:configuration];
        let _: () = msg_send![webview, setWantsLayer:YES];
        let _: () = msg_send![webview, setTranslatesAutoresizingMaskIntoConstraints:NO];
 
        // Provide an easy way to grab this later
        (*this).set_ivar(WEBVIEW_VAR, webview);
        
        // Clean this up to be safe, as WKWebView makes a copy and we don't need it anymore.
        (*this).set_ivar(WEBVIEW_CONFIG_VAR, nil);

        let _: () = msg_send![this, setView:webview]; 
    }
}

/// Used to connect delegates - doing this in `loadView` can be... bug-inducing.
extern fn view_did_load<T: ViewController + WebViewController>(this: &Object, _: Sel) {
    unsafe {
        let webview: id = *this.get_ivar(WEBVIEW_VAR);
        let _: () = msg_send![webview, setNavigationDelegate:&*this];
        let _: () = msg_send![webview, setUIDelegate:&*this];
    }
}

/// Called when an `alert()` from the underlying `WKWebView` is fired. Will call over to your
/// `WebViewController`, where you should handle the event.
extern fn alert<T: WebViewController + 'static>(_: &Object, _: Sel, _: id, s: id, _: id, complete: id) {
    let alert = str_from(s);
    println!("Alert: {}", alert);

    // @TODO: This is technically (I think?) a private method, and there's some other dance that
    // needs to be done here involving taking the pointer/invoke/casting... but this is fine for
    // now as it's being exposed purely for debugging.
    unsafe {
        let _: () = msg_send![complete, invoke];
    }

    /*unsafe {
        let ptr: usize = *this.get_ivar(WEBVIEW_CONTROLLER_PTR);
        let webview = ptr as *const T;
        (*webview).alert(alert);
    }*/

    /*let queue = dispatch::Queue::main();
    queue.exec_async(move || {
        let a = Alert::new("Subatomic", message);
        a.show();
    });*/
}

/// Fires when a message has been passed from the underlying `WKWebView`.
extern fn on_message<T: WebViewController + 'static>(this: &Object, _: Sel, _: id, script_message: id) {
    unsafe {
        let name = str_from(msg_send![script_message, name]);
        let body = str_from(msg_send![script_message, body]);

        let ptr: usize = *this.get_ivar(WEBVIEW_CONTROLLER_PTR);
        let webview = ptr as *const T;
        (*webview).on_message(name, body);
    }
}

/// Fires when deciding a navigation policy - i.e, should something be allowed or not.
extern fn decide_policy_for_action<T: WebViewController + 'static>(this: &Object, _: Sel, _: id, action: id, handler: usize) {
    let webview = unsafe {
        let ptr: usize = *this.get_ivar(WEBVIEW_CONTROLLER_PTR);
        let webview = ptr as *const T;
        &*webview
    };

    let action = NavigationAction::new(action);
    webview.policy_for_navigation_action(action, |policy| {
        // This is very sketch and should be heavily checked. :|
        unsafe {
            let handler = handler as *const Block<(NSInteger,), c_void>;
            (*handler).call((policy.into(),));
        }
    }); 
}

/// Fires when deciding a navigation policy - i.e, should something be allowed or not.
extern fn decide_policy_for_response<T: WebViewController + 'static>(this: &Object, _: Sel, _: id, response: id, handler: usize) {
    let webview = unsafe {
        let ptr: usize = *this.get_ivar(WEBVIEW_CONTROLLER_PTR);
        let webview = ptr as *const T;
        &*webview
    };

    let response = NavigationResponse::new(response);
    webview.policy_for_navigation_response(response, |policy| unsafe {
        let handler = handler as *const Block<(NSInteger,), c_void>;
        (*handler).call((policy.into(),));
    });
}

/// Fires when deciding a navigation policy - i.e, should something be allowed or not.
extern fn run_open_panel<T: WebViewController + 'static>(this: &Object, _: Sel, _: id, params: id, _: id, handler: usize) {
    let webview = unsafe {
        let ptr: usize = *this.get_ivar(WEBVIEW_CONTROLLER_PTR);
        let webview = ptr as *const T;
        &*webview
    };

    webview.run_open_panel(params.into(), move |urls| unsafe {
        let handler = handler as *const Block<(id,), c_void>;

        match urls {
            Some(u) => {
                let nsurls: Vec<id> = u.iter().map(|s| {
                    let s = NSString::alloc(nil).init_str(&s);
                    msg_send![class!(NSURL), URLWithString:s]
                }).collect();

                let array = NSArray::arrayWithObjects(nil, &nsurls);
                (*handler).call((array,));
            },

            None => { (*handler).call((nil,)); }
        }
    });
}

/// Called when a download has been initiated in the WebView, and when the navigation policy
/// response is upgraded to BecomeDownload. Only called when explicitly linked since it's a private
/// API.
#[cfg(feature = "enable-webview-downloading")]
extern fn handle_download<T: WebViewController + 'static>(this: &Object, _: Sel, download: id, suggested_filename: id, handler: usize) {
    let webview = unsafe {
        let ptr: usize = *this.get_ivar(WEBVIEW_CONTROLLER_PTR);
        let webview = ptr as *const T;
        &*webview
    };

    let handler = handler as *const Block<(objc::runtime::BOOL, id), c_void>; 
    let filename = str_from(suggested_filename);

    webview.run_save_panel(filename, move |can_overwrite, path| unsafe {
        if path.is_none() {
            let _: () = msg_send![download, cancel];
        }

        let path = NSString::alloc(nil).init_str(&path.unwrap());
        
        (*handler).call((match can_overwrite {
            true => YES,
            false => NO
        }, path));
    });
}


/// Registers an `NSViewController` that we effectively turn into a `WebViewController`. Acts as
/// both a subclass of `NSViewController` and a delegate of the held `WKWebView` (for the various
/// varieties of delegates needed there).
pub fn register_controller_class<
    T: ViewController + WebViewController + 'static,
>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSViewController").unwrap();
        let mut decl = ClassDecl::new("RSTWebViewController", superclass).unwrap();

        decl.add_ivar::<id>(WEBVIEW_CONFIG_VAR);
        decl.add_ivar::<id>(WEBVIEW_VAR);
        decl.add_ivar::<usize>(WEBVIEW_CONTROLLER_PTR);

        // NSViewController
        decl.add_method(sel!(loadView), load_view::<T> as extern fn(&mut Object, _));
        decl.add_method(sel!(viewDidLoad), view_did_load::<T> as extern fn(&Object, _));

        // WKNavigationDelegate
        decl.add_method(sel!(webView:decidePolicyForNavigationAction:decisionHandler:), decide_policy_for_action::<T> as extern fn(&Object, _, _, id, usize));
        decl.add_method(sel!(webView:decidePolicyForNavigationResponse:decisionHandler:), decide_policy_for_response::<T> as extern fn(&Object, _, _, id, usize));

        // WKScriptMessageHandler
        decl.add_method(sel!(userContentController:didReceiveScriptMessage:), on_message::<T> as extern fn(&Object, _, _, id));
 
        // WKUIDelegate
        decl.add_method(sel!(webView:runJavaScriptAlertPanelWithMessage:initiatedByFrame:completionHandler:), alert::<T> as extern fn(&Object, _, _, id, _, _));
        decl.add_method(sel!(webView:runOpenPanelWithParameters:initiatedByFrame:completionHandler:), run_open_panel::<T> as extern fn(&Object, _, _, id, _, usize));
        
        // WKDownloadDelegate is a private class on macOS that handles downloading (saving) files.
        // It's absurd that this is still private in 2020. This probably couldn't get into the app
        // store, so... screw it, fine for now.
        #[cfg(feature = "enable-webview-downloading")]
        decl.add_method(sel!(_download:decideDestinationWithSuggestedFilename:completionHandler:), handle_download::<T> as extern fn(&Object, _, id, id, usize));

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
