//! The prelude imports a large amount of useful widgets and traits. You're of course free to
//! thread imports yourself, but for many smaller applications this module can be quite a time
//! saver.

pub use crate::app::{App, traits::AppDelegate};

pub use crate::dispatcher::Dispatcher;
    
pub use crate::layout::LayoutConstraint;

pub use crate::menu::{Menu, MenuItem};

#[cfg(feature = "user-notifications")]
pub use crate::notifications::{Notification, NotificationCenter, NotificationAuthOption};

pub use crate::toolbar::{Toolbar, ToolbarController, ToolbarHandle};

pub use crate::networking::URLRequest;

pub use crate::window::{
    Window, config::WindowConfig, traits::WindowDelegate
};

#[cfg(feature = "webview")]
pub use crate::webview::{
    WebView, WebViewConfig, WebViewDelegate
};

pub use crate::view::{View, traits::ViewDelegate};
