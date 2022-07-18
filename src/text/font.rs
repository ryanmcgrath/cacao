//! Implements `Font`, a wrapper around `NSFont` on macOS and `UIFont` on iOS.

use std::ops::Deref;

use core_graphics::base::CGFloat;

use objc_id::ShareId;
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSArray, NSString};

/// A `Font` can be constructed and applied to supported controls to control things like text
/// appearance and size.
#[derive(Clone, Debug)]
pub struct Font(pub ShareId<Object>);

impl Default for Font {
    /// Returns the default `labelFont` on macOS.
    fn default() -> Self {
        Font(unsafe {
            let cls = class!(NSFont);
            let default_size: id = msg_send![cls, labelFontSize];
            ShareId::from_ptr(msg_send![cls, labelFontOfSize:default_size])
        })
    }
}

impl Font {
    /// Creates and returns a default system font at the specified size.
    pub fn system(size: f64) -> Self {
        let size = size as CGFloat;

        Font(unsafe {
            ShareId::from_ptr(msg_send![class!(NSFont), systemFontOfSize:size])
        })
    }

    /// Creates and returns a default bold system font at the specified size.
    pub fn bold_system(size: f64) -> Self {
        let size = size as CGFloat;

        Font(unsafe {
            ShareId::from_ptr(msg_send![class!(NSFont), boldSystemFontOfSize:size])
        })
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
