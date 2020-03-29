//! This module contains a wrapper for `NSToolbar`, one of the standard UI elements in a native
//! Cocoa application. To customize it and provide options, you should implement a
//! `ToolbarController` for your desired struct, and instantiate your `Toolbar` with it. For
//! example:
//!
//! ```
//! use cacao::prelude::*;
//!
//! #[derive(Default)]
//! struct WindowToolbar;
//!
//! impl ToolbarController for WindowToolbar {
//!     /* Your trait implementation here */
//! }
//!
//! ```
//!
//! And then, wherever your window is:
//!
//! ```
//! #[derive(Default)]
//! struct AppWindow {
//!     pub toolbar: Toolbar<WindowToolbar>
//! }
//!
//! impl WindowController for AppWindow {
//!     fn did_load(&mut self, window: WindowHandle) {
//!         window.set_toolbar(&self.toolbar);
//!     }
//! }
//! ```

pub(crate) mod class;

pub mod handle;
pub use handle::ToolbarHandle;

pub mod item;
pub use item::ToolbarItem;

pub mod traits;
pub use traits::ToolbarController;

pub mod toolbar;
pub use toolbar::Toolbar;

pub mod types;
pub use types::*;
