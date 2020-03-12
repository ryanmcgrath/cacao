//! Implements an NSToolbar, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use std::cell::RefCell;
use std::rc::Rc;

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

use objc_id::ShareId;
use objc::{msg_send, sel, sel_impl};

use crate::constants::TOOLBAR_PTR;
use crate::toolbar::class::register_toolbar_class;
use crate::toolbar::handle::ToolbarHandle;
use crate::toolbar::traits::ToolbarController;
use crate::toolbar::types::{ToolbarDisplayMode, ToolbarSizeMode};

/// A wrapper for `NSToolbar`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSToolbar` and associated delegate live.
pub struct Toolbar<T> {
    /// A pointer that we "forget" until dropping this struct. This allows us to keep the retain
    /// count of things appropriate until the Toolbar is done.
    internal_callback_ptr: *const RefCell<T>,

    /// An internal identifier used by the toolbar. We cache it here in case users want it.
    pub identifier: String,

    /// The Objective-C runtime controller (the toolbar, really - it does double duty).
    pub objc_controller: ToolbarHandle,

    /// The user supplied controller.
    pub controller: Rc<RefCell<T>>
}

impl<T> Toolbar<T> where T: ToolbarController + 'static {
    /// Creates a new `NSToolbar` instance, configures it appropriately, injects an `NSObject`
    /// delegate wrapper, and retains the necessary Objective-C runtime pointers.
    pub fn new<S: Into<String>>(identifier: S, controller: T) -> Self {
        let identifier = identifier.into();
        let controller = Rc::new(RefCell::new(controller));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&controller);
            Rc::into_raw(cloned)
        };

        let objc_controller = unsafe {
            let delegate_class = register_toolbar_class::<T>();
            let identifier = NSString::alloc(nil).init_str(&identifier);
            let alloc: id = msg_send![delegate_class, alloc];
            let toolbar: id = msg_send![alloc, initWithIdentifier:identifier];

            (&mut *toolbar).set_ivar(TOOLBAR_PTR, internal_callback_ptr as usize);
            let _: () = msg_send![toolbar, setDelegate:toolbar];

            ShareId::from_ptr(toolbar)
        };

        {
            let mut c = controller.borrow_mut();
            (*c).did_load(ToolbarHandle(objc_controller.clone()));
        }

        Toolbar {
            internal_callback_ptr: internal_callback_ptr,
            identifier: identifier,
            objc_controller: ToolbarHandle(objc_controller),
            controller: controller
        }
    }

    /// Indicates whether the toolbar shows the separator between the toolbar and the main window
    /// contents.
    pub fn set_shows_baseline_separator(&self, shows: bool) {
        self.objc_controller.set_shows_baseline_separator(shows);
    }

    /// Sets the toolbar's display mode.
    pub fn set_display_mode(&self, mode: ToolbarDisplayMode) {
        self.objc_controller.set_display_mode(mode);
    }

    /// Sets the toolbar's size mode.
    pub fn set_size_mode(&self, mode: ToolbarSizeMode) {
        self.objc_controller.set_size_mode(mode);
    }

    /// Set whether the toolbar is visible or not.
    pub fn set_visible(&self, visibility: bool) {
        self.objc_controller.set_visible(visibility);
    }
}

impl<T> Drop for Toolbar<T> {
    /// A bit of extra cleanup for delegate callback pointers.
    /// Note: this currently doesn't check to see if it needs to be removed from a Window it's
    /// attached to. In theory this is fine... in practice (and in Rust) it might be wonky, so
    /// worth circling back on at some point.
    fn drop(&mut self) {
        unsafe {
            let _ = Rc::from_raw(self.internal_callback_ptr);
        }
    }
}
