//! This example showcases setting up a basic application and window.

use cacao::macos::app::{App, AppDelegate};
use cacao::macos::window::Window;

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("A Basic Window");
        self.window.show();
    }
}

fn main() {
    App::new("com.test.window", BasicApp::default()).run();
}
