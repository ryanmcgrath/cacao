//! Utils is a dumping ground for various methods that don't really have a particular module they
//! belong to. These are typically internal, and if you rely on them... well, don't be surprised if
//! they go away one day.

use std::{slice, str};
use std::os::raw::c_char;

use cocoa::base::id;
use cocoa::foundation::NSString;

use objc::{msg_send, sel, sel_impl};

/// A utility method for taking an `NSString` and bridging it to a Rust `&str`.
pub fn str_from(nsstring: id) -> &'static str {
    unsafe {
        let bytes = {
            let bytes: *const c_char = msg_send![nsstring, UTF8String];
            bytes as *const u8
        };

        let len = nsstring.len();
        let bytes = slice::from_raw_parts(bytes, len);
        str::from_utf8(bytes).unwrap()
    }
}
