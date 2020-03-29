//! This example showcases setting up a basic application and window controller.
//! A Window Controller is backed by `NSWindowController`, and typically used in scenarios where
//! you might have documents (backed by `NSDocument`) that you're working with.
//!
//! If you're not using that, you can probably get by fine with a standard `NSWindow`.

use cacao::app::{App, AppDelegate};
use cacao::window::{Window, WindowConfig, WindowController, WindowDelegate};

struct BasicApp {
    window: WindowController<MyWindow>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.show();
    }
}

#[derive(Default)]
struct MyWindow;

impl WindowDelegate for MyWindow {
    fn did_load(&mut self, window: Window) {
        window.set_minimum_content_size(400., 400.);
        window.set_title("A Basic Window!?");
    }

    fn will_close(&self) {
        println!("Closing now!");
    }
}

fn main() {
    App::new("com.test.window-delegate", BasicApp {
        window: WindowController::with(WindowConfig::default(), MyWindow::default())
    }).run();
}
