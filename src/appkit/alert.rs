//! A wrapper for `NSAlert`.
//!
//! This is housed inside `appkit` as it's a useful tool for a few cases, but it doesn't match the
//! iOS API, so we make no guarantees about it being a universal control. In general this also
//! doesn't produce an amazing user experience, and you may want to shy away from using it.
//!
//! If you want to show a complex view in an alert-esque fashion, you may consider looking at
//! `Sheet`.
//!
//! ```rust,no_run
//! use cacao::appkit::{App, AppDelegate, Alert};
//!
//! #[derive(Default)]
//! struct ExampleApp;
//!
//! impl AppDelegate for ExampleApp {
//!     fn did_finish_launching(&self) {
//!
//!     }
//! }
//!
//! fn main() {
//!     App::new("com.alert.example", ExampleApp::default()).run()
//! }
//! ```

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, NSString};

/// Represents an `NSAlert`. Has no information other than the retained pointer to the Objective C
/// side, so... don't bother inspecting this.
#[derive(Debug)]
pub struct Alert(Id<Object>);

impl Alert {
    /// Creates a basic `NSAlert`, storing a pointer to it in the Objective C runtime.
    /// You can show this alert by calling `show()`.
    pub fn new(title: &str, message: &str) -> Self {
        let title = NSString::new(title);
        let message = NSString::new(message);
        let ok = NSString::new("OK");

        Alert(unsafe {
            let alert: id = msg_send![class!(NSAlert), new];
            let _: () = msg_send![alert, setMessageText: title];
            let _: () = msg_send![alert, setInformativeText: message];
            let _: () = msg_send![alert, addButtonWithTitle: ok];
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
