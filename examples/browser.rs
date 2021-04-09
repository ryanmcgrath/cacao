//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::webview::WebView;

use cacao::macos::{App, AppDelegate};
use cacao::macos::menu::{Menu, MenuItem};
use cacao::macos::window::{Window, WindowConfig, WindowDelegate};

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
                MenuItem::Quit
            ]),

            Menu::new("File", vec![
                MenuItem::CloseWindow
            ]),

            Menu::new("Edit", vec![
                MenuItem::Undo,
                MenuItem::Redo,
                MenuItem::Separator,
                MenuItem::Cut,
                MenuItem::Copy,
                MenuItem::Paste,
                MenuItem::Separator,
                MenuItem::SelectAll
            ]),

            // Sidebar option is 11.0+ only.
            Menu::new("View", vec![
                MenuItem::EnterFullScreen
            ]),

            Menu::new("Window", vec![
                MenuItem::Minimize,
                MenuItem::Zoom,
                MenuItem::Separator,
                MenuItem::new("Bring All to Front")
            ]),

            Menu::new("Help", vec![])
        ]);

        App::activate();
        self.window.show();
    }
}

#[derive(Default)]
struct AppWindow {
    content: WebView
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Browser Example");
        window.set_minimum_content_size(400., 400.);

        window.set_content_view(&self.content);

        self.content.load_url("https://www.duckduckgo.com/");
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with(WindowConfig::default(), AppWindow::default())
    }).run();
}
