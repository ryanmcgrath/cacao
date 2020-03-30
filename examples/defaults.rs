//! This tests the `defaults` module to ensure things behave as they should.

use cacao::macos::app::{App, AppDelegate};
use cacao::defaults::UserDefaults;

#[derive(Default)]
struct DefaultsTest;

impl AppDelegate for DefaultsTest {
    fn did_finish_launching(&self) {
        let mut defaults = UserDefaults::standard();

        match defaults.get_string("LOL") {
            Some(s) => { 
                println!("Retrieved {}", s);
            },

            None => { 
                defaults.set_string("LOL", "laugh");
                println!("Run this again to get a laugh");
            }
        }

        App::terminate();
    }
}

fn main() {
    App::new("com.cacao.defaults-test", DefaultsTest::default()).run();
}
