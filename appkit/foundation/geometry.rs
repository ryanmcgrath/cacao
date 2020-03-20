//! Implements Core Graphics Geometry types. Most of this is lifted from `servo/core-foundation-rs`
//! - as such, we include a copy of the license below.
//!
//! Copyright (c) 2012-2013 Mozilla Foundation
//! 
//! Permission is hereby granted, free of charge, to any
//! person obtaining a copy of this software and associated
//! documentation files (the "Software"), to deal in the
//! Software without restriction, including without
//! limitation the rights to use, copy, modify, merge,
//! publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software
//! is furnished to do so, subject to the following
//! conditions:
//!
//! The above copyright notice and this permission notice
//! shall be included in all copies or substantial portions
//! of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
//! ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
//! TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//! PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
//! SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//! CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
//! IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//! DEALINGS IN THE SOFTWARE.

use crate::foundation::CGFloat;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    #[inline]
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize {
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    #[inline]
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint {
            x: x,
            y: y,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize
}

impl CGRect {
    #[inline]
    pub fn new(origin: &CGPoint, size: &CGSize) -> CGRect {
        CGRect {
            origin: *origin,
            size: *size,
        }
    }
}
