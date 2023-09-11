//! Acts as a (currently dumb) wrapper for `UNMutableNotificationContent`, which is what you mostly
//! need to pass to the notification center for things to work.

use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, NSString};

/// A wrapper for `UNMutableNotificationContent`. Retains the pointer from the Objective C side,
/// and is ultimately dropped upon sending.
#[derive(Debug)]
pub struct Notification(pub Id<Object, Owned>);

impl Notification {
    /// Constructs a new `Notification`. This allocates `NSString`'s, as it has to do so for the
    /// Objective C runtime - be aware if you're slaming this (you shouldn't be slamming this).
    pub fn new(title: &str, body: &str) -> Self {
        let title = NSString::new(title);
        let body = NSString::new(body);

        Notification(unsafe {
            let mut content = msg_send_id![class!(UNMutableNotificationContent), new];
            let _: () = msg_send![&mut content, setTitle: &*title];
            let _: () = msg_send![&mut content, setBody: &*body];
            content
        })
    }
}
