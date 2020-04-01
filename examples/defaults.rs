//! This tests the `defaults` module to ensure things behave as they should.

use std::collections::HashMap;

use cacao::macos::{App, AppDelegate};
use cacao::defaults::{UserDefaults, Value};

#[derive(Default)]
struct DefaultsTest;

impl AppDelegate for DefaultsTest {
    fn did_finish_launching(&self) {
        let mut defaults = UserDefaults::standard();

        defaults.register({
            let mut map = HashMap::new();
            map.insert("testbool", Value::Bool(true));
            map.insert("testint", Value::Integer(42));
            map.insert("testfloat", Value::Float(42.));
            map.insert("teststring", Value::string("Testing"));

            let bytes = "BYTES TEST".to_string().into_bytes();
            map.insert("testdata", Value::Data(bytes));

            map
        });

        let testbool = defaults.get("testbool").unwrap().as_bool().unwrap();
        assert_eq!(testbool, true);

        let testint = defaults.get("testint").unwrap().as_i64().unwrap();
        assert_eq!(testint, 42);

        let testfloat = defaults.get("testfloat").unwrap().as_f64().unwrap();
        assert_eq!(testfloat, 42.);

        let teststring = defaults.get("teststring").unwrap();
        assert_eq!(teststring.as_str().unwrap(), "Testing");
        
        let bytes = defaults.get("testdata").unwrap();
        let s = match std::str::from_utf8(bytes.as_data().unwrap()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting bytes {}", e);
                "Error converting bytes"
            }
        };

        assert_eq!(s, "BYTES TEST");
        println!("All UserDefaults tests pass");

        App::terminate();
    }
}

fn main() {
    App::new("com.cacao.defaults-test", DefaultsTest::default()).run();
}
