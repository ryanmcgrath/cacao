use objc::runtime::Object;
use objc::{class, msg_send, sel};

use crate::foundation::{id, NSUInteger, NO, YES};
use crate::objc_access::ObjcAccess;

/// Use this enum for specifying NSControl size types.
#[derive(Copy, Clone, Debug)]
pub enum ControlSize {
    /// The smallest control size.
    Mini,

    /// A smaller control size.
    Small,

    /// The default, regular, size.
    Regular,

    /// A large control. Only available on macOS 11.0+.
    /// If you pass this to the `set_control_size` method on the `Control` trait, it will
    /// transparently map to `Regular` on 10.15 and below.
    Large
}

/// A trait that view wrappers must conform to. Enables managing the subview tree.
#[allow(unused_variables)]
pub trait Control: ObjcAccess {
    /// Whether this control is enabled or not.
    fn set_enabled(&self, is_enabled: bool) {
        self.with_backing_obj_mut(&|obj| unsafe {
            let _: () = msg_send![obj, setEnabled:match is_enabled {
                true => YES,
                false => NO
            }];
        });
    }

    /// Sets the underlying control size.
    fn set_control_size(&self, size: ControlSize) {
        let control_size: NSUInteger = match size {
            ControlSize::Mini => 2,
            ControlSize::Small => 1,
            ControlSize::Regular => 0,

            ControlSize::Large => match crate::utils::os::is_minimum_version(11) {
                true => 3,
                false => 0
            }
        };

        self.with_backing_obj_mut(&|obj| unsafe {
            let _: () = msg_send![obj, setControlSize: control_size];
        });
    }
}
