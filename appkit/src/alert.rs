//! A wrapper for `NSAlert`. Currently doesn't cover everything possible for this class, as it was
//! built primarily for debugging uses. Feel free to extend via pull requests or something.

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

/// Represents an `NSAlert`. Has no information other than the retained pointer to the Objective C
/// side, so... don't bother inspecting this.
pub struct Alert {
    pub inner: Id<Object>
}

impl Alert {
    /// Creates a basic `NSAlert`, storing a pointer to it in the Objective C runtime.
    /// You can show this alert by calling `show()`.
    pub fn new(title: &str, message: &str) -> Self {
        Alert {
            inner: unsafe {
                let cls = class!(NSAlert);
                let alert: id = msg_send![cls, new];

                let title = NSString::alloc(nil).init_str(title);
                let _: () = msg_send![alert, setMessageText:title];

                let message = NSString::alloc(nil).init_str(message);
                let _: () = msg_send![alert, setInformativeText:message];

                let x = NSString::alloc(nil).init_str("OK");
                let _: () = msg_send![alert, addButtonWithTitle:x];

                Id::from_ptr(alert)
            }
        }
    }

    /// Shows this alert as a modal.
    pub fn show(&self) {
        unsafe {
           let _: () = msg_send![&*self.inner, runModal];
        }
    }
}
