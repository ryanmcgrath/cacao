//! Implements an NSToolbar wrapper, which is one of those AppKit niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use core_graphics::geometry::CGSize;
use std::fmt;

use crate::id_shim::{Id, ShareId};
use objc::runtime::Object;
use objc::{class, msg_send, sel};

use crate::appkit::segmentedcontrol::SegmentedControl;
use crate::button::{BezelStyle, Button};
use crate::foundation::{id, NSString, NO, YES};
use crate::image::Image;
use crate::invoker::TargetActionHandler;

/// Wraps `NSToolbarItem`. Enables configuring things like size, view, and so on.
#[derive(Debug)]
pub struct ToolbarItem {
    pub identifier: String,
    pub objc: Id<Object>,
    pub button: Option<Button>,
    pub segmented_control: Option<SegmentedControl>,
    pub image: Option<Image>,
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
            let item: id = msg_send![alloc, initWithItemIdentifier: identifr];
            Id::from_ptr(item)
        };

        ToolbarItem {
            identifier,
            objc,
            button: None,
            segmented_control: None,
            image: None,
            handler: None
        }
    }

    pub(crate) fn wrap(item: id) -> Self {
        ToolbarItem {
            identifier: "".to_string(),
            objc: unsafe { Id::from_retained_ptr(item) },
            button: None,
            segmented_control: None,
            image: None,
            handler: None
        }
    }

    /// Sets the title for this item.
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = NSString::new(title);
            let _: () = msg_send![&*self.objc, setLabel:&*title];
        }
    }

    /// Sets and takes ownership of the button for this item.
    pub fn set_button(&mut self, button: Button) {
        button.set_bezel_style(BezelStyle::TexturedRounded);

        button.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*self.objc, setView: obj];
        });

        self.button = Some(button);
    }

    /// Sets and takes ownership of the segmented control for this item.
    pub fn set_segmented_control(&mut self, control: SegmentedControl) {
        control.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*self.objc, setView: obj];
        });

        self.segmented_control = Some(control);
    }

    /// Sets and takes ownership of the image for this toolbar item.
    pub fn set_image(&mut self, image: Image) {
        unsafe {
            let _: () = msg_send![&*self.objc, setImage:&*image.0];
        }

        self.image = Some(image);
    }

    /// Sets the minimum size for this button.
    pub fn set_min_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setMinSize: size];
        }
    }

    /// Sets the maximum size for this button.
    pub fn set_max_size(&mut self, width: f64, height: f64) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setMaxSize: size];
        }
    }

    /// Sets an action on this item.
    pub fn set_action<F: Fn(*const Object) + Send + Sync + 'static>(&mut self, action: F) {
        let handler = TargetActionHandler::new(&*self.objc, action);
        self.handler = Some(handler);
    }

    pub fn set_bordered(&self, bordered: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setBordered:match bordered {
                true => YES,
                false => NO
            }];
        }
    }
}
