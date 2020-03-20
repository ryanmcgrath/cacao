//! A wrapper for `NSLayoutConstraint`, enabling AutoLayout across views. This does a few things
//! that might seem weird, but are generally good and rely on the idea that this is all written
//! once and used often.
//!
//! Notably: there are 3 structs for wrapping layout constraints; in practice, you likely don't need to
//! care. This is because we want to detect at compile time invalid layout items - i.e, you should
//! not be able to attach a left-axis to a top-axis. In Rust this is a bit tricky, but by using
//! some `impl Trait`'s in the right places we can mostly hide this detail away.

pub mod traits;
pub use traits::Layout;

pub mod constraint;
pub use constraint::LayoutConstraint;

pub mod dimension;
pub use dimension::LayoutAnchorDimension;

pub mod horizontal;
pub use horizontal::LayoutAnchorX;

pub mod vertical;
pub use vertical::LayoutAnchorY;
