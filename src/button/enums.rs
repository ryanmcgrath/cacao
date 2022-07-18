use crate::foundation::NSUInteger;

/// Represents a bezel style for a button. This is a macOS-specific control, and has no effect
/// under iOS or tvOS.
#[cfg(feature = "appkit")]
#[derive(Debug)]
pub enum BezelStyle {
    /// A standard circular button.
    Circular,

    /// A standard disclosure style button.
    Disclosure,

    /// The standard looking "Help" (?) button.
    HelpButton,

    /// An inline style, varies across OS's.
    Inline,

    /// A recessed style, varies slightly across OS's.
    Recessed,

    /// A regular square style, with no special styling.
    RegularSquare,

    /// A standard rounded rectangle.
    RoundRect,

    /// A standard rounded button.
    Rounded,

    /// A standard rounded disclosure button.
    RoundedDisclosure,

    /// A shadowless square styl.e
    ShadowlessSquare,

    /// A small square style.
    SmallSquare,

    /// A textured rounded style.
    TexturedRounded,

    /// A textured square style.
    TexturedSquare,

    /// Any style that's not known by this framework (e.g, if Apple 
    /// introduces something new).
    Unknown(NSUInteger)
}

#[cfg(feature = "appkit")]
impl From<BezelStyle> for NSUInteger {
    fn from(style: BezelStyle) -> Self {
        match style {
            BezelStyle::Circular => 7,
            BezelStyle::Disclosure => 5,
            BezelStyle::HelpButton => 9,
            BezelStyle::Inline => 15,
            BezelStyle::Recessed => 13,
            BezelStyle::RegularSquare => 2,
            BezelStyle::RoundRect => 12,
            BezelStyle::Rounded => 1,
            BezelStyle::RoundedDisclosure => 14,
            BezelStyle::ShadowlessSquare => 6,
            BezelStyle::SmallSquare => 10,
            BezelStyle::TexturedRounded => 11,
            BezelStyle::TexturedSquare => 8,
            BezelStyle::Unknown(i) => i
        }
    }
}

#[cfg(feature = "appkit")]
impl From<NSUInteger> for BezelStyle {
    fn from(i: NSUInteger) -> Self {
        match i {
            7 => Self::Circular,
            5 => Self::Disclosure,
            9 => Self::HelpButton,
            15 => Self::Inline,
            13 => Self::Recessed,
            2 => Self::RegularSquare,
            12 => Self::RoundRect,
            1 => Self::Rounded,
            14 => Self::RoundedDisclosure,
            6 => Self::ShadowlessSquare,
            10 => Self::SmallSquare,
            11 => Self::TexturedRounded,
            8 => Self::TexturedSquare,
            i => Self::Unknown(i)
        }
    }
}
