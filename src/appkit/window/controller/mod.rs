//! A `WindowController` is useful for handling certain document patterns on macOS.
//!
//! (iOS has no equivalent, as `UIWindowController` is private there).
//!
//! In particular, this is useful for certain situations regarding document handling
//! (which this framework does not yet cover, but may eventually). Note that this control can only
//! be created by providing a `WindowDelegate`.
//!
//! >If your application only uses a single `Window`, you may not even need this - just set the
//! autosave name on your `Window` to get the benefit of cached window location across restarts.
//!
//! # How to use
//!
//! ```rust,no_run
//! use cacao::appkit::app::AppDelegate;
//! use cacao::appkit::window::{WindowController, WindowDelegate};
//!
//! #[derive(Default)]
//! struct MyWindow;
//!
//! impl WindowDelegate for MyWindow {
//!     // Your implementation here...
//! }
//!
//! struct MyApp {
//!     pub window: WindowController<MyWindow>
//! }
//! ```

use std::fmt;

use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, nil};
use crate::utils::Controller;
use crate::appkit::window::{Window, WindowConfig, WindowDelegate, WINDOW_DELEGATE_PTR};

mod class;
use class::register_window_controller_class;

/// A `WindowController` wraps your `WindowDelegate` into an underlying `Window`, and 
/// provides some extra lifecycle methods.
pub struct WindowController<T> {
    /// A handler to the underlying `NSWindowController`.
    pub objc: Id<Object>,

    /// The underlying `Window` that this controller wraps.
    pub window: Window<T>
}

impl<T> WindowController<T> where T: WindowDelegate + 'static {
    /// Allocates and configures an `NSWindowController` in the Objective-C/Cocoa runtime that maps over
    /// to your supplied delegate.
    pub fn with(config: WindowConfig, delegate: T) -> Self {
        let mut window = Window::with(config, delegate);

        let objc = unsafe {
            let window_controller_class = register_window_controller_class::<T>();
            let controller_alloc: id = msg_send![window_controller_class, alloc];
            let controller: id = msg_send![controller_alloc, initWithWindow:&*window.objc];

            if let Some(delegate) = &window.delegate {
                let ptr: *const T = &**delegate;
                (&mut *controller).set_ivar(WINDOW_DELEGATE_PTR, ptr as usize);
            }

            Id::from_ptr(controller)
        };

        if let Some(delegate) = &mut window.delegate {
            (*delegate).did_load(Window {
                delegate: None,
                objc: window.objc.clone()
            });
        }

        WindowController { objc, window }
    }

    /// Given a view, sets it as the content view controller for this window.
    pub fn set_content_view_controller<C: Controller + 'static>(&self, controller: &C) {
        let backing_node = controller.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, setContentViewController:&*backing_node];
        }
    }

    /// Shows the window, running a configuration pass if necessary.
    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, showWindow:nil];
        }
    }

    /// Closes the window.
    pub fn close(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, close];
        }
    }
}

impl<T> fmt::Debug for WindowController<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WindowController")
            .field("objc", &self.objc)
            .finish()
    }
}
