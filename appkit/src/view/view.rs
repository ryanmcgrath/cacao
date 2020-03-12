//! A wrapper for `NSViewController`. Uses interior mutability to 

use std::cell::RefCell;
use std::rc::Rc;

use cocoa::base::{id, nil, YES};
use cocoa::foundation::NSArray;

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::color::Color;
use crate::constants::{BACKGROUND_COLOR, VIEW_CONTROLLER_PTR};
use crate::pasteboard::PasteboardType;
use crate::view::traits::{Node, ViewController};
use crate::view::controller::register_controller_class;

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Debug, Default, Clone)]
pub struct ViewHandle(Option<ShareId<Object>>);

impl ViewHandle {
    /// Call this to set the background color for the backing layer.
    pub fn set_background_color(&self, color: Color) {
        if let Some(controller) = &self.0 {
            unsafe {
                let view: id = msg_send![*controller, view];
                (*view).set_ivar(BACKGROUND_COLOR, color.into_platform_specific_color());
                let _: () = msg_send![view, setNeedsDisplay:YES];
            }
        }
    }

    /// Register this view for drag and drop operations.
    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        if let Some(controller) = &self.0 {
            unsafe {
                let types = NSArray::arrayWithObjects(nil, &types.iter().map(|t| {
                    t.to_nsstring()
                }).collect::<Vec<id>>());

                let view: id = msg_send![*controller, view];
                let _: () = msg_send![view, registerForDraggedTypes:types];
            }
        }
    }
}

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

        {
            let mut vc = controller.borrow_mut();
            (*vc).did_load(ViewHandle(Some(inner.clone())));
        }

        View {
            internal_callback_ptr: internal_callback_ptr,
            objc_controller: ViewHandle(Some(inner)),
            controller: controller
        }
    }

    pub fn set_background_color(&self, color: Color) {
        self.objc_controller.set_background_color(color);
    }

    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        self.objc_controller.register_for_dragged_types(types);
    }
}

impl<T> Node for View<T> {
    /// Returns the Objective-C object used for handling the view heirarchy.
    fn get_backing_node(&self) -> Option<ShareId<Object>> {
        self.objc_controller.0.clone()
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
