//! Implements `Font`, a wrapper around `NSFont` on macOS and `UIFont` on iOS.

use std::ops::Deref;

use objc::foundation::CGFloat;
use objc::rc::{Id, Shared};
use objc::runtime::{Class, Object};
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, nil, NSArray, NSString, NO, YES};
use crate::utils::os;

/// A `Font` can be constructed and applied to supported controls to control things like text
/// appearance and size.
#[derive(Clone, Debug)]
pub struct Font(pub Id<Object, Shared>);

impl Default for Font {
    /// Returns the default `labelFont` on macOS.
    fn default() -> Self {
        let cls = Self::class();
        let default_size: id = unsafe { msg_send![cls, labelFontSize] };

        #[cfg(feature = "appkit")]
        let font = Font(unsafe { msg_send_id![cls, labelFontOfSize: default_size] });

        #[cfg(all(feature = "uikit", not(feature = "appkit")))]
        let font = Font(unsafe { msg_send_id![cls, systemFontOfSize: default_size] });
        font
    }
}

impl Font {
    fn class() -> &'static Class {
        #[cfg(feature = "appkit")]
        let class = class!(NSFont);
        #[cfg(all(feature = "uikit", not(feature = "appkit")))]
        let class = class!(UIFont);

        class
    }
    /// Creates and returns a default system font at the specified size.
    pub fn system(size: f64) -> Self {
        let size = size as CGFloat;

        Font(unsafe { msg_send_id![Self::class(), systemFontOfSize: size] })
    }

    /// Creates and returns a default bold system font at the specified size.
    pub fn bold_system(size: f64) -> Self {
        let size = size as CGFloat;

        Font(unsafe { msg_send_id![Self::class(), boldSystemFontOfSize: size] })
    }

    /// Creates and returns a monospace system font at the specified size and weight
    ///
    /// # Support
    ///
    /// The `monospace` font feature is available from version `10.15`.
    ///
    /// If the current system is using an older version the `monospacedSystemFontOfSize`
    /// option will be omitted.
    pub fn monospace(size: f64, weight: f64) -> Self {
        let size = size as CGFloat;
        let weight = weight as CGFloat;

        if os::is_minimum_semversion(10, 15, 0) {
            Font(unsafe { msg_send_id![class!(NSFont), monospacedSystemFontOfSize: size, weight: weight] })
        } else {
            Font(unsafe { msg_send_id![class!(NSFont), systemFontOfSize: size, weight: weight] })
        }
    }
}

impl Deref for Font {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.0
    }
}

impl AsRef<Font> for Font {
    /// Provided to make passing `Font` types around less of a headache.
    #[inline]
    fn as_ref(&self) -> &Font {
        self
    }
}

#[test]
fn font_test() {
    let default_font = Font::default();
    let system_font = Font::system(100.0);
    let bold_system_font = Font::bold_system(100.0);
}
