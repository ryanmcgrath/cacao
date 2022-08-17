//! This example showcases setting up a basic application and window.

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};

#[derive(Default)]
struct BasicApp {
    window: Window,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new(
                "",
                vec![
                    MenuItem::Services,
                    MenuItem::Separator,
                    MenuItem::Hide,
                    MenuItem::HideOthers,
                    MenuItem::ShowAll,
                    MenuItem::Separator,
                    MenuItem::Quit,
                ],
            ),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new(
                "Window",
                vec![
                    MenuItem::Minimize,
                    MenuItem::Zoom,
                    MenuItem::Separator,
                    MenuItem::new("Bring All to Front"),
                ],
            ),
        ]);

        App::activate();

        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("A Basic Window");
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

fn main() {
    App::new("com.test.window", BasicApp::default()).run();
}
