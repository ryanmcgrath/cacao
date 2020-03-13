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
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::pasteboard::PasteboardType;
use crate::view::controller::register_controller_class;
use crate::view::traits::ViewController;

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Debug, Default, Clone)]
pub struct ViewHandle {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: Option<ShareId<Object>>,

    /// A pointer to the Objective-C runtime top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    pub center_y: LayoutAnchorY
}

impl ViewHandle {
    pub(crate) fn new(object: ShareId<Object>) -> Self {
        let view: id = unsafe {
            msg_send![&*object, view]
        };

        ViewHandle {
            objc: Some(object),
            top: LayoutAnchorY::new(unsafe { msg_send![view, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![view, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![view, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![view, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![view, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![view, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![view, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![view, centerYAnchor] }),
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color(&self, color: Color) {
        if let Some(objc) = &self.objc {
            unsafe {
                let view: id = msg_send![*objc, view];
                (*view).set_ivar(BACKGROUND_COLOR, color.into_platform_specific_color());
                let _: () = msg_send![view, setNeedsDisplay:YES];
            }
        }
    }

    /// Register this view for drag and drop operations.
    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        if let Some(objc) = &self.objc {
            unsafe {
                let types = NSArray::arrayWithObjects(nil, &types.iter().map(|t| {
                    t.to_nsstring()
                }).collect::<Vec<id>>());

                let view: id = msg_send![*objc, view];
                let _: () = msg_send![view, registerForDraggedTypes:types];
            }
        }
    }

    pub fn add_subview<T: Layout>(&self, subview: &T) {
        if let Some(this) = &self.objc {
            if let Some(subview_controller) = subview.get_backing_node() {
                unsafe {
                    let _: () = msg_send![*this, addChildViewController:&*subview_controller];

                    let subview: id = msg_send![&*subview_controller, view];
                    let view: id = msg_send![*this, view];
                    let _: () = msg_send![view, addSubview:subview]; 
                }
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
