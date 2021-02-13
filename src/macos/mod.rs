//! Mac-specific implementations.
//!
//! macOS is a much older system than iOS, and as a result has some... quirks, in addition to just
//! plain different APIs. It's tempting to want to find a common one and just implement that, but
//! unfortunately doing so erases a lot of control and finer points of the macOS platform.
//!
//! With that said, this framework makes attempts to make things mostly work as you'd expect them
//! to from the iOS-side of things, which means we wrap things like `NSView` and `NSTableView` and
//! so on to act like their iOS counterparts (we also layer-back everything by default, as it's
//! typically what you want).
//!
//! _However_, there are some specific things that just can't be wrapped well - for example,
//! `NSToolbar`. Yes, `UIToolbar` exists, but it's really not close to `NSToolbar` in functionality
//! at all. For controls like these, we surface them here - the goal is to enable you to write 90%
//! of your app as a cross platform codebase, with the initial 10% being scaffolding code for the
//! platform (e.g, NSApplication vs UIApplication lifecycle).

mod alert;
pub use alert::Alert;

mod app;
pub use app::*;

mod cursor;
pub use cursor::{Cursor, CursorType};

mod enums;
pub use enums::{FocusRingType};

mod event;
pub use event::*;

pub mod menu;
pub mod printing;
pub mod toolbar;
pub mod window;
