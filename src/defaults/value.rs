//! 

use std::collections::HashMap;

use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, YES, NO, nil, NSInteger, NSDictionary, NSString};

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultValue {
    Bool(bool),
    String(String),
    Float(f64),
    Integer(i64)
}

impl DefaultValue {
    /// A handy initializer for `DefaultValue::Bool`.
    pub fn bool(value: bool) -> Self {
        DefaultValue::Bool(value)
    }

    /// A handy initializer for `DefaultValue::String`;
    pub fn string<S: Into<String>>(value: S) -> Self {
        DefaultValue::String(value.into())
    }
    
    /// Returns `true` if the value is a boolean value. Returns `false` otherwise.
    pub fn is_boolean(&self) -> bool {
        match self {
            DefaultValue::Bool(_) => true,
            _ => false
        }
    }

    /// If this is a Bool, it returns the associated bool. Returns `None` otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DefaultValue::Bool(v) => Some(*v),
            _ => None
        }
    }
    
    /// Returns `true` if the value is a string. Returns `false` otherwise.
    pub fn is_string(&self) -> bool {
        match self {
            DefaultValue::String(_) => true,
            _ => false
        }
    }

    /// If this is a String, it returns a &str. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DefaultValue::String(s) => Some(s),
            _ => None
        }
    }
    
    /// Returns `true` if the value is a float. Returns `false` otherwise.
    pub fn is_integer(&self) -> bool {
        match self {
            DefaultValue::Integer(_) => true,
            _ => false
        }
    }

    /// If this is a int, returns it (`i32`). Returns `None` otherwise.
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            DefaultValue::Integer(i) => Some(*i as i32),
            _ => None
        }
    }

    /// If this is a int, returns it (`i64`). Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DefaultValue::Integer(i) => Some(*i as i64),
            _ => None
        }
    }

    /// Returns `true` if the value is a float. Returns `false` otherwise.
    pub fn is_float(&self) -> bool {
        match self {
            DefaultValue::Float(_) => true,
            _ => false
        }
    }

    /// If this is a float, returns it (`f32`). Returns `None` otherwise.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            DefaultValue::Float(f) => Some(*f as f32),
            _ => None
        }
    }

    /// If this is a float, returns it (`f64`). Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DefaultValue::Float(f) => Some(*f as f64),
            _ => None
        }
    }
}

impl From<&DefaultValue> for id {
    /// Shepherds `DefaultValue` types into `NSObject`s that can be stored in `NSUserDefaults`.
    // These currently work, but may not be exhaustive and should be looked over past the preview
    // period.
    fn from(value: &DefaultValue) -> Self {
        unsafe {
            match value {
                DefaultValue::Bool(b) => msg_send![class!(NSNumber), numberWithBool:match b {
                    true => YES,
                    false => NO
                }],

                DefaultValue::String(s) => NSString::new(&s).into_inner(),
                DefaultValue::Float(f) => msg_send![class!(NSNumber), numberWithDouble:*f],
                DefaultValue::Integer(i) => msg_send![class!(NSNumber), numberWithInteger:*i as NSInteger]
            }
        }
    }
}

impl<K> From<HashMap<K, DefaultValue>> for NSDictionary
where
    K: AsRef<str>
{
    /// Translates a `HashMap` of `DefaultValue`s into an `NSDictionary`.
    fn from(map: HashMap<K, DefaultValue>) -> Self {
        NSDictionary(unsafe {
            let dictionary: id = msg_send![class!(NSMutableDictionary), new];

            for (key, value) in map.iter() {
                let k = NSString::new(key.as_ref()); 
                let v: id = value.into();
                let _: () = msg_send![dictionary, setObject:v forKey:k];
            }

            Id::from_ptr(dictionary)
        })
    }
}
