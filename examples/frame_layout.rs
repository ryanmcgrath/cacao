//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::color::Color;
use cacao::geometry::Rect;
use cacao::layout::Layout;
use cacao::view::View;

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{App, AppDelegate};

const CORNER_RADIUS: f64 = 16.;
const SPACING: f64 = 10.;
const TOP: f64 = 40.;
const WIDTH: f64 = 100.;
const HEIGHT: f64 = 100.;

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
        self.window.delegate.as_ref().unwrap().layout();
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

impl AppWindow {
    pub fn layout(&self) {
        self.blue.set_background_color(Color::SystemBlue);
        self.blue.set_frame(Rect {
            top: TOP,
            left: SPACING,
            width: WIDTH,
            height: HEIGHT
        });
        self.blue.layer.set_corner_radius(CORNER_RADIUS);
        self.content.add_subview(&self.blue);

        self.red.set_background_color(Color::SystemRed);
        self.red.set_frame(Rect {
            top: TOP,
            left: WIDTH + (SPACING * 2.),
            width: WIDTH,
            height: HEIGHT
        });
        self.red.layer.set_corner_radius(CORNER_RADIUS);
        self.content.add_subview(&self.red);

        self.green.set_background_color(Color::SystemGreen);
        self.green.set_frame(Rect {
            top: TOP,
            left: (WIDTH * 2.) + (SPACING * 3.),
            width: WIDTH,
            height: HEIGHT
        });
        self.green.layer.set_corner_radius(CORNER_RADIUS);
        self.content.add_subview(&self.green);
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Frame Layout Example");
        window.set_minimum_content_size(300., 300.);
        window.set_content_view(&self.content);
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with(WindowConfig::default(), AppWindow::default())
    })
    .run();
}
