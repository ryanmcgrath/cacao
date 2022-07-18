//! Implements a shared `WKProcessPool`, so that multiple webviews (should they be needed) properly
//! share cookies and the like. It also, if you opt in to the feature flag, enables a download
//! delegate that's sadly a private API... in 2020.
//!
//! If you use that feature, there are no guarantees you'll be accepted into the App Store.

use std::sync::Once;
use std::ffi::c_void;

use block::Block;

use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSArray, NSInteger};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO};
use crate::webview::traits::WebViewController;

extern fn download_delegate(this: &Object, _: Sel) -> id {
    println!("YO!");
    unsafe {
    NSString::alloc(nil).init_str("")
    }
}

pub fn register_process_pool() -> *const Object {
    static mut PROCESS_POOL: *const Object = 0 as *const Object;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("WKProcessPool").unwrap();
        let mut decl = ClassDecl::new("RSTWebViewProcessPool", superclass).unwrap();

        //decl.add_ivar::<id>(DOWNLOAD_DELEGATE_PTR);
        decl.add_method(sel!(_downloadDelegate), download_delegate as extern fn(&Object, _) -> id);

        //PROCESS_POOL = decl.register();
        PROCESS_POOL = msg_send![decl.register(), new];
    });

    unsafe { PROCESS_POOL }
}
