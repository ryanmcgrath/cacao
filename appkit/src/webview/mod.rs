//! A wrapper for WKWebview and associated configurations and properties.

pub mod action;

pub(crate) mod controller;
//pub(crate) mod process_pool;

pub mod traits;
pub use traits::{WebViewController};

pub mod config;
pub use config::{WebViewConfig, InjectAt};

pub mod webview;
pub use webview::WebView;
