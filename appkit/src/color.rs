//! Implements `Color`. Heavily based on the `Color` module in Servo's CSS parser, but tweaked
//! for (what I believe) is a friendlier API.

use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, CGFloat};

/// A color with red, green, blue, and alpha components, in a byte each.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    /// The red component.
    pub red: u8,
    /// The green component.
    pub green: u8,
    /// The blue component.
    pub blue: u8,
    /// The alpha component.
    pub alpha: u8,
}

impl Default for Color {
    fn default() -> Color {
        Color { red: 0, green: 0, blue: 0, alpha: 0 }
    }
}

impl Color {
    /// Constructs a new Color value from float components. It expects the red,
    /// green, blue and alpha channels in that order, and all values will be
    /// clamped to the 0.0 ... 1.0 range.
    #[inline]
    pub fn from_floats(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self::new(
            clamp_unit_f32(red),
            clamp_unit_f32(green),
            clamp_unit_f32(blue),
            clamp_unit_f32(alpha),
        )
    }

    /// Returns a transparent color.
    #[inline]
    pub fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Same thing, but with `u8` values instead of floats in the 0 to 1 range.
    #[inline]
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }
    
    /// Maps to NS/UIColor.
    pub fn into_platform_specific_color(&self) -> id {
        let red = self.red as CGFloat / 255.0;
        let green = self.green as CGFloat / 255.0;
        let blue = self.blue as CGFloat / 255.0;
        let alpha = self.alpha as CGFloat / 255.0;
       
        unsafe {
            msg_send![class!(NSColor), colorWithRed:red green:green blue:blue alpha:alpha]
        }
    }

    /// Returns the red channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn red_f32(&self) -> f32 {
        self.red as f32 / 255.0
    }

    /// Returns the green channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn green_f32(&self) -> f32 {
        self.green as f32 / 255.0
    }

    /// Returns the blue channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn blue_f32(&self) -> f32 {
        self.blue as f32 / 255.0
    }

    /// Returns the alpha channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn alpha_f32(&self) -> f32 {
        self.alpha as f32 / 255.0
    }    
}

#[inline]
pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
    rgba(red, green, blue, 255)
}

#[inline]
pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color::new(red, green, blue, alpha)
}

fn clamp_unit_f32(val: f32) -> u8 {
    // Whilst scaling by 256 and flooring would provide
    // an equal distribution of integers to percentage inputs,
    // this is not what Gecko does so we instead multiply by 255
    // and round (adding 0.5 and flooring is equivalent to rounding)
    //
    // Chrome does something similar for the alpha value, but not
    // the rgb values.
    //
    // See https://bugzilla.mozilla.org/show_bug.cgi?id=1340484
    //
    // Clamping to 256 and rounding after would let 1.0 map to 256, and
    // `256.0_f32 as u8` is undefined behavior:
    //
    // https://github.com/rust-lang/rust/issues/10184
    clamp_floor_256_f32(val * 255.)
}

fn clamp_floor_256_f32(val: f32) -> u8 {
    val.round().max(0.).min(255.) as u8
}
