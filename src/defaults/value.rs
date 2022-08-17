use std::collections::HashMap;

use crate::foundation::{id, NSData, NSMutableDictionary, NSNumber, NSString};

/// Represents a Value that can be stored or queried with `UserDefaults`.
///
/// In general, this wraps a few types that should hopefully work for most cases. Note that the
/// `Value` always owns whatever it holds - this is both for ergonomic considerations, as
/// well as contractual obligations with the underlying `NSUserDefaults` system.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Represents a Boolean value.
    Bool(bool),

    /// Represents a String value.
    String(String),

    /// Represents a Float (`f64`) value.
    Float(f64),

    /// Represents an Integer (`i64`) value.
    Integer(i64),

    /// Represents Data (bytes). You can use this to store arbitrary things that aren't supported
    /// above. You're responsible for moving things back and forth to the necessary types.
    Data(Vec<u8>),
}

impl Value {
    /// A handy initializer for `Value::String`.
    pub fn string<S: Into<String>>(value: S) -> Self {
        Value::String(value.into())
    }

    /// Returns `true` if the value is a boolean value. Returns `false` otherwise.
    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    /// If this is a Bool, it returns the associated bool. Returns `None` otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns `true` if the value is a string. Returns `false` otherwise.
    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    /// If this is a String, it returns a &str. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns `true` if the value is a float. Returns `false` otherwise.
    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    /// If this is a int, returns it (`i32`). Returns `None` otherwise.
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::Integer(i) => Some(*i as i32),
            _ => None,
        }
    }

    /// If this is a int, returns it (`i64`). Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i as i64),
            _ => None,
        }
    }

    /// Returns `true` if the value is a float. Returns `false` otherwise.
    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    /// If this is a float, returns it (`f32`). Returns `None` otherwise.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::Float(f) => Some(*f as f32),
            _ => None,
        }
    }

    /// If this is a float, returns it (`f64`). Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f as f64),
            _ => None,
        }
    }

    /// Returns `true` if the value is data. Returns `false` otherwise.
    pub fn is_data(&self) -> bool {
        match self {
            Value::Data(_) => true,
            _ => false,
        }
    }

    /// If this is data, returns it (`&[u8]`). If you need to own the underlying buffer, you can
    /// extract it yourself. Returns `None` if this is not Data.
    pub fn as_data(&self) -> Option<&[u8]> {
        match self {
            Value::Data(data) => Some(data),
            _ => None,
        }
    }
}

impl From<Value> for id {
    /// Shepherds `Value` types into `NSObject`s that can be stored in `NSUserDefaults`.
    // These currently work, but may not be exhaustive and should be looked over past the preview
    // period.
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => NSNumber::bool(b).into(),
            Value::String(s) => NSString::new(&s).into(),
            Value::Float(f) => NSNumber::float(f).into(),
            Value::Integer(i) => NSNumber::integer(i).into(),
            Value::Data(data) => NSData::new(data).into(),
        }
    }
}

impl<K> From<HashMap<K, Value>> for NSMutableDictionary
where
    K: AsRef<str>,
{
    /// Translates a `HashMap` of `Value`s into an `NSDictionary`.
    fn from(map: HashMap<K, Value>) -> Self {
        let mut dictionary = NSMutableDictionary::new();

        for (key, value) in map.into_iter() {
            let k = NSString::new(key.as_ref());
            dictionary.insert(k, value.into());
        }

        dictionary
    }
}
