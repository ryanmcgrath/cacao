//! This module contains traits and helpers for layout. By default, standard frame-based layouts 
//! are supported via the `Layout` trait, which all widgets implement. If you opt in to the
//! `AutoLayout` feature, each widget will default to using AutoLayout, which can be beneficial in
//! more complicated views that need to deal with differing screen sizes.

pub mod traits;
pub use traits::Layout;

#[cfg(feature = "autolayout")]
pub mod attributes;

#[cfg(feature = "autolayout")]
pub use attributes::*;

#[cfg(feature = "autolayout")]
pub mod constraint;

#[cfg(feature = "autolayout")]
pub use constraint::LayoutConstraint;

#[cfg(feature = "autolayout")]
pub mod dimension;

#[cfg(feature = "autolayout")]
pub use dimension::LayoutAnchorDimension;

#[cfg(feature = "autolayout")]
pub mod horizontal;

#[cfg(feature = "autolayout")]
pub use horizontal::LayoutAnchorX;

#[cfg(feature = "autolayout")]
pub mod vertical;

#[cfg(feature = "autolayout")]
pub use vertical::LayoutAnchorY;
