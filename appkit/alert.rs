//! A wrapper for `NSAlert`. Currently doesn't cover everything possible for this class, as it was
//! built primarily for debugging uses. Feel free to extend via pull requests or something.

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSString};

/// Represents an `NSAlert`. Has no information other than the retained pointer to the Objective C
/// side, so... don't bother inspecting this.
pub struct Alert(Id<Object>);

impl Alert {
    /// Creates a basic `NSAlert`, storing a pointer to it in the Objective C runtime.
    /// You can show this alert by calling `show()`.
    pub fn new(title: &str, message: &str) -> Self {
        let title = NSString::new(title);
        let message = NSString::new(message);
        let x = NSString::new("OK");

        Alert(unsafe {
            let alert: id = msg_send![class!(NSAlert), new];
            let _: () = msg_send![alert, setMessageText:title];
            let _: () = msg_send![alert, setInformativeText:message];
            let _: () = msg_send![alert, addButtonWithTitle:x];
            Id::from_ptr(alert)
        })
    }

    /// Shows this alert as a modal.
    pub fn show(&self) {
        unsafe {
           let _: () = msg_send![&*self.0, runModal];
        }
    }
}
