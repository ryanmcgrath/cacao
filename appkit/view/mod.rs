//! A `ViewHandle` represents an underlying `NSView`. You're passed a reference to one during your
//! `ViewController::did_load()` method. This method is safe to store and use, however as it's
//! UI-specific it's not thread safe.
//!
//! You can use this struct to configure how a view should look and layout. It implements
//! AutoLayout - for more information, see the AutoLayout tutorial.

use std::rc::Rc;
use std::cell::RefCell;

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, YES, NO, NSArray, NSString};
use crate::color::Color;
use crate::constants::{BACKGROUND_COLOR, VIEW_DELEGATE_PTR};
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::pasteboard::PasteboardType;

mod class;
use class::{register_view_class, register_view_class_with_delegate};

pub mod controller;
pub use controller::ViewController;

pub mod traits;
pub use traits::ViewDelegate;

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Debug)]
pub struct View<T = ()> {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ShareId<Object>,

    /// An internal callback pointer that we use in delegate loopbacks. Default implementations
    /// don't require this.
    internal_callback_ptr: Option<*const RefCell<T>>,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Rc<RefCell<T>>>,

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

impl Default for View {
    fn default() -> Self {
        View::new()
    }
}

impl View {
    /// Returns a default `View`, suitable for 
    pub fn new() -> Self {
        let view: id = unsafe {
            let view: id = msg_send![register_view_class(), new];
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            view
        };

        View {
            internal_callback_ptr: None,
            delegate: None,
            top: LayoutAnchorY::new(unsafe { msg_send![view, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![view, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![view, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![view, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![view, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![view, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![view, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![view, centerYAnchor] }),
            objc: ShareId::from_ptr(view),
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color(&self, color: Color) {
        unsafe {
            //let view: id = msg_send![*self.objc, view];
            //(*view).set_ivar(BACKGROUND_COLOR, color.into_platform_specific_color());
            //let _: () = msg_send![view, setNeedsDisplay:YES];
        }
    }

    /// Register this view for drag and drop operations.
    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        unsafe {
            let types: NSArray = types.into_iter().map(|t| {
                // This clone probably doesn't need to be here, but it should also be cheap as
                // this is just an enum... and this is not an oft called method.
                let x: NSString = t.clone().into();
                x.into_inner()
            }).collect::<Vec<id>>().into();

            let _: () = msg_send![&*self.objc, registerForDraggedTypes:types.into_inner()];
        }
    }

    /// Given a subview, adds it to this view.
    pub fn add_subview<T: Layout>(&self, subview: &T) {
            /*if let Some(subview_controller) = subview.get_backing_node() {
                unsafe {
                    let _: () = msg_send![*this, addChildViewController:&*subview_controller];

                    let subview: id = msg_send![&*subview_controller, view];
                    let view: id = msg_send![*this, view];
                    let _: () = msg_send![view, addSubview:subview]; 
                }
            }*/
    }
}

impl<T> View<T> where T: ViewDelegate + 'static {
    /// Initializes a new View with a given `ViewDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> View<T> {
        let delegate = Rc::new(RefCell::new(delegate));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&delegate);
            Rc::into_raw(cloned)
        };

        let view = unsafe {
            let view: id = msg_send![register_view_class_with_delegate::<T>(), new];
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            (&mut *view).set_ivar(VIEW_DELEGATE_PTR, internal_callback_ptr as usize);
            view
        };

        let view = View {
            internal_callback_ptr: Some(internal_callback_ptr),
            delegate: Some(delegate),
            top: LayoutAnchorY::new(unsafe { msg_send![view, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![view, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![view, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![view, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![view, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![view, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![view, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![view, centerYAnchor] }),
            objc: ShareId::from_ptr(view),
        };

        {
            let mut delegate = delegate.borrow_mut();
            (*delegate).did_load(View { 
                internal_callback_ptr: None,
                delegate: None,
                top: view.top.clone(),
                leading: view.leading.clone(),
                trailing: view.trailing.clone(),
                bottom: view.bottom.clone(),
                width: view.width.clone(),
                height: view.height.clone(),
                center_x: view.center_x.clone(),
                center_y: view.center_y.clone(),
                objc: view.objc.clone()
            });
        }

        view
    }
}

impl Layout for View {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, _: &V) {}
}

impl<T> Drop for View<T> {
    /// A bit of extra cleanup for delegate callback pointers.
    fn drop(&mut self) {
        if let ptr = &self.internal_callback_ptr {
            unsafe {
                let _ = Rc::from_raw(ptr);
            }
        }
    }
}
