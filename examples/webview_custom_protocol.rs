//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::webview::{WebView, WebViewConfig, WebViewDelegate};

use cacao::macos::{App, AppDelegate};
use cacao::macos::menu::{Menu, MenuItem};
use cacao::macos::toolbar::Toolbar;
use cacao::macos::window::{Window, WindowConfig, WindowDelegate, WindowToolbarStyle};

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
pub struct WebViewInstance;

impl WebViewDelegate for WebViewInstance {
    fn on_custom_protocol_request(&self, path: &str) -> Option<Vec<u8>> {
        let requested_asset_path = path.replace("cacao://", "");

        let index_html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
            <h1>Welcome 🍫</h1>
            <a href="/hello.html">Link</a>
            </body>
        </html>"#;

        let link_html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
            <h1>Hello!</h1>
            <a href="/index.html">Back home</a>
            </body>
        </html>"#;

        return match requested_asset_path.as_str() {
            "/hello.html" => Some(link_html.as_bytes().into()),
            _ => Some(index_html.as_bytes().into()),
        }
    }
}

struct AppWindow {
    content: WebView<WebViewInstance>
}

impl AppWindow {
    pub fn new() -> Self {
        let mut webview_config = WebViewConfig::default();

        // register the protocol in the webview
        webview_config.add_custom_protocol("cacao");

        AppWindow {
            content: WebView::with(webview_config, WebViewInstance::default())
        }
    }

    pub fn load_url(&self, url: &str) {
        self.content.load_url(url);
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Browser Example");
        window.set_autosave_name("CacaoBrowserExample");
        window.set_minimum_content_size(400., 400.);

        window.set_content_view(&self.content);

        // load custom protocol
        self.load_url("cacao://");
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with(WindowConfig::default(), AppWindow::new())
    }).run();
}
