//! A wrapper for `NSNotificationCenter`.
//!
//! With this, you can:
//!
//! - Register for notifications, both from the system or posted from your code
//! - Post your own notifications
//! - Clean up and remove your handlers
//!
//! Note that in some cases (e.g, looping) this will be much slower than if you have a handle and
//! can call through to your desired path directly. This control is provided due to the need for
//! integrating with certain aspects of the underlying Cocoa/Foundation/Kit frameworks.
//!
//! ## Example

//use std::sync::Mutex;
//use std::collections::HashMap;

//use lazy_static::lazy_static;
//use objc::{class, msg_send, msg_send_id, sel};
//use objc::runtime::Object;
//use objc::rc::{Id, Shared};

mod name;
pub use name::NotificationName;

mod traits;
pub use traits::Dispatcher;

/*lazy_static! {
    pub static ref DefaultNotificationCenter: NotificationCenter = {
        NotificationCenter {
            objc: unsafe {
                msg_send_id![class!(NSNotificationCenter), defaultCenter]
            },

            subscribers: Mutex::new(HashMap::new())
        }
    };
}*/

// Wraps a reference to an `NSNotificationCenter` instance. Currently this only supports the
// default center; in the future it should aim to support custom variants.
//#[derive(Debug)]
//pub struct NotificationCenter {
//    pub objc: Id<Object, Shared>,
//pub subscribers: Mutex<HashMap<String, Vec<Dispatcher>>>
//}

/*impl Default for NotificationCenter {
    /// Returns a wrapper over `[NSNotificationCenter defaultCenter]`. From here you can handle
    /// observing, removing, and posting notifications.
    fn default() -> Self {
        NotificationCenter {
            objc: unsafe {
                msg_send_id![class!(NSNotificationCenter), defaultCenter]
            }
        }
    }
}*/

/*impl NotificationCenter {
    pub fn observe<T: Dispatcher>(&self, name: &str, handler: &T) {

    }

    pub fn remove<T: Dispatcher>(&self, name: &str, handler: &T) {

    }

    pub fn post(&self, name: &str) {

    }
}*/
