//! A `ViewHandle` represents an underlying `NSView`. You're passed a reference to one during your
//! `ViewController::did_load()` method. This method is safe to store and use, however as it's
//! UI-specific it's not thread safe.
//!
//! You can use this struct to configure how a view should look and layout. It implements
//! AutoLayout - for more information, see the AutoLayout tutorial.

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, YES, NSArray, NSString};
use crate::color::Color;
use crate::constants::BACKGROUND_COLOR;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::pasteboard::PasteboardType;

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
                let types: NSArray = types.into_iter().map(|t| {
                    // This clone probably doesn't need to be here, but it should also be cheap as
                    // this is just an enum... and this is not an oft called method.
                    let x: NSString = t.clone().into();
                    x.into_inner()
                }).collect::<Vec<id>>().into();

                let view: id = msg_send![*objc, view];
                let _: () = msg_send![view, registerForDraggedTypes:types.into_inner()];
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
