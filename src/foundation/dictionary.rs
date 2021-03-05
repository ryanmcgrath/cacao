use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, NSString};

/// A wrapper for `NSMutableDictionary`.
#[derive(Debug)]
pub struct NSMutableDictionary(pub Id<Object>);

impl Default for NSMutableDictionary {
    /// Returns a blank NSMutableDictionary.
    fn default() -> Self {
        NSMutableDictionary::new()
    }
}

impl NSMutableDictionary {
    /// Constructs an `NSMutableDictionary` and retains it.
    ///
    /// Why mutable? It's just easier for working with it, as they're (mostly) interchangeable when
    /// passed around in Objective-C. We guard against mutation on our side using the standard Rust
    /// object model. You can, of course, bypass it and `msg_send![]` yourself, but it'd require an
    /// `unsafe {}` block... so you'll know you're in special territory then.
    pub fn new() -> Self {
        NSMutableDictionary(unsafe {
            Id::from_ptr(msg_send![class!(NSMutableDictionary), new])
        })
    }

    /// Inserts an object into the backing NSMutablyDictionary.
    ///
    /// This intentionally requires `NSString` be allocated ahead of time.
    pub fn insert(&mut self, key: NSString, object: id) {
        unsafe {
            let _: () = msg_send![&*self.0, setObject:object forKey:&*key];
        }
    }

    /// Consumes and returns the underlying `NSMutableDictionary`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}

impl Deref for NSMutableDictionary {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.0
    }
}

impl DerefMut for NSMutableDictionary {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.0
    }
}
