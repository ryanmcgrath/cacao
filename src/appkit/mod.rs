//! This module implements the core components necessary for making a well-formed macOS
//! application. These components are ones that are uniquely macOS-specific, and don't have a true
//! equivalent on iOS and tvOS as the interaction patterns are significantly different.
//!
//! The coverage here is not exhaustive, but should be sufficient enough for relatively complex
//! applications. For examples, check the `examples` folder in the repository.

mod alert;
pub use alert::Alert;

mod animation;
pub use animation::AnimationContext;

mod app;
pub use app::*;

mod cursor;
pub use cursor::{Cursor, CursorType};

mod enums;
pub use enums::FocusRingType;

mod event;
pub use event::*;

pub mod menu;
pub mod printing;
pub mod toolbar;
pub mod window;

pub mod segmentedcontrol;
