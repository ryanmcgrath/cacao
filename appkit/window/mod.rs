//! Implements an `NSWindow` wrapper for MacOS, backed by Cocoa and associated widgets. This also handles looping back
//! lifecycle events, such as window resizing or close events.

use std::rc::Rc;
use std::cell::RefCell;

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, nil, YES, NO, NSString, NSUInteger, CGRect, CGSize};
use crate::layout::traits::Layout;
use crate::toolbar::{Toolbar, ToolbarController};
use crate::utils::Controller;

mod class;
use class::{register_window_class, register_window_class_with_delegate};

pub mod config;
pub use config::WindowConfig;

pub mod controller;
pub use controller::WindowController;

pub mod enums;

pub mod traits;
pub use traits::WindowDelegate;

pub(crate) static WINDOW_DELEGATE_PTR: &str = "rstWindowDelegate";

/// A `Window` represents your way of interacting with an `NSWindow`. It wraps the various moving
/// pieces to enable you to focus on reacting to lifecycle methods and doing your thing.
#[derive(Debug)]
pub struct Window<T = ()> {
    /// A pointer to the Objective-C `NSWindow`. Used in callback orchestration.
    pub(crate) internal_callback_ptr: Option<*const RefCell<T>>,

    /// Represents an `NSWindow` in the Objective-C runtime.
    pub objc: ShareId<Object>,

    /// A delegate for this window.
    pub delegate: Option<Rc<RefCell<T>>>
}

impl Default for Window {
    fn default() -> Self {
        Window::new(WindowConfig::default())
    }
}

impl Window {
    /// Constructs a new Window. 
    pub fn new(config: WindowConfig) -> Window {
        let objc = unsafe {
            let alloc: id = msg_send![register_window_class(), alloc];
            
            // Other types of backing (Retained/NonRetained) are archaic, dating back to the
            // NeXTSTEP era, and are outright deprecated... so we don't allow setting them.
            let buffered: NSUInteger = 2;
            let dimensions: CGRect = config.initial_dimensions.into();
            let window: id = msg_send![alloc, initWithContentRect:dimensions 
                styleMask:config.style 
                backing:buffered
                defer:match config.defer {
                    true => YES,
                    false => NO
                }
            ];

            let _: () = msg_send![window, autorelease];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync by releasing the window out from underneath of
            // us.
            let _: () = msg_send![window, setReleasedWhenClosed:NO];

            ShareId::from_ptr(window)
        };

        Window {
            internal_callback_ptr: None,
            objc: objc,
            delegate: None
        }
    }
}

impl<T> Window<T> {
    /// Handles setting the title on the underlying window. Allocates and passes an `NSString` over
    /// to the Objective C runtime.
    pub fn set_title(&self, title: &str) {
        unsafe {
            let title = NSString::new(title);
            let _: () = msg_send![&*self.objc, setTitle:title];
        }
    }

    /// Sets the title visibility for the underlying window.
    pub fn set_title_visibility(&self, visibility: usize) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitleVisibility:visibility];
        }
    }

    /// Used for configuring whether the window is movable via the background.
    pub fn set_movable_by_background(&self, movable: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setMovableByWindowBackground:match movable {
                true => YES,
                false => NO
            }];
        }
    }

    /// Used for setting whether this titlebar appears transparent.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitlebarAppearsTransparent:match transparent {
                true => YES,
                false => NO
            }];
        }
    }

    /// Used for setting this Window autosave name.
    pub fn set_autosave_name(&self, name: &str) {
        unsafe {
            let autosave = NSString::new(name);
            let _: () = msg_send![&*self.objc, setFrameAutosaveName:autosave]; 
        }
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setMinSize:size];
        }
    }

    /// Used for setting a toolbar on this window. 
    pub fn set_toolbar<TC: ToolbarController>(&self, toolbar: &Toolbar<TC>) {
        unsafe {
            let _: () = msg_send![&*self.objc, setToolbar:&*toolbar.objc_controller.0];
        }
    }

    /// Given a view, sets it as the content view for this window.
    pub fn set_content_view<L: Layout + 'static>(&self, view: &L) {
        let backing_node = view.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, setContentView:&*backing_node];
        }
    }

    /// Given a view, sets it as the content view controller for this window.
    pub fn set_content_view_controller<C: Controller + 'static>(&self, controller: &C) {
        let backing_node = controller.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, setContentViewController:&*backing_node];
        }
    }

    /// Shows the window.
    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyAndOrderFront:nil];
        }
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    pub fn close(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, close];
        }
    }
}

impl<T> Window<T> where T: WindowDelegate + 'static {
    /// Constructs a new Window. 
    pub fn with(config: WindowConfig, delegate: T) -> Self {
        let delegate = Rc::new(RefCell::new(delegate));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&delegate);
            Rc::into_raw(cloned)
        };

        let objc = unsafe {
            let alloc: id = msg_send![register_window_class_with_delegate::<T>(), alloc];
            
            // Other types of backing (Retained/NonRetained) are archaic, dating back to the
            // NeXTSTEP era, and are outright deprecated... so we don't allow setting them.
            let buffered: NSUInteger = 2;
            let dimensions: CGRect = config.initial_dimensions.into();
            let window: id = msg_send![alloc, initWithContentRect:dimensions 
                styleMask:config.style 
                backing:buffered
                defer:match config.defer {
                    true => YES,
                    false => NO
                }
            ];

            (&mut *window).set_ivar(WINDOW_DELEGATE_PTR, internal_callback_ptr as usize);

            let _: () = msg_send![window, autorelease];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync by releasing the window out from underneath of
            // us.
            let _: () = msg_send![window, setReleasedWhenClosed:NO];

            // We set the window to be its own delegate - this is cleaned up inside `Drop`.
            let _: () = msg_send![window, setDelegate:window];

            ShareId::from_ptr(window)
        };

        {
            let mut window_delegate = delegate.borrow_mut();
            (*window_delegate).did_load(Window {
                internal_callback_ptr: None,
                delegate: None,
                objc: objc.clone()
            });
        }

        Window {
            internal_callback_ptr: Some(internal_callback_ptr),
            objc: objc,
            delegate: Some(delegate)
        }
    }
}

/*impl<T> Drop for Window<T> {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    ///
    /// We also clean up our loopback pointer that we use for callbacks.
    fn drop(&mut self) {
        unsafe { 
            if let Some(objc_controller) = &self.objc_controller.0 {
                let window: id = msg_send![*objc_controller, window];
                let _: () = msg_send![window, setDelegate:nil];
            }

            let _ = Rc::from_raw(self.internal_callback_ptr);
        }
    }
}*/
