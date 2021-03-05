//! Implements `Font`, a wrapper around `NSFont` on macOS and `UIFont` on iOS.

use core_graphics::base::CGFloat;

use objc_id::ShareId;
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSArray, NSString};

#[derive(Debug)]
pub struct Font {
    pub objc: ShareId<Object>
}

impl Default for Font {
    fn default() -> Self {
        Font {
            objc: unsafe {
                let cls = class!(NSFont);
                let default_size: id = msg_send![cls, labelFontSize];
                ShareId::from_ptr(msg_send![cls, labelFontOfSize:default_size])
            }
        }
    }
}

impl Font {
    pub fn system(size: CGFloat) -> Self {
        Font {
            objc: unsafe {
                ShareId::from_ptr(msg_send![class!(NSFont), systemFontOfSize:size])
            }
        }
    }

    pub fn bold_system(size: CGFloat) -> Self {
        Font {
            objc: unsafe {
                ShareId::from_ptr(msg_send![class!(NSFont), boldSystemFontOfSize:size])
            }
        }
    }
}
