use std::cell::Ref;

use core_foundation::base::TCFType;

use objc::rc::{Id, Owned, Shared};
use objc::runtime::{Class, Object};
use objc::{msg_send, msg_send_id, sel};

use crate::color::Color;
use crate::foundation::{id, nil, NSArray, NSString, NO, YES};
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;

use crate::layer::Layer;
#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};

#[cfg(feature = "appkit")]
mod appkit;

#[cfg(feature = "appkit")]
use appkit::register_image_view_class;

#[cfg(feature = "uikit")]
mod uikit;

#[cfg(all(feature = "uikit", not(feature = "appkit")))]
use uikit::register_image_view_class;

mod image;
pub use image::{DrawConfig, Image, ResizeBehavior};

mod icons;
pub use icons::*;

/// A helper method for instantiating view classes and applying default settings to them.
fn allocate_view(registration_fn: fn() -> &'static Class) -> id {
    unsafe {
        let view: id = msg_send![registration_fn(), new];

        #[cfg(feature = "autolayout")]
        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints: NO];

        #[cfg(feature = "appkit")]
        let _: () = msg_send![view, setWantsLayer: YES];

        view
    }
}

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Clone, Debug)]
pub struct ImageView {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ObjcProperty,

    /// References the underlying layer. This is consistent across AppKit & UIKit - in AppKit
    /// we explicitly opt in to layer backed views.
    pub layer: Layer,

    /// A pointer to the Objective-C runtime top layout constraint.
    #[cfg(feature = "autolayout")]
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    #[cfg(feature = "autolayout")]
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    #[cfg(feature = "autolayout")]
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    #[cfg(feature = "autolayout")]
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    #[cfg(feature = "autolayout")]
    pub right: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    #[cfg(feature = "autolayout")]
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    #[cfg(feature = "autolayout")]
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    #[cfg(feature = "autolayout")]
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_y: LayoutAnchorY
}

impl Default for ImageView {
    fn default() -> Self {
        ImageView::new()
    }
}

impl ImageView {
    /// Returns a default `View`, suitable for
    pub fn new() -> Self {
        let view = allocate_view(register_image_view_class);

        ImageView {
            #[cfg(feature = "autolayout")]
            top: LayoutAnchorY::top(view),

            #[cfg(feature = "autolayout")]
            left: LayoutAnchorX::left(view),

            #[cfg(feature = "autolayout")]
            leading: LayoutAnchorX::leading(view),

            #[cfg(feature = "autolayout")]
            right: LayoutAnchorX::right(view),

            #[cfg(feature = "autolayout")]
            trailing: LayoutAnchorX::trailing(view),

            #[cfg(feature = "autolayout")]
            bottom: LayoutAnchorY::bottom(view),

            #[cfg(feature = "autolayout")]
            width: LayoutAnchorDimension::width(view),

            #[cfg(feature = "autolayout")]
            height: LayoutAnchorDimension::height(view),

            #[cfg(feature = "autolayout")]
            center_x: LayoutAnchorX::center(view),

            #[cfg(feature = "autolayout")]
            center_y: LayoutAnchorY::center(view),

            layer: Layer::from_id(unsafe { msg_send_id![view, layer] }),

            objc: ObjcProperty::retain(view)
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        self.objc.with_mut(|obj| unsafe {
            let cg = color.as_ref().cg_color().as_concrete_TypeRef();
            let layer: id = msg_send![obj, layer];
            let _: () = msg_send![layer, setBackgroundColor: cg];
        });
    }

    /// Given an image reference, sets it on the image view. You're currently responsible for
    /// retaining this yourself.
    pub fn set_image(&self, image: &Image) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setImage:&*image.0];
        });
    }

    /*pub fn set_image_scaling(&self, scaling_type: ImageScale) {
        self.objc.with_mut(|obj| unsafe {

            let _: () = msg_send![obj, setImageScaling:
        });
    }*/
}

impl ObjcAccess for ImageView {
    fn with_backing_obj_mut(&self, handler: &dyn Fn(id)) {
        self.objc.with_mut(handler);
    }

    fn get_backing_obj(&self) -> Ref<'_, Id<Object, Owned>> {
        self.objc.get_ref()
    }
}

impl Layout for ImageView {}

impl Drop for ImageView {
    /// A bit of extra cleanup for delegate callback pointers. If the originating `View` is being
    /// dropped, we do some logic to clean it all up (e.g, we go ahead and check to see if
    /// this has a superview (i.e, it's in the heirarchy) on the AppKit side. If it does, we go
    /// ahead and remove it - this is intended to match the semantics of how Rust handles things).
    ///
    /// There are, thankfully, no delegates we need to break here.
    fn drop(&mut self) {
        /*if self.delegate.is_some() {
            unsafe {
                let superview: id = msg_send![&*self.objc, superview];
                if superview != nil {
                    let _: () = msg_send![&*self.objc, removeFromSuperview];
                }
            }
        }*/
    }
}

#[test]
fn test_image() {
    let image_view = ImageView::new();
    image_view.set_background_color(Color::SystemBlue);
    let image_bytes = include_bytes!("../../test-data/favicon.ico");
    let image = Image::with_data(image_bytes);
    image_view.set_image(&image);
}
