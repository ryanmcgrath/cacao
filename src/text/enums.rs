use crate::foundation::{NSInteger, NSUInteger};

/// Specifies how text should align for a supported control.
#[derive(Copy, Clone, Debug)]
pub enum TextAlign {
    /// Align text to the left.
    Left,

    /// Align text to the right.
    Right,

    /// Center-align text.
    Center,

    /// Justify text.
    Justified,

    /// Natural.
    Natural,
}

impl From<TextAlign> for NSInteger {
    fn from(alignment: TextAlign) -> Self {
        match alignment {
            TextAlign::Left => 0,
            TextAlign::Center => 1,
            TextAlign::Right => 2,
            TextAlign::Justified => 3,
            TextAlign::Natural => 4,
        }
    }
}

/// Instructs text controls how to optimize line breaks.
#[derive(Copy, Clone, Debug)]
pub enum LineBreakMode {
    /// Wrap at word boundaries (the default)
    WrapWords,

    /// Wrap at character boundaries
    WrapChars,

    /// Clip with no regard
    Clip,

    /// Truncate the start, e.g, ...my sentence
    TruncateHead,

    /// Truncate the end, e.g, my sentenc...
    TruncateTail,

    /// Truncate the middle, e.g, my se...ce
    TruncateMiddle,
}

impl Into<NSUInteger> for LineBreakMode {
    fn into(self) -> NSUInteger {
        match self {
            LineBreakMode::WrapWords => 0,
            LineBreakMode::WrapChars => 1,
            LineBreakMode::Clip => 2,
            LineBreakMode::TruncateHead => 3,
            LineBreakMode::TruncateTail => 4,
            LineBreakMode::TruncateMiddle => 5,
        }
    }
}
