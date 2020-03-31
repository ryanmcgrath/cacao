//! This tests the `defaults` module to ensure things behave as they should.

use std::collections::HashMap;

use cacao::macos::app::{App, AppDelegate};
use cacao::defaults::{UserDefaults, Value};
use cacao::foundation::NSData;

#[derive(Default)]
struct DefaultsTest;

impl AppDelegate for DefaultsTest {
    fn did_finish_launching(&self) {
        let mut defaults = UserDefaults::standard();

        defaults.register({
            let mut map = HashMap::new();
            //map.insert("LOL", Value::string("laugh"));
            //map.insert("X", Value::Integer(1));
            //map.insert("X2", Value::Float(1.0));
            map.insert("BOOL", Value::bool(true));

            println!("Test equivalency:");
            let s = "BYTES TEST".to_string().into_bytes();
            println!("    {:?}", s);
            let x = NSData::new(s);
            println!("    {:?}", x.bytes());
            
            let s2 = "BYTES TEST".to_string().into_bytes();
            map.insert("BYTES", Value::Data(s2));
            
            map
        });

        //println!("Retrieved LOL: {:?}", defaults.get("LOL"));
        //println!("Retrieved LOL: {:?}", defaults.get("X"));
        //println!("Retrieved LOL: {:?}", defaults.get("X2"));

        let bytes = defaults.get("BYTES").unwrap();
        println!("Bytes: {:?}", bytes);
        let data = match std::str::from_utf8(bytes.as_data().unwrap()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting bytes {}", e);
                "Error converting bytes"
            }
        };

        println!("Retrieved Bytes: {}", data);
        

        App::terminate();
    }
}

fn main() {
    App::new("com.cacao.defaults-test", DefaultsTest::default()).run();
}
