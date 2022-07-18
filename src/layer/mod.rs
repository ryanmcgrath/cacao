//! Wraps `CALayer` across all platforms.
//!
//! Each widget has an underlying `layer` field that you can access, which offers additional
//! rendering tools.
//!
//! ```rust,no_run
//! // Create a rounded red box
//! let view = View::default();
//! view.set_background_color(Color::SystemRed);
//! view.layer.set_corner_radius(4.0);
//! ```

use core_graphics::base::CGFloat;

use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::id;
use crate::utils::properties::ObjcProperty;

/// Represents a `CALayer`.
/// 
/// Each widget has an underlying `layer` field that you can access, which offers additional
/// rendering tools.
///
/// ```rust,no_run
/// // Create a rounded red box
/// let view = View::default();
/// view.set_background_color(Color::SystemRed);
/// view.layer.set_corner_radius(4.0);
/// ```
#[derive(Clone, Debug)]
pub struct Layer {
    /// The underlying layer pointer.
    pub objc: ObjcProperty
}

impl Layer {
    /// Creates a new `CALayer` and retains it.
    pub fn new() -> Self {
        Layer {
            objc: ObjcProperty::retain(unsafe {
                msg_send![class!(CALayer), new]
            })
        }
    }

    /// Wraps an existing (already retained) `CALayer`.
    pub fn wrap(layer: id) -> Self {
        Layer {
            objc: ObjcProperty::from_retained(layer)
        }
    }

    /// Sets the corner radius (for all four corners).
    ///
    /// Note that for performance sensitive contexts, you might want to apply a mask instead.
    pub fn set_corner_radius(&self, radius: f64) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setCornerRadius:radius as CGFloat];
        });
    }
}
