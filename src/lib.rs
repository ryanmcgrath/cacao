//#![deny(missing_docs)]
//#![deny(missing_debug_implementations)]

// Copyright 2019+, the Cacao developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Dual-licensed under an MIT/MPL-2.0 license, see the LICENSE files in this repository.

//! # Cacao
//!
//! This library provides safe Rust bindings for `AppKit` on macOS and (eventually) `UIKit` on iOS. It
//! tries to do so in a way that, if you've done programming for the framework before (in Swift or
//! Objective-C), will feel familiar. This is tricky in Rust due to the ownership model, but some
//! creative coding and assumptions can get us pretty far.
//!
//! This library is currently _very_ early stage and may have bugs. Your usage of it is at
//! your own risk. With that said, provided you follow the rules (regarding memory/ownership) it's
//! already fine for some apps.
//!
//! _Note that this crate relies on the Objective-C runtime. Interfacing with the runtime *requires*
//! unsafe blocks; this crate handles those unsafe interactions for you and provides a safe wrapper, 
//! but by using this crate you understand that usage of `unsafe` is a given and will be somewhat 
//! rampant for wrapped controls. This does _not_ mean you can't assess, review, or question unsafe 
//! usage - just know it's happening, and in large part it's not going away._
//!
//! # Hello World
//!
//! ```rust
//! use cacao::macos::app::{App, AppDelegate};
//! use cacao::macos::window::Window;
//! 
//! #[derive(Default)]
//! struct BasicApp {
//!     window: Window
//! }
//!
//! impl AppDelegate for BasicApp {
//!     fn did_finish_launching(&self) {
//!        self.window.set_minimum_content_size(400., 400.);
//!        self.window.set_title("Hello World!");
//!        self.window.show();
//!     }
//! }
//!
//! fn main() {
//!     App::new("com.hello.world", BasicApp::default()).run();
//! }
//! ```
//!
//! ## Initialization
//! Due to the way that AppKit and UIKit programs typically work, you're encouraged to do the bulk
//! of your work starting from the `did_finish_launching()` method of your `AppDelegate`. This
//! ensures the application has had time to initialize and do any housekeeping necessary behind the
//! scenes.
//!
//! ## Optional Features
//!
//! The following are a list of [Cargo features][cargo-features] that can be enabled or disabled.
//!
//! - **cloudkit**: Links `CloudKit.framework` and provides some wrappers around CloudKit
//! functionality. Currently not feature complete.
//! - **user-notifications**: Links `UserNotifications.framework` and provides functionality for
//! emitting notifications on macOS and iOS. Note that this _requires_ your application be
//! code-signed, and will not work without it.
//! - **webview**: Links `WebKit.framework` and provides a `WebView` control backed by `WKWebView`.
//! - **webview-downloading**: Enables downloading files from the `WebView` via a private
//! interface. This is not an App-Store-safe feature, so be aware of that before enabling.
//!
//! [cargo-features]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section

pub use core_foundation;
pub use core_graphics;
pub use objc;
pub use url;

#[cfg(feature = "macos")]
pub mod macos;

pub mod button;

#[cfg(feature = "cloudkit")]
pub mod cloudkit;

pub mod color;
pub mod dragdrop;
pub mod error;
pub mod events;
pub mod defaults;
pub mod filesystem;
pub mod foundation;
pub mod geometry;
pub mod layout;
pub mod networking;
pub mod notification_center;
pub mod pasteboard;

#[cfg(feature = "user-notifications")]
pub mod user_notifications;

pub mod user_activity;
pub(crate) mod utils;

pub mod view;

#[cfg(feature = "webview")]
pub mod webview;
