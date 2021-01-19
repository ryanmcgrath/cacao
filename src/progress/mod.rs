use objc_id::ShareId;
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSUInteger};
use crate::color::Color;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::{register_progress_indicator_class};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::{register_progress_indicator_class};

mod enums;
pub use enums::ProgressIndicatorStyle;

#[derive(Debug)]
pub struct ProgressIndicator {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ShareId<Object>,

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

impl Default for ProgressIndicator {
    fn default() -> Self {
        ProgressIndicator::new()
    }
}

impl ProgressIndicator {
    /// Returns a default `ProgressIndicator`, suitable for 
    pub fn new() -> Self {
        let view = unsafe {
            let view: id = msg_send![register_progress_indicator_class(), new];
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];

            #[cfg(target_os = "macos")]
            let _: () = msg_send![view, setWantsLayer:YES];

            view
        };

        ProgressIndicator {
            top: LayoutAnchorY::new(unsafe { msg_send![view, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![view, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![view, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![view, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![view, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![view, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![view, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![view, centerYAnchor] }),
            objc: unsafe { ShareId::from_ptr(view) },
        }
    }
}

impl ProgressIndicator {
    /// Call this to set the background color for the backing layer.
    pub fn set_background_color(&self, color: Color) {
        let bg = color.into_platform_specific_color();
        
        unsafe {
            let cg: id = msg_send![bg, CGColor];
            let layer: id = msg_send![&*self.objc, layer];
            let _: () = msg_send![layer, setBackgroundColor:cg];
        }
    }

    /// Starts the animation for an indeterminate indicator.
    pub fn start_animation(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, startAnimation:nil];
        }
    }

    pub fn stop_animation(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, stopAnimation:nil];
        }
    }

    pub fn increment(&self, by: f64) {
        unsafe {
            let _: () = msg_send![&*self.objc, incrementBy:by];
        }
    }

    pub fn set_style(&self, style: ProgressIndicatorStyle) {
        unsafe {
            let style = style as NSUInteger;
            let _: () = msg_send![&*self.objc, setStyle:style];
        }
    }

    pub fn set_indeterminate(&self, is_indeterminate: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setIndeterminate:match is_indeterminate {
                true => YES,
                false => NO
            }];
        }
    }
}

impl Layout for ProgressIndicator {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, view: &V) {
        let backing_node = view.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, addSubview:backing_node];
        }
    }
}

impl Drop for ProgressIndicator {
    /// A bit of extra cleanup for delegate callback pointers. If the originating `ProgressIndicator` is being
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
