//! Wraps `CALayer` across all platforms.
//!
//! Each widget has an underlying `layer` field that you can access, which offers additional
//! rendering tools.
//!
//! ```rust,no_run
//! // Create a rounded red box
//! use cacao::view::View;
//! use cacao::color::Color;
//! let view = View::default();
//! view.set_background_color(Color::SystemRed);
//! view.layer.set_corner_radius(4.0);
//! ```

use core_graphics::base::CGFloat;

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::id;
use crate::utils::properties::ObjcProperty;

/// Represents a `CALayer`.
///
/// Each widget has an underlying `layer` field that you can access, which offers additional
/// rendering tools.
///
/// ```rust,no_run
/// // Create a rounded red box
/// use cacao::view::View;
/// use cacao::color::Color;
/// let view = View::default();
/// view.set_background_color(Color::SystemRed);
/// view.layer.set_corner_radius(4.0);
/// ```
#[derive(Clone, Debug)]
pub struct Layer {
    /// The underlying layer pointer.
    pub objc: Id<Object, Shared>
}

impl Layer {
    /// Creates a new `CALayer` and retains it.
    pub fn new() -> Self {
        Layer {
            objc: unsafe { msg_send_id![class!(CALayer), new] }
        }
    }

    /// Wraps an existing `CALayer`.
    pub fn from_id(objc: Id<Object, Shared>) -> Self {
        Layer { objc }
    }

    /// Sets the corner radius (for all four corners).
    ///
    /// Note that for performance sensitive contexts, you might want to apply a mask instead.
    pub fn set_corner_radius(&self, radius: f64) {
        let _: () = unsafe { msg_send![&self.objc, setCornerRadius: radius as CGFloat] };
    }
}
