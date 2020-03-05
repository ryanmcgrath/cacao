//! Wrapper methods for various geometry types (rects, sizes, ec).

use cocoa::foundation::{NSRect, NSPoint, NSSize};

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

impl From<Rect> for NSRect {
    fn from(rect: Rect) -> NSRect {
        NSRect::new(
             NSPoint::new(rect.top, rect.left),
             NSSize::new(rect.width, rect.height)
        )
    }
}
