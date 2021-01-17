//! A wrapper type for `NSArray`.
//!
//! This is abstracted out as we need to use `NSArray` in a ton of
//! instances in this framework, and down the road I'd like to investigate using `CFArray` instead
//! of `NSArray` (i.e, if the ObjC runtime is ever pulled or something - perhaps those types would
//! stick around).
//!
//! Essentially, consider this some sanity/cleanliness/future-proofing. End users should never need
//! to touch this.

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::id;

/// A wrapper for `NSArray` that makes common operations in our framework a bit easier to handle
/// and reason about.
#[derive(Debug)]
pub struct NSArray(pub Id<Object>);

impl From<Vec<&Object>> for NSArray {
    /// Given a set of `Object`s, creates an `NSArray` that holds them.
    fn from(objects: Vec<&Object>) -> Self {
        NSArray(unsafe {
            Id::from_ptr(msg_send![class!(NSArray), arrayWithObjects:objects.as_ptr() count:objects.len()])
        })
    }
}

impl From<Vec<id>> for NSArray {
    /// Given a set of `*mut Object`s, creates an `NSArray` that holds them.
    fn from(objects: Vec<id>) -> Self {
        NSArray(unsafe {
            Id::from_ptr(msg_send![class!(NSArray), arrayWithObjects:objects.as_ptr() count:objects.len()])
        })
    }
}

impl NSArray {
    /// Given a set of `Object`s, creates an `NSArray` that holds them.
    pub fn new(objects: &[id]) -> Self {
        NSArray(unsafe {
            Id::from_ptr(msg_send![class!(NSArray), arrayWithObjects:objects.as_ptr() count:objects.len()])
        })
    }

    /// In some cases, we're vended an `NSArray` by the system, and it's ideal to not retain that.
    /// This handles that edge case.
    pub fn wrap(array: id) -> Self {
        NSArray(unsafe {
            Id::from_ptr(array)
        })
    }

    /// Consumes and returns the underlying Objective-C value.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }

    /// Returns the `count` (`len()` equivalent) for the backing `NSArray`.
    pub fn count(&self) -> usize {
        unsafe { msg_send![self.0, count] }
    }

    /// A helper method for mapping over the backing `NSArray` items.
    /// Often times we need to map in this framework to convert between Rust types, so isolating
    /// this out makes life much easier.
    pub fn map<T, F: Fn(id) -> T>(&self, transform: F) -> Vec<T> {
        let count = self.count();
        let mut ret: Vec<T> = Vec::with_capacity(count);
        let mut index = 0;

        loop {
            let item: id = unsafe { msg_send![&*self.0, objectAtIndex:index] };
            ret.push(transform(item));
                
            index += 1;
            if index == count { break }
        }

        ret
    }
}
