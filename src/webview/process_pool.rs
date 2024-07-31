//! Implements a shared `WKProcessPool`, so that multiple webviews (should they be needed) properly
//! share cookies and the like. It also, if you opt in to the feature flag, enables a download
//! delegate that's sadly a private API... in 2020.
//!
//! If you use that feature, there are no guarantees you'll be accepted into the App Store.

use block::Block;
use cocoa::foundation::{NSArray, NSInteger, NSPoint, NSRect, NSSize, NSString};
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::foundation::{id, nil};
use crate::webview::traits::WebViewController;

extern "C" fn download_delegate(this: &Object, _: Sel) -> id {
    println!("YO!");
    unsafe { NSString::alloc(nil).init_str("") }
}

pub fn register_process_pool() -> *const Object {
    load_or_register_class("WKProcessPool", "RSTWebViewProcessPool", |decl| unsafe {
        //decl.add_ivar::<id>(DOWNLOAD_DELEGATE_PTR);
        decl.add_method(sel!(_downloadDelegate), download_delegate as extern "C" fn(_, _) -> _);
    })
}
