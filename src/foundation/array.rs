use std::ops::{Deref, DerefMut};

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::id;

use super::Retainable;

/// A wrapper for `NSArray` that makes common operations in our framework a bit easier to handle
/// and reason about. This also provides a central place to look at replacing with `CFArray` if
/// ever deemed necessary (unlikely, given how much Apple has optimized the Foundation classes, but
/// still...).
#[derive(Debug)]
pub struct NSArray(pub Id<Object>);

impl NSArray {
    /// Given a set of `Object`s, creates and retains an `NSArray` that holds them.
    pub fn new(objects: &[id]) -> Self {
        NSArray(unsafe {
            Id::from_ptr(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        })
    }

    /// Returns the `count` (`len()` equivalent) for the backing `NSArray`.
    pub fn count(&self) -> usize {
        unsafe { msg_send![&*self.0, count] }
    }

    /// Returns an iterator over the `NSArray`
    pub fn iter<'a>(&'a self) -> NSArrayIterator<'a> {
        NSArrayIterator {
            next_index: 0,
            count: self.count(),
            array: self
        }
    }
}

impl Retainable for NSArray {
    /// In some cases, we're vended an `NSArray` by the system that we need to call retain on.
    /// This handles that case.
    fn retain(array: id) -> Self {
        NSArray(unsafe { Id::from_ptr(array) })
    }

    /// In some cases, we're vended an `NSArray` by the system, and it's ideal to not retain that.
    /// This handles that edge case.
    fn from_retained(array: id) -> Self {
        NSArray(unsafe { Id::from_retained_ptr(array) })
    }
}

#[derive(Debug)]
pub struct NSArrayIterator<'a> {
    next_index: usize,
    count: usize,

    array: &'a NSArray
}

impl Iterator for NSArrayIterator<'_> {
    type Item = id;

    fn next(&mut self) -> Option<Self::Item> {
        // I don't know if it's worth trying to get in with NSFastEnumeration here. I'm content to
        // just rely on Rust, but someone is free to profile it if they want.
        if self.next_index < self.count {
            let objc = &*self.array.0;
            let index = self.next_index;

            let item: id = unsafe { msg_send![objc, objectAtIndex: index] };

            self.next_index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl From<Vec<&Object>> for NSArray {
    /// Given a set of `Object`s, creates an `NSArray` that holds them.
    fn from(objects: Vec<&Object>) -> Self {
        NSArray(unsafe {
            Id::from_ptr(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        })
    }
}

impl From<Vec<id>> for NSArray {
    /// Given a set of `*mut Object`s, creates an `NSArray` that holds them.
    fn from(objects: Vec<id>) -> Self {
        NSArray(unsafe {
            Id::from_ptr(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        })
    }
}

impl From<NSArray> for id {
    /// Consumes and returns the pointer to the underlying NSArray.
    fn from(mut array: NSArray) -> Self {
        &mut *array
    }
}

impl Deref for NSArray {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.0
    }
}

impl DerefMut for NSArray {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.0
    }
}
