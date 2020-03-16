//! This crate provides pieces necessary for interfacing with `AppKit` (`Cocoa`, on macOS). It
//! tries to do so in a way that, if you've done programming for the framework before (in Swift or
//! Objective-C), will feel familiar. This is tricky in Rust due to the ownership model, but some
//! creative coding and assumptions can get us pretty far.
//!
//! Note that this crate relies on the Objective-C runtime. Interfacing with the runtime _requires_
//! unsafe blocks; this crate handles those unsafe interactions for you, but by using this crate
//! you understand that usage of `unsafe` is a given and will be somewhat rampant for wrapped
//! controls. This does _not_ mean you can't assess, review, or question unsafe usage - just know
//! it's happening, and in large part it's not going away.
//!
//! It's best to look at this crate as a bridge to the future: you can write your own (safe) Rust
//! code, and have it intermix in the (existing, unsafe) world.
//!
//! This crate is also, currently, _very_ early stage and may have bugs. Your usage of it is at
//! your own risk. With that said, provided you follow the rules (regarding memory/ownership) it's
//! already fine for some apps. Check the README for more info!

pub use objc_id::ShareId;
pub use objc::runtime::Object;
pub use cocoa::base::id;

pub mod alert;
pub mod app;
pub mod button;
pub mod color;
pub mod collection_view;
pub mod constants;
pub mod dragdrop;
pub mod error;
pub mod events;
pub mod filesystem;
pub mod geometry;
pub mod layout;
pub mod menu;
pub mod networking;
pub mod notifications;
pub mod pasteboard;
pub mod printing;
pub mod toolbar;
pub mod utils;
pub mod view;
pub mod webview;
pub mod window;

// We re-export these so that they can be used without increasing build times.
pub use url;

pub mod prelude {
    pub use crate::app::{App, AppController, Dispatcher};
    
    pub use crate::layout::LayoutConstraint;

    pub use crate::menu::{Menu, MenuItem};
    pub use crate::notifications::{Notification, NotificationCenter, NotificationAuthOption};
    pub use crate::toolbar::{Toolbar, ToolbarController, ToolbarHandle};

    pub use crate::networking::URLRequest;

    pub use crate::window::{
        Window, WindowController, WindowHandle
    };

    pub use crate::webview::{
        WebView, WebViewConfig, WebViewController
    };

    pub use crate::view::{View, ViewHandle, ViewController};

    pub use appkit_derive::{
        WindowWrapper, ViewWrapper
    };
}
