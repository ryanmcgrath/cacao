//! Wraps `NSUserDefaults`, providing an interface to fetch and store small amounts of data.
//!
//! In general, this tries to take an approach popularized by `serde_json`'s `Value` struct. In
//! this case, `DefaultValue` handles wrapping types for insertion/retrieval, shepherding between
//! the Objective-C runtime and your Rust code.
//!
//! It currently supports a number of primitive types, as well as a generic `Data` type for custom
//! usage. Note that the `Data` type is stored internally as an `NSData` instance.
//!
//! Do not use this for storing sensitive data - you want the Keychain for that.
//!
//! In general, you should expect that some allocations are happening under the hood here, due to
//! the way the Objective-C runtime and Cocoa work. Where possible attempts are made to minimize
//! them, but in general... well, profile the rest of your code first, and don't call this stuff in
//! a loop.
//!
//! ## Example
//! ```rust
//! use std::collections::HashMap;
//! use cacao::defaults::{UserDefaults, DefaultValue};
//!
//! let mut defaults = UserDefaults::standard();
//!
//! defaults.register({
//!     let map = HashMap::new();
//!     map.insert("test", DefaultValue::string("value"));
//!     map
//! });
//!
//! // Ignore the unwrap() calls, it's a demo ;P
//! let value = defaults.get("test").unwrap().as_str().unwrap();
//! assert_eq!(value, "value");
//! ```

use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::unreachable;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::{id, nil, YES, BOOL, NSInteger, NSString, NSDictionary};

mod value;
pub use value::DefaultValue;

/// Wraps and provides methods for interacting with `NSUserDefaults`, which can be used for storing
/// pieces of information (preferences, or _defaults_) to persist across application launches.
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
    /// 
    /// _Note that if you're planning to share preferences across things (e.g, an app and an
    /// extension) you *probably* want to use `suite()` instead!_
    ///
    /// ```rust
    /// use cacao::defaults::UserDefaults;
    ///
    /// let defaults = UserDefaults::standard();
    /// 
    /// let _ = defaults.get("test");
    /// ```
    pub fn standard() -> Self {
        UserDefaults(unsafe {
            Id::from_ptr(msg_send![class!(NSUserDefaults), standardUserDefaults])
        })
    }

    /// Returns a user defaults instance for the given suite name. You typically use this to share
    /// preferences across apps and extensions.
    ///
    /// ```rust
    /// use cacao::defaults::UserDefaults;
    ///
    /// let defaults = UserDefaults::suite("com.myapp.shared");
    ///
    /// // This value would be shared between apps, extensions, and so on that are in this suite.
    /// let _ = defaults.get("test");
    /// ```
    pub fn suite(named: &str) -> Self {
        let name = NSString::new(named);

        UserDefaults(unsafe {
            let alloc: id = msg_send![class!(NSUserDefaults), alloc];
            Id::from_ptr(msg_send![alloc, initWithSuiteName:name.into_inner()])
        })
    }

    /// You can use this to register defaults at the beginning of your program. Note that these are
    /// just that - _defaults_. If a user has done something to cause an actual value to be set
    /// here, that value will be returned instead for that key.
    ///
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use cacao::defaults::{UserDefaults, DefaultValue};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// 
    /// defaults.register({
    ///     let mut map = HashMap::new();
    ///     map.insert("test", DefaultValue::Bool(true));
    ///     map
    /// });
    /// ```
    pub fn register<K: AsRef<str>>(&mut self, values: HashMap<K, DefaultValue>) {
        let dictionary = NSDictionary::from(values);
        
        unsafe {
            let _: () = msg_send![&*self.0, registerDefaults:dictionary.into_inner()];
        }
    }

    /// Inserts a value for the specified key. This synchronously updates the backing
    /// `NSUserDefaults` store, and asynchronously persists to the disk.
    ///
    /// ```rust
    /// use cacao::defaults::{UserDefaults, DefaultValue};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.insert("test", DefaultValue::Bool(true));
    /// ```
    pub fn insert<K: AsRef<str>>(&mut self, key: K, value: DefaultValue) {
        let key = NSString::new(key.as_ref());
        let value: id = (&value).into();

        unsafe {
            let _: () = msg_send![&*self.0, setObject:value forKey:key];
        }
    }
    
    /// Remove the default associated with the key. If the key doesn't exist, this is a noop.
    ///
    /// ```rust
    /// use cacao::defaults::{UserDefaults, DefaultValue};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.remove("test");
    /// ```
    pub fn remove<K: AsRef<str>>(&mut self, key: K) {
        let key = NSString::new(key.as_ref());

        unsafe {
            let _: () = msg_send![&*self.0, removeObjectForKey:key.into_inner()];
        }
    }

    /// Returns a `DefaultValue` for the given key, from which you can further extract the data you
    /// need. Note that this does a `nil` check and will return `None` in such cases, with the
    /// exception of `bool` values, where it will always return either `true` or `false`. This is
    /// due to the underlying storage engine used for `NSUserDefaults`.
    ///
    /// Note that this also returns owned values, not references. `NSUserDefaults` explicitly
    /// returns new immutable copies each time, so we're free to vend them as such.
    ///
    /// ```rust
    /// use cacao::defaults::{UserDefaults, DefaultValue};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.insert("test", DefaultValue::string("value"));
    ///
    /// let value = defaults.get("test").unwrap().as_str().unwrap();
    /// assert_eq!(value, "value");
    /// ```
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<DefaultValue> {
        let key = NSString::new(key.as_ref());

        let result: id = unsafe {
            msg_send![&*self.0, objectForKey:key.into_inner()]
        };

        if result == nil {
            return None;
        }

        let is_string: BOOL = unsafe { msg_send![result, isKindOfClass:class!(NSString)] };
        if is_string == YES {
            let s = NSString::wrap(result).to_str().to_string();
            return Some(DefaultValue::String(s));
        }

        // This works, but might not be the best approach. We basically need to inspect the
        // `NSNumber` returned and see what the wrapped encoding type is. `q` and `d` represent
        // `NSInteger` (platform specific) and `double` (f64) respectively, but conceivably we
        // might need others.
        let is_number: BOOL = unsafe { msg_send![result, isKindOfClass:class!(NSNumber)] };
        if is_number == YES {
            unsafe {
                let t: *const c_char = msg_send![result, objCType];
                let slice = CStr::from_ptr(t);

                if let Ok(code) = slice.to_str() {
                    println!("Code: {}", code);

                    if code == "q" {
                        let v: NSInteger = msg_send![result, integerValue];
                        return Some(DefaultValue::Integer(v as i64));
                    }

                    if code == "d" {
                        let v: f64 = msg_send![result, doubleValue];
                        return Some(DefaultValue::Float(v));
                    }
                }
            }
        }

        None
    }
}
