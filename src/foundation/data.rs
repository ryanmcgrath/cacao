//! A wrapper for `NSData`.
//!
//! This more or less exists to try and wrap the specific APIs we use to interface with Cocoa. Note
//! that in general this is only concerned with bridging for arguments - i.e, we need an `NSData`
//! from a `Vec<u8>` to satisfy the Cocoa side of things. It's expected that all processing of data
//! happens solely on the Rust side of things before coming through here.
//!
//! tl;dr this is an intentionally limited API.

use std::mem;
use std::os::raw::c_void;
use std::slice;

use block::{Block, ConcreteBlock};

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, BOOL, YES, NO, NSUInteger};

/// Wrapper for a retained `NSData` object.
///
/// Supports constructing a new `NSData` from a `Vec<u8>`, wrapping and retaining an existing
/// pointer from the Objective-C side, and turning an `NSData` into a `Vec<u8>`.
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

    /// If we're vended an NSData from a method (e.g, a push notification token) we might want to
    /// wrap it while we figure out what to do with it. This does that.
    pub fn wrap(data: id) -> Self {
        NSData(unsafe {
            Id::from_ptr(data)
        })
    }

    /// A helper method for determining if a given `NSObject` is an `NSData`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe {
            msg_send![obj, isKindOfClass:class!(NSData)]
        };

        match result {
            YES => true,
            NO => false,
            _ => unreachable!()
        }
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

    /// Consumes and returns the underlying `NSData`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
