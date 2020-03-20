//! A wrapper for NSButton. Currently the epitome of jank - if you're poking around here, expect
//! that this will change at some point.

use std::sync::Once;

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{nil, NSString};

/// A wrapper for `NSButton`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSButton` lives.
pub struct Button {
    pub inner: Id<Object>
}

impl Button {
    /// Creates a new `NSButton` instance, configures it appropriately,
    /// and retains the necessary Objective-C runtime pointer.
    pub fn new(text: &str) -> Self {
        let title = NSString::new(text);
        let inner = unsafe {
            Id::from_ptr(msg_send![register_class(), buttonWithTitle:title target:nil action:nil])
        };

        Button {
            inner: inner
        }
    }

    /// Sets the bezel style for this button.
    pub fn set_bezel_style(&self, bezel_style: i32) {
        unsafe {
            let _: () = msg_send![&*self.inner, setBezelStyle:bezel_style];
        }
    }
}

/// Registers an `NSButton` subclass, and configures it to hold some ivars for various things we need
/// to store.
fn register_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSButton);
        let decl = ClassDecl::new("RSTButton", superclass).unwrap(); 
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
