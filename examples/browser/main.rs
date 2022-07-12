//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::notification_center::Dispatcher;
use cacao::webview::{WebView, WebViewConfig, WebViewDelegate};

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::toolbar::Toolbar;
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate, WindowToolbarStyle};

mod toolbar;
use toolbar::BrowserToolbar;

#[derive(Debug)]
pub enum Action {
    Back,
    Forwards,
    Load(String)
}

impl Action {
    pub fn dispatch(self) {
        App::<BasicApp, Self>::dispatch_main(self);
    }
}

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

impl Dispatcher for BasicApp {
    type Message = Action;

    fn on_ui_message(&self, message: Self::Message) {
        let window = self.window.delegate.as_ref().unwrap();
        let webview = &window.content;

        match message {
            Action::Back => { webview.go_back(); },
            Action::Forwards => { webview.go_forward(); },
            Action::Load(url) => { window.load_url(&url); } 
        }
    }
}

#[derive(Default)]
pub struct WebViewInstance;

impl WebViewDelegate for WebViewInstance {}

struct AppWindow {
    toolbar: Toolbar<BrowserToolbar>,
    content: WebView<WebViewInstance>
}

impl AppWindow {
    pub fn new() -> Self {
        AppWindow {
            toolbar: Toolbar::new("com.example.BrowserToolbar", BrowserToolbar::new()),
            content: WebView::with(WebViewConfig::default(), WebViewInstance::default())
        }
    }

    pub fn load_url(&self, url: &str) {
        self.toolbar.delegate.as_ref().unwrap().set_url(url);
        self.content.load_url(url);
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Browser Example");
        window.set_autosave_name("CacaoBrowserExample");
        window.set_minimum_content_size(400., 400.);

        window.set_toolbar(&self.toolbar);
        window.set_content_view(&self.content);

        self.load_url("https://www.duckduckgo.com/");
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with({
            let mut config = WindowConfig::default();

            // This flag is necessary for Big Sur to use the correct toolbar style.
            config.toolbar_style = WindowToolbarStyle::Expanded;

            config
        }, AppWindow::new())
    }).run();
}
