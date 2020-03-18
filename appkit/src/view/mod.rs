//! This module implements tooling for constructing Views. Notably, it provides the following:
//!
//! - A `View` type, which holds your `impl ViewController` and handles routing around platform
//! lifecycle methods accordingly. This is a heap allocation.
//! - A `ViewController` trait, which enables you to hook into the `NSViewController` lifecycle
//! methods.
//! - A `ViewHandle` struct, which wraps a platform-provided `NSView` and enables you to configure
//! things such as appearance and layout.
//!
//! You can use it like the following:
//!
//! ```
//! use appkit::prelude::{View, ViewController, ViewHandle};
//! use appkit::color::rgb;
//! 
//! #[derive(Default)]
//! pub struct MyView {
//!     pub view: ViewHandle
//! }
//!
//! impl ViewController for MyView {
//!     fn did_load(&mut self, view: ViewHandle) {
//!         self.view = view;
//!         self.view.set_background_color(rgb(0, 0, 0));
//!     }
//! }
//! ```
//!
//! For more information and examples, consult the sample code distributed in the git repository.

use std::rc::Rc;
use std::cell::RefCell;

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::foundation::id;
use crate::color::Color;
use crate::constants::VIEW_CONTROLLER_PTR;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::pasteboard::PasteboardType;

mod class;
mod controller;
use controller::register_controller_class;

pub mod traits;
pub use traits::ViewController;

pub mod handle;
pub use handle::ViewHandle;

/// A `View` wraps two different controllers - one on the Objective-C/Cocoa side, which forwards
/// calls into your supplied `ViewController` trait object. This involves heap allocation, but all
/// of Cocoa is essentially Heap'd, so... well, enjoy.
#[derive(Clone)]
pub struct View<T> {
    internal_callback_ptr: *const RefCell<T>,
    pub objc_controller: ViewHandle,
    pub controller: Rc<RefCell<T>>
}

impl<T> View<T> where T: ViewController + 'static {
    /// Allocates and configures a `ViewController` in the Objective-C/Cocoa runtime that maps over
    /// to your supplied view controller.
    pub fn new(controller: T) -> Self {
        let controller = Rc::new(RefCell::new(controller));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&controller);
            Rc::into_raw(cloned)
        };

        let inner = unsafe {
            let view_controller: id = msg_send![register_controller_class::<T>(), new];
            (&mut *view_controller).set_ivar(VIEW_CONTROLLER_PTR, internal_callback_ptr as usize);
            
            let view: id = msg_send![view_controller, view];
            (&mut *view).set_ivar(VIEW_CONTROLLER_PTR, internal_callback_ptr as usize);
            
            ShareId::from_ptr(view_controller)
        };

        let handle = ViewHandle::new(inner);

        {
            let mut vc = controller.borrow_mut();
            (*vc).did_load(handle.clone());
        }

        View {
            internal_callback_ptr: internal_callback_ptr,
            objc_controller: handle,
            controller: controller
        }
    }

    pub fn set_background_color(&self, color: Color) {
        self.objc_controller.set_background_color(color);
    }

    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        self.objc_controller.register_for_dragged_types(types);
    }

    pub fn top(&self) -> &LayoutAnchorY {
        &self.objc_controller.top
    }

    pub fn leading(&self) -> &LayoutAnchorX {
        &self.objc_controller.leading
    }

    pub fn trailing(&self) -> &LayoutAnchorX {
        &self.objc_controller.trailing
    }

    pub fn bottom(&self) -> &LayoutAnchorY {
        &self.objc_controller.bottom
    }

    pub fn width(&self) -> &LayoutAnchorDimension {
        &self.objc_controller.width
    }

    pub fn height(&self) -> &LayoutAnchorDimension {
        &self.objc_controller.height
    }
}

impl<T> Layout for View<T> {
    /// Returns the Objective-C object used for handling the view heirarchy.
    fn get_backing_node(&self) -> Option<ShareId<Object>> {
        self.objc_controller.objc.clone()
    }

    fn add_subview<V: Layout>(&self, subview: &V) {
        self.objc_controller.add_subview(subview);
    }
}

impl<T> std::fmt::Debug for View<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View ({:p})", self)
    }
}

impl<T> Drop for View<T> {
    /// A bit of extra cleanup for delegate callback pointers.
    fn drop(&mut self) {
        unsafe {
            let _ = Rc::from_raw(self.internal_callback_ptr);
        }
    }
}
