//! Wraps `NSUserDefaults`, providing an interface to store and query small amounts of data.
//!
//! It mirrors much of the API of the standard Rust `HashMap`, but uses `NSUserDefaults` as a
//! backing store.

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, NSString};

#[derive(Debug)]
pub struct UserDefaults(pub Id<Object>);

impl Default for UserDefaults {
    fn default() -> Self {
        UserDefaults::standard()
    }
}

impl UserDefaults {
    pub fn standard() -> Self {
        UserDefaults(unsafe {
            Id::from_ptr(msg_send![class!(NSUserDefaults), standardUserDefaults])
        })
    }

    pub fn new() -> Self {
        UserDefaults(unsafe {
            let alloc: id = msg_send![class!(NSUserDefaults), alloc];
            Id::from_ptr(msg_send![alloc, init])
        })
    }

    pub fn suite(named: &str) -> Self {
        let name = NSString::new(named);

        UserDefaults(unsafe {
            let alloc: id = msg_send![class!(NSUserDefaults), alloc];
            Id::from_ptr(msg_send![alloc, initWithSuiteName:name.into_inner()])
        })
    }

    /// Remove the default associated with the key. If the key doesn't exist, this is a noop.
    pub fn remove(&self, key: &str) {
        let key = NSString::new(key);

        unsafe {
            let _: () = msg_send![&*self.0, removeObjectForKey:key.into_inner()];
        }
    }
}
