//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::color::{Color, Theme};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::View;

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{App, AppDelegate};

struct BasicApp {
    window: Window<AppWindow>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new("", vec![
                MenuItem::Services,
                MenuItem::Separator,
                MenuItem::Hide,
                MenuItem::HideOthers,
                MenuItem::ShowAll,
                MenuItem::Separator,
                MenuItem::Quit,
            ]),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new("Window", vec![
                MenuItem::Minimize,
                MenuItem::Zoom,
                MenuItem::Separator,
                MenuItem::new("Bring All to Front"),
            ]),
        ]);

        App::activate();

        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct AppWindow {
    content: View,
    blue: View,
    red: View,
    green: View
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("AutoLayout Example");
        window.set_minimum_content_size(300., 300.);

        let dynamic = Color::dynamic(|style| match (style.theme, style.contrast) {
            (Theme::Dark, _) => Color::SystemGreen,
            _ => Color::SystemRed
        });

        self.blue.set_background_color(Color::SystemBlue);
        self.blue.layer.set_corner_radius(16.);
        self.content.add_subview(&self.blue);

        self.red.set_background_color(Color::SystemRed);
        self.content.add_subview(&self.red);

        self.green.set_background_color(dynamic);
        self.content.add_subview(&self.green);

        window.set_content_view(&self.content);

        LayoutConstraint::activate(&[
            self.blue.top.constraint_equal_to(&self.content.top).offset(46.),
            self.blue.leading.constraint_equal_to(&self.content.leading).offset(16.),
            self.blue.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
            self.blue.width.constraint_equal_to_constant(100.),
            self.red.top.constraint_equal_to(&self.content.top).offset(46.),
            self.red.leading.constraint_equal_to(&self.blue.trailing).offset(16.),
            self.red.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
            self.green.top.constraint_equal_to(&self.content.top).offset(46.),
            self.green.leading.constraint_equal_to(&self.red.trailing).offset(16.),
            self.green.trailing.constraint_equal_to(&self.content.trailing).offset(-16.),
            self.green.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
            self.green.width.constraint_equal_to_constant(100.)
        ]);
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with(WindowConfig::default(), AppWindow::default())
    })
    .run();
}
