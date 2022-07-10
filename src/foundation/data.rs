use std::mem;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::slice;

use block::{Block, ConcreteBlock};

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, to_bool, BOOL, YES, NO, NSUInteger};

/// Wrapper for a retained `NSData` object.
///
/// Supports constructing a new `NSData` from a `Vec<u8>`, wrapping and retaining an existing
/// pointer from the Objective-C side, and turning an `NSData` into a `Vec<u8>`.
///
/// This is an intentionally limited API.
#[derive(Debug)]
pub struct NSData(pub Id<Object>);

impl NSData {
    /// Given a vector of bytes, creates, retains, and returns a wrapped `NSData`.
    ///
    /// This method is borrowed straight out of [objc-foundation](objc-foundation) by the amazing
    /// Steven Sheldon, and just tweaked slightly to fit the desired API semantics here.
    ///
    /// [objc-foundation]: https://crates.io/crates/objc-foundation
    pub fn new(bytes: Vec<u8>) -> Self {
        let capacity = bytes.capacity();

        let dealloc = ConcreteBlock::new(move |bytes: *mut c_void, len: usize| unsafe {
            // Recreate the Vec and let it drop
            let _ = Vec::from_raw_parts(bytes as *mut u8, len, capacity);
        });
        let dealloc = dealloc.copy();
        let dealloc: &Block<(*mut c_void, usize), ()> = &dealloc;

        let mut bytes = bytes;
        let bytes_ptr = bytes.as_mut_ptr() as *mut c_void;

        unsafe {
            let obj: id = msg_send![class!(NSData), alloc];
            let obj: id = msg_send![obj, initWithBytesNoCopy:bytes_ptr
                                                             length:bytes.len()
                                                        deallocator:dealloc];
            mem::forget(bytes);
            NSData(Id::from_ptr(obj))
        }
    }

    /// Given a slice of bytes, creates, retains, and returns a wrapped `NSData`.
    ///
    /// This method is borrowed straight out of [objc-foundation](objc-foundation) by the amazing
    /// Steven Sheldon, and just tweaked slightly to fit the desired API semantics here.
    ///
    /// [objc-foundation]: https://crates.io/crates/objc-foundation
    pub fn with_slice(bytes: &[u8]) -> Self {
        let bytes_ptr = bytes.as_ptr() as *mut c_void;

        unsafe {
            let obj: id = msg_send![class!(NSData), dataWithBytes:bytes_ptr length:bytes.len()];
            NSData(Id::from_ptr(obj))
        }
    }

    /// Given a (presumably) `NSData`, wraps and retains it.
    pub fn retain(data: id) -> Self {
        NSData(unsafe {
            Id::from_ptr(data)
        })
    }

    /// If we're vended an NSData from a method (e.g, a push notification token) we might want to
    /// wrap it while we figure out what to do with it. This does that.
    pub fn from_retained(data: id) -> Self {
        NSData(unsafe {
            Id::from_retained_ptr(data)
        })
    }

    /// A helper method for determining if a given `NSObject` is an `NSData`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe {
            msg_send![obj, isKindOfClass:class!(NSData)]
        };

        to_bool(result)
    }

    /// Returns the length of the underlying `NSData` bytes.
    pub fn len(&self) -> usize {
        unsafe {
            let x: NSUInteger = msg_send![&*self.0, length];
            x as usize
        }
    }

    /// Returns a reference to the underlying bytes for the wrapped `NSData`.
    ///
    /// This, like `NSData::new()`, is cribbed from [objc-foundation](objc-foundation).
    ///
    /// [objc-foundation](https://crates.io/crates/objc-foundation)
    pub fn bytes(&self) -> &[u8] {
        let ptr: *const c_void = unsafe { msg_send![&*self.0, bytes] };

        // The bytes pointer may be null for length zero
        let (ptr, len) = if ptr.is_null() {
            (0x1 as *const u8, 0)
        } else {
            (ptr as *const u8, self.len())
        };

        unsafe {
            slice::from_raw_parts(ptr, len)
        }
    }

    /// Creates a new Vec, copies the NSData (safely, but quickly) bytes into that Vec, and
    /// consumes the NSData enabling it to release (provided nothing in Cocoa is using it).
    ///
    // A point of discussion: I think this is the safest way to handle it, however I could be
    // wrong - it's a bit defensive but I can't think of a way to reliably return an owned set of
    // this data without messing up the Objective-C side of things. Thankfully this isn't used too
    // often, but still... open to ideas.
    pub fn into_vec(self) -> Vec<u8> {
        let mut data = Vec::new();

        let bytes = self.bytes();
        data.resize(bytes.len(), 0);
        data.copy_from_slice(bytes);

        data
    }
}

impl From<NSData> for id {
    /// Consumes and returns the underlying `NSData`.
    fn from(mut data: NSData) -> Self {
        &mut *data.0
    }
}

impl Deref for NSData {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.0
    }
}

impl DerefMut for NSData {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.0
    }
}
