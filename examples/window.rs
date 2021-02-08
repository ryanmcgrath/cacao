//! This example showcases setting up a basic application and window.

use cacao::macos::{App, AppDelegate};
use cacao::macos::window::Window;

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::activate();

        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("A Basic Window");
        self.window.show();
    }
}

fn main() {
    App::new("com.test.window", BasicApp::default()).run();
}
