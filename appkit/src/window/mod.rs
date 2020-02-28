//! Implements wrappers and traits for `NSWindowController` and associated types. 

pub mod traits;
pub use traits::{WindowController, WindowWrapper};

mod controller;

pub mod config;
pub use config::{WindowConfig, WindowStyle};

pub mod window;
pub use window::{Window, WindowTitleVisibility};
