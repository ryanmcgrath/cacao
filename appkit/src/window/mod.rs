//! Implements wrappers and traits for `NSWindowController` and associated types. 

pub mod traits;
pub use traits::WindowController;

mod controller;

pub mod config;
pub use config::{WindowConfig, WindowStyle};

pub mod handle;
pub use handle::WindowHandle;

pub mod window;
pub use window::{Window, WindowTitleVisibility};
