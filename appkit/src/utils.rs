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

/// A utility method for mapping over NSArray instances. There's a number of places where we want
/// or need this functionality to provide Rust interfaces - this tries to do it in a way where the
/// `Vec` doesn't need to resize after being allocated.
pub fn map_nsarray<T, F>(array: id, transform: F) -> Vec<T>
where F: Fn(id) -> T {
    let count: usize = unsafe { msg_send![array, count] };

    let mut ret: Vec<T> = Vec::with_capacity(count);
    let mut index = 0;

    loop {
        let file: id = unsafe { msg_send![array, objectAtIndex:index] };
        ret.push(transform(file));
            
        index += 1;
        if index == count { break }
    }

    ret
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
