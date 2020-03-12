//! A wrapper for the underlying `NSToolbar`, which is safe to clone and pass around. We do this to
//! provide a uniform and expectable API.

use cocoa::base::{YES, NO};
use cocoa::foundation::{NSUInteger};

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::toolbar::types::{ToolbarDisplayMode, ToolbarSizeMode};

#[derive(Clone, Debug)]
pub struct ToolbarHandle(pub ShareId<Object>);

impl ToolbarHandle {
    /// Indicates whether the toolbar shows the separator between the toolbar and the main window
    /// contents.
    pub fn set_shows_baseline_separator(&self, shows: bool) {
        unsafe {
            let _: () = msg_send![&*self.0, setShowsBaselineSeparator:match shows {
                true => YES,
                false => NO
            }];
        }
    }

    /// Sets the toolbar's display mode.
    pub fn set_display_mode(&self, mode: ToolbarDisplayMode) {
        let mode: NSUInteger = mode.into();

        unsafe {
            let _: () = msg_send![&*self.0, setDisplayMode:mode];
        }
    }

    /// Sets the toolbar's size mode.
    pub fn set_size_mode(&self, mode: ToolbarSizeMode) {
        let mode: NSUInteger = mode.into();

        unsafe {
            let _: () = msg_send![&*self.0, setSizeMode:mode];
        }
    }

    /// Set whether the toolbar is visible or not.
    pub fn set_visible(&self, visibility: bool) {
        unsafe {
            let _: () = msg_send![&*self.0, setVisible:match visibility {
                true => YES,
                false => NO
            }];
        }
    }
}
