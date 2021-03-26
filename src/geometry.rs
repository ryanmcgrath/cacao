//! Wrapper methods for various geometry types (rects, sizes, ec).

use core_graphics::geometry::{CGRect, CGPoint, CGSize};

/// A struct that represents a box - top, left, width and height. You might use this for, say,
/// setting the initial frame of a view.
#[derive(Copy, Clone, Debug)]
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

impl From<CGRect> for Rect {
    fn from(rect: CGRect) -> Rect {
        Rect {
            top: rect.origin.y as f64,
            left: rect.origin.x as f64,
            width: rect.size.width as f64,
            height: rect.size.height as f64
        }
    }
}
