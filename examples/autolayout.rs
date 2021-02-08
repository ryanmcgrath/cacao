//! This example showcases setting up a basic application and window, and setting up some views to
//! work with autolayout.

use cacao::color::rgb;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::View;

use cacao::macos::{App, AppDelegate};
use cacao::macos::window::{Window, WindowConfig, WindowDelegate};

struct BasicApp {
    window: Window<AppWindow>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::activate();
        self.window.show();
    }
}

#[derive(Default)]
struct AppWindow {
    content: View,
    blue: View,
    red: View,
    green: View,
    window: Window
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("AutoLayout Example");
        window.set_minimum_content_size(300., 300.);

        self.blue.set_background_color(rgb(105, 162, 176));
        self.content.add_subview(&self.blue);

        self.red.set_background_color(rgb(224, 82, 99));
        self.content.add_subview(&self.red);

        self.green.set_background_color(rgb(161, 192, 132));
        self.content.add_subview(&self.green);

        window.set_content_view(&self.content);

        LayoutConstraint::activate(&[
            self.blue.top.constraint_equal_to(&self.content.top).offset(16.),
            self.blue.leading.constraint_equal_to(&self.content.leading).offset(16.),
            self.blue.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
            self.blue.width.constraint_equal_to_constant(100.),

            self.red.top.constraint_equal_to(&self.content.top).offset(16.),
            self.red.leading.constraint_equal_to(&self.blue.trailing).offset(16.),
            self.red.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
            
            self.green.top.constraint_equal_to(&self.content.top).offset(16.),
            self.green.leading.constraint_equal_to(&self.red.trailing).offset(16.),
            self.green.trailing.constraint_equal_to(&self.content.trailing).offset(-16.),
            self.green.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
            self.green.width.constraint_equal_to_constant(100.),
        ]);
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with(WindowConfig::default(), AppWindow::default())
    }).run();
}
