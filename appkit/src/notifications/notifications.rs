//! Acts as a (currently dumb) wrapper for `UNMutableNotificationContent`, which is what you mostly
//! need to pass to the notification center for things to work.

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

/// A wrapper for `UNMutableNotificationContent`. Retains the pointer from the Objective C side,
/// and is ultimately dropped upon sending.
pub struct Notification {
    pub inner: Id<Object>
}

impl Notification {
    /// Constructs a new `Notification`. This allocates `NSString`'s, as it has to do so for the
    /// Objective C runtime - be aware if you're slaming this (you shouldn't be slamming this).
    pub fn new(title: &str, body: &str) -> Self {
        Notification {
            inner: unsafe {
                let cls = class!(UNMutableNotificationContent);
                let content: id = msg_send![cls, new];

                let title = NSString::alloc(nil).init_str(title);
                let _: () = msg_send![content, setTitle:title];

                let body = NSString::alloc(nil).init_str(body);
                let _: () = msg_send![content, setBody:body];

                Id::from_ptr(content)
            }
        }
    }
}
