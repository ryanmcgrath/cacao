//! Acts as a (currently dumb) wrapper for `UNMutableNotificationContent`, which is what you mostly
//! need to pass to the notification center for things to work.

use crate::id_shim::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel};

use crate::foundation::{id, NSString};

/// A wrapper for `UNMutableNotificationContent`. Retains the pointer from the Objective C side,
/// and is ultimately dropped upon sending.
#[derive(Debug)]
pub struct Notification(pub Id<Object>);

impl Notification {
    /// Constructs a new `Notification`. This allocates `NSString`'s, as it has to do so for the
    /// Objective C runtime - be aware if you're slaming this (you shouldn't be slamming this).
    pub fn new(title: &str, body: &str) -> Self {
        let title = NSString::new(title);
        let body = NSString::new(body);

        Notification(unsafe {
            let content: id = msg_send![class!(UNMutableNotificationContent), new];
            let _: () = msg_send![content, setTitle: title];
            let _: () = msg_send![content, setBody: body];
            Id::from_ptr(content)
        })
    }
}
