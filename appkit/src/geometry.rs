//! Wrapper methods for various geometry types (rects, sizes, ec).

use crate::foundation::{CGRect, CGPoint, CGSize};

/// A struct that represents a box - top, left, width and height.
pub struct Rect {
    /// Distance from the top, in points.
    pub top: f64,
    
    /// Distance from the left, in points.
    pub left: f64,

    /// Width, in points.
    pub width: f64,

    /// Height, in points.
    pub height: f64
}

impl Rect {
    /// Returns a new `Rect` initialized with the values specified.
    pub fn new(top: f64, left: f64, width: f64, height: f64) -> Self {
        Rect { top: top, left: left, width: width, height: height }
    }

    /// Returns a zero'd out Rect, with f64 (32-bit is mostly dead on Cocoa, so... this is "okay").
    pub fn zero() -> Rect {
        Rect {
            top: 0.0,
            left: 0.0,
            width: 0.0,
            height: 0.0
        }
    }
}

impl From<Rect> for CGRect {
    fn from(rect: Rect) -> CGRect {
        CGRect::new(
             &CGPoint::new(rect.top, rect.left),
             &CGSize::new(rect.width, rect.height)
        )
    }
}
