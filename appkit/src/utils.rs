//! Utils is a dumping ground for various methods that don't really have a particular module they
//! belong to. These are typically internal, and if you rely on them... well, don't be surprised if
//! they go away one day.

use std::rc::Rc;
use std::cell::RefCell;
use std::{slice, str};
use std::os::raw::c_char;

use cocoa::base::id;
use cocoa::foundation::NSString;

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;

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

/// Used for moving a pointer back into an Rc, so we can work with the object held behind it. Note
/// that it's very important to make sure you reverse this when you're done (using
/// `Rc::into_raw()`) otherwise you'll cause problems due to the `Drop` logic.
pub fn load<T>(this: &Object, ptr: &str) -> Rc<RefCell<T>> {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr);
        let view_ptr = ptr as *const RefCell<T>;
        Rc::from_raw(view_ptr)
    }
}
