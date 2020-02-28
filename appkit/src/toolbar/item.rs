//! Implements an NSToolbar wrapper, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use cocoa::base::{id, nil};
use cocoa::foundation::{NSSize, NSString};

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::button::Button;

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
pub struct ToolbarItem<'a> {
    pub identifier: &'a str,
    pub inner: Id<Object>,
    pub button: Option<Button>
}

impl<'a> ToolbarItem<'a> {
    /// Creates a new `NSWindow` instance, configures it appropriately (e.g, titlebar appearance),
    /// injects an `NSObject` delegate wrapper, and retains the necessary Objective-C runtime
    /// pointers.
    pub fn new(identifier: &'a str) -> Self {
        let inner = unsafe {
            let identifier = NSString::alloc(nil).init_str(identifier);
            let alloc: id = msg_send![class!(NSToolbarItem), alloc];
            let item: id = msg_send![alloc, initWithItemIdentifier:identifier];
            Id::from_ptr(item)
        };

        ToolbarItem {
            identifier: identifier,
            inner: inner,
            button: None
        }
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = NSString::alloc(nil).init_str(title);
            let _: () = msg_send![&*self.inner, setTitle:title];
        }
    }

    pub fn set_button(&mut self, button: Button) {
        button.set_bezel_style(11);

        unsafe {
            let _: () = msg_send![&*self.inner, setView:&*button.inner];
        }
        
        self.button = Some(button);
    }

    pub fn set_min_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = NSSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.inner, setMinSize:size];
        }
    }

    pub fn set_max_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = NSSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.inner, setMaxSize:size];
        }
    }
}
