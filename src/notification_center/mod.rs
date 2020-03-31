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

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

mod traits;
pub use traits::Dispatcher;

/// Wraps a reference to an `NSNotificationCenter` instance. Currently this only supports the
/// default center; in the future it should aim to support custom variants.
#[derive(Debug)]
pub struct NotificationCenter(pub ShareId<Object>);

impl Default for NotificationCenter {
    /// Returns a wrapper over `[NSNotificationCenter defaultCenter]`. From here you can handle
    /// observing, removing, and posting notifications.
    fn default() -> Self {
        NotificationCenter(unsafe {
            ShareId::from_ptr(msg_send![class!(NSNotificationCenter), defaultCenter])
        })
    }
}

impl NotificationCenter {
    pub fn observe<T: Dispatcher>(&self, name: &str, handler: &T) {

    }

    pub fn remove<T: Dispatcher>(&self, name: &str, handler: &T) {

    }

    pub fn post(&self, name: &str) {

    }
}
