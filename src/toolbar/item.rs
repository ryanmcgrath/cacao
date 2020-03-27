//! Implements an NSToolbar wrapper, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use core_graphics::geometry::CGSize;

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSString};
use crate::button::Button;

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
pub struct ToolbarItem {
    pub identifier: String,
    pub inner: Id<Object>,
    pub button: Option<Button>
}

impl ToolbarItem {
    /// Creates a new `NSWindow` instance, configures it appropriately (e.g, titlebar appearance),
    /// injects an `NSObject` delegate wrapper, and retains the necessary Objective-C runtime
    /// pointers.
    pub fn new<S: Into<String>>(identifier: S) -> Self {
        let identifier = identifier.into();

        let inner = unsafe {
            let identifr = NSString::new(&identifier);
            let alloc: id = msg_send![class!(NSToolbarItem), alloc];
            let item: id = msg_send![alloc, initWithItemIdentifier:identifr];
            Id::from_ptr(item)
        };

        ToolbarItem {
            identifier: identifier,
            inner: inner,
            button: None
        }
    }

    /// Sets the title for this item.
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = NSString::new(title);
            let _: () = msg_send![&*self.inner, setTitle:title];
        }
    }

    /// Sets and takes ownership of the button for this item.
    pub fn set_button(&mut self, button: Button) {
        button.set_bezel_style(11);

        unsafe {
            let _: () = msg_send![&*self.inner, setView:&*button.inner];
        }
        
        self.button = Some(button);
    }

    /// Sets the minimum size for this button.
    pub fn set_min_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.inner, setMinSize:size];
        }
    }

    /// Sets the maximum size for this button.
    pub fn set_max_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.inner, setMaxSize:size];
        }
    }
}
