//! A progress indicator widget.
//!
//! This control wraps `NSProgressIndicator` on macOS, and 
//! `UIProgressView+UIActivityIndicatorView` on iOS and tvOS. It operates in two modes: determinate
//! (where you have a fixed start and end) and indeterminate (infinite; it will go and go until you
//! tell it to stop).
//!
//! ```rust,no_run
//! let indicator = ProgressIndicator::new();
//! indicator.set_indeterminate(true);
//! my_view.add_subview(&indicator);
//! ```

use core_graphics::base::CGFloat;

use objc_id::ShareId;
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSUInteger};
use crate::color::Color;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::register_progress_indicator_class;

mod enums;
pub use enums::ProgressIndicatorStyle;

/// A control used for reporting progress to a user visually.
#[derive(Debug)]
pub struct ProgressIndicator {
    /// A pointer to the Objective-C Object.
    pub objc: ShareId<Object>,

    /// A pointer to the Objective-C top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C bottom layout constraint.
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C width layout constraint.
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C height layout constraint.
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C center X layout constraint.
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C center Y layout constraint.
    pub center_y: LayoutAnchorY
}

impl Default for ProgressIndicator {
    fn default() -> Self {
        ProgressIndicator::new()
    }
}

impl ProgressIndicator {
    /// Returns a default `ProgressIndicator`. You should retain this yourself for as long as you
    /// need it to stay around.
    pub fn new() -> Self {
        let view = unsafe {
            #[cfg(feature = "macos")]
            let view: id = msg_send![class!(NSProgressIndicator), new];
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];

            #[cfg(feature = "macos")]
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
    /// Starts the animation for an indeterminate indicator.
    pub fn start_animation(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, startAnimation:nil];
        }
    }

    /// Stops any animations that are currently happening on this indicator (e.g, if it's an
    /// indeterminate looping animation).
    pub fn stop_animation(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, stopAnimation:nil];
        }
    }

    /// Increment the progress indicator by the amount specified.
    pub fn increment(&self, amount: f64) {
        unsafe {
            let _: () = msg_send![&*self.objc, incrementBy:amount];
        }
    }

    /// Set the style for the progress indicator.
    pub fn set_style(&self, style: ProgressIndicatorStyle) {
        unsafe {
            let style = style as NSUInteger;
            let _: () = msg_send![&*self.objc, setStyle:style];
        }
    }

    /// Set whether this is an indeterminate indicator or not. Indeterminate indicators are
    /// "infinite" and their appearance is that of a circular spinner.
    ///
    /// Invert this to go back to a bar appearance.
    pub fn set_indeterminate(&self, is_indeterminate: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setIndeterminate:match is_indeterminate {
                true => YES,
                false => NO
            }];
        }
    }

    /// Sets the value of this progress indicator.
    ///
    /// If this progress indicator is indeterminate, this will have no effect.
    pub fn set_value(&self, value: f64) {
        let value = value as CGFloat;

        unsafe {
            let _: () = msg_send![&*self.objc, setDoubleValue:value];
        }
    }

    /// Set whether this control is hidden or not.
    pub fn set_hidden(&self, hidden: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setHidden:match hidden {
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
    /// A bit of extra cleanup for delegate callback pointers. 
    /// If the originating `ProgressIndicator` is being
    /// dropped, we do some logic to clean it all up (e.g, we go ahead and check to see if
    /// this has a superview (i.e, it's in the heirarchy) on the Objective-C side. If it does, we go
    /// ahead and remove it - this is intended to match the semantics of how Rust handles things).
    ///
    /// There are, thankfully, no delegates we need to break here.
    fn drop(&mut self) {
        unsafe {
            let superview: id = msg_send![&*self.objc, superview];
            if superview != nil {
                let _: () = msg_send![&*self.objc, removeFromSuperview];
            }
        }
    }
}
