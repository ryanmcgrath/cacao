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
use crate::utils::properties::ObjcProperty;

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
    pub objc: ObjcProperty,
    
    /// A pointer to the Objective-C runtime top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    pub right: LayoutAnchorX,

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
            top: LayoutAnchorY::top(view),
            left: LayoutAnchorX::left(view),
            leading: LayoutAnchorX::leading(view),
            right: LayoutAnchorX::right(view),
            trailing: LayoutAnchorX::trailing(view),
            bottom: LayoutAnchorY::bottom(view),
            width: LayoutAnchorDimension::width(view),
            height: LayoutAnchorDimension::height(view),
            center_x: LayoutAnchorX::center(view),
            center_y: LayoutAnchorY::center(view),
            objc: ObjcProperty::retain(view),
        }
    }
}

impl ProgressIndicator {
    /// Starts the animation for an indeterminate indicator.
    pub fn start_animation(&self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, startAnimation:nil];
        });
    }

    /// Stops any animations that are currently happening on this indicator (e.g, if it's an
    /// indeterminate looping animation).
    pub fn stop_animation(&self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, stopAnimation:nil];
        });
    }

    /// Increment the progress indicator by the amount specified.
    pub fn increment(&self, amount: f64) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, incrementBy:amount];
        });
    }

    /// Set the style for the progress indicator.
    pub fn set_style(&self, style: ProgressIndicatorStyle) {
        let style = style as NSUInteger;
        
        self.objc.with_mut(move |obj| unsafe {
            let _: () = msg_send![obj, setStyle:style];
        });
    }

    /// Set whether this is an indeterminate indicator or not. Indeterminate indicators are
    /// "infinite" and their appearance is that of a circular spinner.
    ///
    /// Invert this to go back to a bar appearance.
    pub fn set_indeterminate(&self, is_indeterminate: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setIndeterminate:match is_indeterminate {
                true => YES,
                false => NO
            }];
        });
    }

    /// Sets the value of this progress indicator.
    ///
    /// If this progress indicator is indeterminate, this will have no effect.
    pub fn set_value(&self, value: f64) {
        let value = value as CGFloat;

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setDoubleValue:value];
        });
    }

    /// Set whether this control is hidden or not.
    pub fn set_hidden(&self, hidden: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setHidden:match hidden {
                true => YES,
                false => NO
            }];
        });
    }
}

impl Layout for ProgressIndicator {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_node<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
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
        /*unsafe {
            let superview: id = msg_send![&*self.objc, superview];
            if superview != nil {
                let _: () = msg_send![&*self.objc, removeFromSuperview];
            }
        }*/
    }
}
