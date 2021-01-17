//! Implements an NSToolbar wrapper, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use std::fmt;
use core_graphics::geometry::CGSize;

use objc_id::{Id, ShareId};
use objc::runtime::{Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSString};
use crate::invoker::TargetActionHandler;
use crate::button::Button;

/// Wraps `NSToolbarItem`. Enables configuring things like size, view, and so on.
#[derive(Debug)]
pub struct ToolbarItem {
    pub identifier: String,
    pub objc: Id<Object>,
    pub button: Option<Button>,
    handler: Option<TargetActionHandler>
}

impl ToolbarItem {
    /// Creates and returns a new `ToolbarItem`, ensuring the underlying `NSToolbarItem` is
    /// properly initialized.
    pub fn new<S: Into<String>>(identifier: S) -> Self {
        let identifier = identifier.into();

        let objc = unsafe {
            let identifr = NSString::new(&identifier);
            let alloc: id = msg_send![class!(NSToolbarItem), alloc];
            let item: id = msg_send![alloc, initWithItemIdentifier:identifr];
            Id::from_ptr(item)
        };

        ToolbarItem {
            identifier: identifier,
            objc: objc,
            button: None,
            handler: None
        }
    }

    /// Sets the title for this item.
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = NSString::new(title).into_inner();
            let _: () = msg_send![&*self.objc, setLabel:&*title];
            let _: () = msg_send![&*self.objc, setTitle:&*title];
        }
    }

    /// Sets and takes ownership of the button for this item.
    pub fn set_button(&mut self, button: Button) {
        button.set_bezel_style(11);

        unsafe {
            let _: () = msg_send![&*self.objc, setView:&*button.objc];
        }
        
        self.button = Some(button);
    }

    /// Sets the minimum size for this button.
    pub fn set_min_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setMinSize:size];
        }
    }

    /// Sets the maximum size for this button.
    pub fn set_max_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setMaxSize:size];
        }
    }

    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        let handler = TargetActionHandler::new(&*self.objc, action);
        self.handler = Some(handler);
    }
}
