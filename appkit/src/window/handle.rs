//! Implements `WindowHandle`, which wraps a lower-level `NSWindowController` and handles method
//! shuffling to call through to the window it holds.
//!
//! We use `NSWindowController` as it has lifecycle methods that are useful, in addition to the
//! standard `NSWindowDelegate` methods.

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, nil, YES, NO, CGSize, NSString};
use crate::layout::traits::Layout;
use crate::toolbar::{Toolbar, ToolbarController};

#[derive(Debug, Default, Clone)]
pub struct WindowHandle(pub Option<ShareId<Object>>);

impl WindowHandle {
    /// Handles setting the title on the underlying window. Allocates and passes an `NSString` over
    /// to the Objective C runtime.
    pub fn set_title(&self, title: &str) {
        if let Some(controller) = &self.0 {
            unsafe {
                let title = NSString::new(title);
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setTitle:title];
            }
        }
    }

    /// Sets the title visibility for the underlying window.
    pub fn set_title_visibility(&self, visibility: usize) {
        if let Some(controller) = &self.0 {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setTitleVisibility:visibility];
            }
        }
    }

    /// Used for configuring whether the window is movable via the background.
    pub fn set_movable_by_background(&self, movable: bool) {
        if let Some(controller) = &self.0 {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setMovableByWindowBackground:match movable {
                    true => YES,
                    false => NO
                }];
            }
        }
    }

    /// Used for setting whether this titlebar appears transparent.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        if let Some(controller) = &self.0 {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setTitlebarAppearsTransparent:match transparent {
                    true => YES,
                    false => NO
                }];
            }
        }
    }

    /// Used for setting this Window autosave name.
    pub fn set_autosave_name(&mut self, name: &str) {
         if let Some(controller) = &self.0 {
            unsafe {
                let window: id = msg_send![*controller, window];
                let autosave = NSString::new(name);
                let _: () = msg_send![window, setFrameAutosaveName:autosave]; 
            }
        }       
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_content_size<F: Into<f64>>(&self, width: F, height: F) {
        if let Some(controller) = &self.0 {
            unsafe {
                let size = CGSize::new(width.into(), height.into());
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setMinSize:size];
            }
        }
    }

    /// Used for setting a toolbar on this window. 
    pub fn set_toolbar<TC: ToolbarController>(&self, toolbar: &Toolbar<TC>) {
        if let Some(controller) = &self.0 {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setToolbar:&*toolbar.objc_controller.0];
            }
        }
    }

    /// Used for setting the content view controller for this window.
    pub fn set_content_view_controller<T: Layout + 'static>(&self, view_controller: &T) {
        if let Some(controller) = &self.0 {
            unsafe {
                if let Some(vc) = view_controller.get_backing_node() {
                    let _: () = msg_send![*controller, setContentViewController:&*vc];
                }
            }
        }
    }

    /// On macOS, calling `show()` is equivalent to calling `makeKeyAndOrderFront`. This is the
    /// most common use case, hence why this method was chosen - if you want or need something
    /// else, feel free to open an issue to discuss.
    ///
    /// You should never be calling this yourself, mind you - Alchemy core handles this for you.
    pub fn show(&self) {
        if let Some(controller) = &self.0 {
            unsafe {
                let _: () = msg_send![*controller, showWindow:nil];
            }
        }
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    pub fn close(&self) {
        if let Some(controller) = &self.0 {
            unsafe {
                let _: () = msg_send![*controller, close];
            }
        }
    }
}
