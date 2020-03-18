//! Implements Window controls for macOS, by wrapping the various necessary moving pieces
//! (`NSWindow`, `NSWindowController`, and `NSWindowDelegate`) into one trait that you can
//! implement.
//!
//! For example:
//!
//! ```
//! use appkit::prelude::{AppController, Window};
//! use window::MyWindow;
//!
//! pub struct MyApp(Window<MyWindow>);
//!
//! impl MyApp {
//!     pub fn new() -> Self {
//!         MyApp(Window::new(MyWindow {
//!             // Your things here
//!         }))
//!     }
//! }
//!
//! impl AppController for MyApp {
//!     fn did_load(&self) {
//!         self.0.show();
//!     }
//! }
//! ```
//!
//! This simulate class-based structures well enough - you just can't subclass. Now you can do the
//! following:
//!
//! ```
//! use appkit::prelude::{WindowController, WindowHandle};
//!
//! pub struct MyWindow;
//!
//! impl WindowController for MyWindow {
//!     fn did_load(&mut self, handle: WindowHandle) {}
//! }
//! ```

use std::rc::Rc;
use std::cell::RefCell;

use objc::{msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, nil};
use crate::constants::WINDOW_CONTROLLER_PTR;
use crate::layout::traits::Layout;
use crate::toolbar::{Toolbar, ToolbarController};

mod controller;
use controller::register_window_controller_class;

pub mod enums;
pub use enums::{WindowTitleVisibility};

pub mod config;
pub use config::{WindowConfig, WindowStyle};

pub mod handle;
pub use handle::WindowHandle;

pub mod traits;
pub use traits::WindowController;

/// A `Window` represents your way of interacting with an `NSWindow`. It wraps the various moving
/// pieces to enable you to focus on reacting to lifecycle methods and doing your thing.
#[derive(Clone, Debug)]
pub struct Window<T> {
    internal_callback_ptr: *const RefCell<T>,
    pub objc_controller: WindowHandle,
    pub controller: Rc<RefCell<T>>
}

impl<T> Window<T> where T: WindowController + 'static {
    /// Allocates and configures a `WindowController` in the Objective-C/Cocoa runtime that maps over
    /// to your supplied controller.
    ///
    /// Now, you may look at this and go "hey, the hell is going on here - why don't you make the
    /// `NSWindow` in `[NSWindowController loadWindow]`?
    ///
    /// This is a great question. It's because NSWindowController is... well, broken or buggy -
    /// pick a term, either works. It's optimized for loading from xib/nib files, and attempting to
    /// get loadWindow to fire properly is a pain in the rear (you're fighting a black box).
    ///
    /// This is why we do this work here, but for things subclassing `NSViewController`, we go with
    /// the route of implementing `loadView`.
    ///
    /// APPKIT!
    pub fn new(controller: T) -> Self {
        let window = controller.config().0;
        let controller = Rc::new(RefCell::new(controller));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&controller);
            Rc::into_raw(cloned)
        };

        let inner = unsafe {
            let window_controller_class = register_window_controller_class::<T>();
            let controller_alloc: id = msg_send![window_controller_class, alloc];
            let controller: id = msg_send![controller_alloc, initWithWindow:window];
            (&mut *controller).set_ivar(WINDOW_CONTROLLER_PTR, internal_callback_ptr as usize);
            
            let window: id = msg_send![controller, window];
            let _: () = msg_send![window, setDelegate:controller];
            
            ShareId::from_ptr(controller)
        };

        {
            let mut vc = controller.borrow_mut();
            (*vc).did_load(WindowHandle(Some(inner.clone())));
        }

        Window {
            internal_callback_ptr: internal_callback_ptr,
            objc_controller: WindowHandle(Some(inner)),
            controller: controller
        }
    }

    /// Sets the title for this window.
    pub fn set_title(&self, title: &str) {
        self.objc_controller.set_title(title.into());
    }

    /// Sets the toolbar for this window.
    pub fn set_toolbar<TC: ToolbarController + 'static>(&self, toolbar: &Toolbar<TC>) {
        self.objc_controller.set_toolbar(toolbar);
    }

    /// Sets the content view controller for the window.
    pub fn set_content_view_controller<VC: Layout + 'static>(&self, view_controller: &VC) {
        self.objc_controller.set_content_view_controller(view_controller);
    }

    /// Shows the window, running a configuration pass if necessary.
    pub fn show(&self) {
        self.objc_controller.show();
    }

    /// Closes the window.
    pub fn close(&self) {
        self.objc_controller.close();
    }
}

impl<T> Drop for Window<T> {
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
}
