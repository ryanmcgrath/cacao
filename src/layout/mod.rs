//! This module contains traits and helpers for layout. By default, standard frame-based layouts
//! are supported via the `Layout` trait, which all widgets implement. If you opt in to the
//! `AutoLayout` feature, each widget will default to using AutoLayout, which can be beneficial in
//! more complicated views that need to deal with differing screen sizes.

mod traits;
pub use traits::{Frame, Layout};

#[cfg(all(feature = "appkit", target_os = "macos"))]
mod animator;

#[cfg(all(feature = "appkit", target_os = "macos"))]
pub use animator::LayoutConstraintAnimatorProxy;

#[cfg(feature = "autolayout")]
mod attributes;

#[cfg(feature = "autolayout")]
pub use attributes::*;

#[cfg(feature = "autolayout")]
mod constraint;

#[cfg(feature = "autolayout")]
pub use constraint::LayoutConstraint;

#[cfg(feature = "autolayout")]
mod dimension;

#[cfg(feature = "autolayout")]
pub use dimension::LayoutAnchorDimension;

#[cfg(feature = "autolayout")]
mod horizontal;

#[cfg(feature = "autolayout")]
pub use horizontal::LayoutAnchorX;

#[cfg(feature = "autolayout")]
mod vertical;

#[cfg(feature = "autolayout")]
pub use vertical::LayoutAnchorY;

#[cfg(feature = "autolayout")]
mod safe_guide;

#[cfg(feature = "autolayout")]
pub use safe_guide::SafeAreaLayoutGuide;
