use std::collections::HashMap;

use cacao::defaults::{UserDefaults, Value};

const EXAMPLE: &'static str = "exampleSetting";

/// A very basic wrapper around UserDefaults. If I wind up implementing Serde support for
/// UserDefaults, then much of this could be removed or simplified - but I'm not sold on that yet,
/// so this exists for now.
#[derive(Debug)]
pub struct Defaults;

impl Defaults {
    /// Registers the default settings for our application. Note that just because this is run at
    /// application start does _not_ mean it's always the defaults; this is effectively "on first
    /// run, set these defaults". Updates will persist and overwrite these accordingly.
    pub fn register() {
        let mut defaults = UserDefaults::standard();

        defaults.register({
            let mut map = HashMap::new();
            map.insert(EXAMPLE, Value::Bool(true));
            map
        });
    }

    /// Toggles the example setting.
    pub fn toggle_should_whatever() {
        toggle_bool(EXAMPLE);
    }

    /// Returns whether the example setting is currently true or false.
    pub fn should_whatever() -> bool {
        load_bool(EXAMPLE)
    }
}

/// A helper method for toggling a boolean value held at the specified key. If the value cannot
/// be pulled as a bool, then this panics.
///
/// Note that choosing to panic here is a personal design decision; this is core functionality that
/// should work, and I'd rather it crash with a meaningful message rather than `unwrap()` issues.
fn toggle_bool(key: &str) {
    let mut defaults = UserDefaults::standard();
    
    if let Some(value) = defaults.get(key) {
        if let Some(value) = value.as_bool() {
            defaults.insert(key, Value::Bool(!value));
            return;
        }
        
        panic!("Attempting to toggle a boolean value for {}, but it's not a boolean.", key);
    }

    panic!("Attempting to toggle a boolean value for {}, but this key does not exist.", key);
}

/// A helper method for loading a boolean value held at the specified key. If the value cannot
/// be pulled as a bool, then this panics.
///
/// Note that choosing to panic here is a personal design decision; this is core functionality that
/// should work, and I'd rather it crash with a meaningful message rather than `unwrap()` issues.
fn load_bool(key: &str) -> bool {
    let defaults = UserDefaults::standard();
    
    if let Some(value) = defaults.get(key) {
        if let Some(value) = value.as_bool() {
            return value;
        }
        
        panic!("Attempting to load a boolean value for {}, but it's not a boolean.", key);
    }

    panic!("Attempting to load a boolean value for {}, but this key does not exist.", key);
}
