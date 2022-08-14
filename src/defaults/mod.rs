//! Wraps `NSUserDefaults`, providing an interface to fetch and store small amounts of data.
//!
//! In general, this tries to take an approach popularized by `serde_json`'s `Value` struct. In
//! this case, `Value` handles wrapping types for insertion/retrieval, shepherding between
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
//! ```rust,no_run
//! use std::collections::HashMap;
//! use cacao::defaults::{UserDefaults, Value};
//!
//! let mut defaults = UserDefaults::standard();
//!
//! defaults.register({
//!     let mut map = HashMap::new();
//!     map.insert("test", Value::string("value"));
//!     map
//! });
//!
//! let value = defaults.get("test").unwrap();
//! let value = value.as_str().unwrap();
//! assert_eq!(value, "value");
//! ```

use std::collections::HashMap;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, nil, to_bool, NSData, NSMutableDictionary, NSNumber, NSString, BOOL, NO, YES};

mod value;
pub use value::Value;

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
        UserDefaults(unsafe { Id::from_ptr(msg_send![class!(NSUserDefaults), standardUserDefaults]) })
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
            Id::from_ptr(msg_send![alloc, initWithSuiteName:&*name])
        })
    }

    /// You can use this to register defaults at the beginning of your program. Note that these are
    /// just that - _defaults_. If a user has done something to cause an actual value to be set
    /// here, that value will be returned instead for that key.
    ///
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use cacao::defaults::{UserDefaults, Value};
    ///
    /// let mut defaults = UserDefaults::standard();
    ///
    /// defaults.register({
    ///     let mut map = HashMap::new();
    ///     map.insert("test", Value::Bool(true));
    ///     map
    /// });
    /// ```
    pub fn register<K: AsRef<str>>(&mut self, values: HashMap<K, Value>) {
        let dictionary = NSMutableDictionary::from(values);

        unsafe {
            let _: () = msg_send![&*self.0, registerDefaults:&*dictionary];
        }
    }

    /// Inserts a value for the specified key. This synchronously updates the backing
    /// `NSUserDefaults` store, and asynchronously persists to the disk.
    ///
    /// ```rust
    /// use cacao::defaults::{UserDefaults, Value};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.insert("test", Value::Bool(true));
    /// ```
    pub fn insert<K: AsRef<str>>(&mut self, key: K, value: Value) {
        let key = NSString::new(key.as_ref());
        let value: id = value.into();

        unsafe {
            let _: () = msg_send![&*self.0, setObject:value forKey:key];
        }
    }

    /// Remove the default associated with the key. If the key doesn't exist, this is a noop.
    ///
    /// ```rust
    /// use cacao::defaults::{UserDefaults, Value};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.remove("test");
    /// ```
    pub fn remove<K: AsRef<str>>(&mut self, key: K) {
        let key = NSString::new(key.as_ref());

        unsafe {
            let _: () = msg_send![&*self.0, removeObjectForKey:&*key];
        }
    }

    /// Returns a `Value` for the given key, from which you can further extract the data you
    /// need. Note that this does a `nil` check and will return `None` in such cases, with the
    /// exception of `bool` values, where it will always return either `true` or `false`. This is
    /// due to the underlying storage engine used for `NSUserDefaults`.
    ///
    /// Note that this also returns owned values, not references. `NSUserDefaults` explicitly
    /// returns new immutable copies each time, so we're free to vend them as such.
    ///
    /// ```rust,no_run
    /// use cacao::defaults::{UserDefaults, Value};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.insert("test", Value::string("value"));
    ///
    /// let value = defaults.get("test").unwrap();
    /// let value = value.as_str().unwrap();
    /// assert_eq!(value, "value");
    /// ```
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<Value> {
        let key = NSString::new(key.as_ref());

        let result: id = unsafe { msg_send![&*self.0, objectForKey:&*key] };

        if result == nil {
            return None;
        }

        if NSData::is(result) {
            let data = NSData::retain(result);
            return Some(Value::Data(data.into_vec()));
        }

        if NSString::is(result) {
            let s = NSString::retain(result).to_string();
            return Some(Value::String(s));
        }

        // This works, but might not be the best approach. We basically need to inspect the
        // `NSNumber` returned and see what the wrapped encoding type is. `q` and `d` represent
        // `NSInteger` (platform specific) and `double` (f64) respectively, but conceivably we
        // might need others.
        //
        // BOOL returns as "c", which... something makes me feel weird there, but testing it seems
        // reliable.
        //
        // For context: https://nshipster.com/type-encodings/
        if NSNumber::is(result) {
            let number = NSNumber::wrap(result);

            return match number.objc_type() {
                "c" => Some(Value::Bool(number.as_bool())),
                "d" => Some(Value::Float(number.as_f64())),
                "q" => Some(Value::Integer(number.as_i64())),

                _x => {
                    // Debugging code that should be removed at some point.
                    #[cfg(debug_assertions)]
                    println!("Unexpected code type found: {}", _x);

                    None
                }
            };
        }

        None
    }

    /// Returns a boolean value if the object stored for the specified key is managed by an
    /// administrator. This is rarely used - mostly in managed environments, e.g a classroom.
    ///
    /// For managed keys, the application should disable any user interface that allows the
    /// user to modify the value of key.
    ///
    /// ```rust
    /// use cacao::defaults::{UserDefaults, Value};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.insert("test", Value::string("value"));
    ///
    /// let value = defaults.is_forced_for_key("test");
    /// assert_eq!(value, false);
    /// ```
    pub fn is_forced_for_key<K: AsRef<str>>(&self, key: K) -> bool {
        let result: BOOL = unsafe {
            let key = NSString::new(key.as_ref());
            msg_send![&*self.0, objectIsForcedForKey:&*key]
        };

        to_bool(result)
    }

    /// Blocks for any asynchronous updates to the defaults database and returns.
    ///
    /// This method is legacy, likely unnecessary and shouldn't be used unless you know exactly why
    /// you need it... and even then, you should double check it.
    /// ```rust
    /// use cacao::defaults::{UserDefaults, Value};
    ///
    /// let mut defaults = UserDefaults::standard();
    /// defaults.insert("test", Value::string("value"));
    /// defaults.synchronize();
    /// ```
    pub fn synchronize(&self) {
        unsafe {
            let _: () = msg_send![&*self.0, synchronize];
        }
    }
}
