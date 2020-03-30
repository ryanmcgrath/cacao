//! Wraps `NSUserDefaults`, providing an interface to store and query small amounts of data.
//!
//! It may seem a bit verbose at points, but it aims to implement everything on the Objective-C
//! side as closely as possible.

use std::unreachable;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, nil, YES, NO, BOOL, NSString};

/// Wraps and provides methods for interacting with `NSUserDefaults`, which can be used for storing
/// pieces of information (preferences, or _defaults_) to persist across application restores.
///
/// This should not be used for sensitive data - use the Keychain for that.
#[derive(Debug)]
pub struct UserDefaults(pub Id<Object>);

impl Default for UserDefaults {
    /// Equivalent to calling `UserDefaults::standard()`.
    fn default() -> Self {
        UserDefaults::standard()
    }
}

impl UserDefaults {
    /// Returns the `standardUserDefaults`, which is... exactly what it sounds like.
    pub fn standard() -> Self {
        UserDefaults(unsafe {
            Id::from_ptr(msg_send![class!(NSUserDefaults), standardUserDefaults])
        })
    }

    /// Returns a new user defaults to work with. You probably don't want this, and either want
    /// `suite()` or `standard()`.
    pub fn new() -> Self {
        UserDefaults(unsafe {
            let alloc: id = msg_send![class!(NSUserDefaults), alloc];
            Id::from_ptr(msg_send![alloc, init])
        })
    }

    /// Returns a user defaults instance for the given suite name. You typically use this to share
    /// preferences across apps and extensions.
    pub fn suite(named: &str) -> Self {
        let name = NSString::new(named);

        UserDefaults(unsafe {
            let alloc: id = msg_send![class!(NSUserDefaults), alloc];
            Id::from_ptr(msg_send![alloc, initWithSuiteName:name.into_inner()])
        })
    }

    /// Remove the default associated with the key. If the key doesn't exist, this is a noop.
    pub fn remove(&mut self, key: &str) {
        let key = NSString::new(key);

        unsafe {
            let _: () = msg_send![&*self.0, removeObjectForKey:key.into_inner()];
        }
    }

    /// Returns a bool for the given key. If the key doesn't exist, it returns `false`.
    ///
    /// Note that behind the scenes, this will coerce certain "truthy" and "falsy" values - this is
    /// done on the system side, and is not something that can be changed.
    ///
    /// e.g:
    /// `"true"`, `"YES"`, `"1"`, `1`, `1.0` will become `true`
    /// `"false"`, `"NO"`, `"0"`, `0`, `0.0` will become `false`
    pub fn get_bool(&self, key: &str) -> bool {
        let key = NSString::new(key);

        let result: BOOL = unsafe {
            msg_send![&*self.0, boolForKey:key.into_inner()]
        };

        match result {
            YES => true,
            NO => false,
            _ => unreachable!()
        }
    }
    
    /// Sets the bool for the given key to the specified value.
    pub fn set_bool(&mut self, key: &str, value: bool) {
        let key = NSString::new(key);

        unsafe {
            let _: () = msg_send![&*self.0, setBool:match value {
                true => YES,
                false => NO
            } forKey:key];
        }
    }

    /// Returns the given String if it exists, mapping Objective-C's `nil` to `None`.
    pub fn get_string(&self, key: &str) -> Option<String> {
        let key = NSString::new(key);

        let result: id = unsafe {
            msg_send![&*self.0, stringForKey:key.into_inner()]
        };

        if result == nil {
            None
        } else {
            Some(NSString::wrap(result).to_str().to_string())
        }
    }

    /// Sets the string for the given key to the specified value.
    pub fn set_string(&mut self, key: &str, value: &str) {
        let key = NSString::new(key);
        let value = NSString::new(value);

        unsafe {
            let _: () = msg_send![&*self.0, setObject:value.into_inner() forKey:key.into_inner()];
        }
    }
}
