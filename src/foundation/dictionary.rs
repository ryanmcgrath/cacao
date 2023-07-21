use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, NSString};

use super::{NSArray, Retainable};

/// A wrapper for `NSMutableDictionary`.
#[derive(Debug)]
pub struct NSMutableDictionary(Id<Object>);

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
        NSMutableDictionary(unsafe { Id::from_ptr(msg_send![class!(NSMutableDictionary), new]) })
    }

    /// Consumes and returns the underlying `NSMutableDictionary`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }

    /// Inserts an object into the backing NSMutableDictionary. Corresponds to setObject:forKey:
    ///
    /// This intentionally requires `NSString` be allocated ahead of time.
    pub fn insert(&mut self, key: NSString, object: id) {
        unsafe {
            let _: () = msg_send![&*self.0, setObject:object forKey:&*key];
        }
    }

    /// Returns the value associated with a given key. Corresponds to objectForKey:
    pub fn get(&self, key: &str) -> Option<id> {
        let key = NSString::new(key);
        let id: id = unsafe { msg_send![self.0, objectForKey: key] };

        if !id.is_null() {
            Some(id)
        } else {
            None
        }
    }

    /// A new array containing the dictionary’s keys, or an empty array if the dictionary has no entries. Corresponds to allKeys
    ///
    /// **NOTE:** This only works with string keys
    pub fn keys(&self) -> Vec<String> {
        let keys = NSArray::retain(unsafe { msg_send![self.0, allKeys] });
        keys.iter().map(|s| NSString::retain(s).to_string()).collect()
    }

    /// Converts the dictionary into a hashmap, passing each item through a transform function.
    ///
    /// **NOTE:** This only works with string keys
    pub fn into_hashmap<T, F>(&self, item_transform: F) -> HashMap<String, T>
    where
        F: Fn(&String, id) -> T
    {
        let mut map = HashMap::new();

        let keys = self.keys();

        for key in keys {
            let item_id = self.get(&key);

            if let Some(item_id) = item_id {
                let item = item_transform(&key, item_id);

                map.insert(key, item);
            } else {
                // TODO: Should there be an assertion here for runtime failure?
                continue;
            }
        }

        map
    }

    /// Returns an iterator over the `NSMutableDictionary`.
    ///
    /// **NOTE:** This only works with string keys
    pub fn iter<'a>(&'a self) -> NSMutableDictionaryIterator<'a> {
        let keys = self.keys();

        NSMutableDictionaryIterator {
            next_index: 0,
            count: keys.len(),
            keys,
            dict: self
        }
    }
}

#[derive(Debug)]
pub struct NSMutableDictionaryIterator<'a> {
    next_index: usize,
    count: usize,
    keys: Vec<String>,

    dict: &'a NSMutableDictionary
}

impl Iterator for NSMutableDictionaryIterator<'_> {
    type Item = (String, id);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index < self.count {
            // This could be optimized to not clone
            let key = self.keys[self.next_index].clone();

            let value = self.dict.get(&key).expect("Could not find key");

            self.next_index += 1;

            Some((key, value))
        } else {
            None
        }
    }
}

impl Retainable for NSMutableDictionary {
    fn retain(object: id) -> Self {
        unsafe { NSMutableDictionary(Id::from_ptr(object)) }
    }

    fn from_retained(object: id) -> Self {
        unsafe { NSMutableDictionary(Id::from_retained_ptr(object)) }
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

impl From<&HashMap<String, String>> for NSMutableDictionary {
    fn from(value: &HashMap<String, String>) -> Self {
        let mut dictionary = Self::new();

        for (key, value) in value.iter() {
            let key = NSString::new(key);
            let mut value = NSString::new(value);

            dictionary.insert(key, &mut *value);
        }

        dictionary
    }
}
