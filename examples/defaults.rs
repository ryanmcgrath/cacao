//! This tests the `defaults` module to ensure things behave as they should.

use std::collections::HashMap;

use cacao::macos::app::{App, AppDelegate};
use cacao::defaults::{UserDefaults, DefaultValue};

#[derive(Default)]
struct DefaultsTest;

impl AppDelegate for DefaultsTest {
    fn did_finish_launching(&self) {
        let mut defaults = UserDefaults::standard();

        defaults.register({
            let mut map = HashMap::new();
            map.insert("LOL", DefaultValue::string("laugh"));
            map.insert("X", DefaultValue::Integer(1));
            map.insert("X2", DefaultValue::Float(1.0));
            map.insert("BOOL", DefaultValue::bool(true));
            map
        });

        println!("Retrieved LOL: {:?}", defaults.get("LOL"));
        println!("Retrieved LOL: {:?}", defaults.get("X"));
        println!("Retrieved LOL: {:?}", defaults.get("X2"));

        App::terminate();
    }
}

fn main() {
    App::new("com.cacao.defaults-test", DefaultsTest::default()).run();
}
