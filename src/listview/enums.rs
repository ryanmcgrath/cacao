use crate::foundation::{NSInteger, NSUInteger};

/// This enum represents the different stock animations possible
/// for ListView row operations. You can pass it to `insert_rows`
/// and `remove_rows` - reloads don't get animations.
pub enum RowAnimation {
    /// No animation.
    None,

    /// Fades rows in and out.
    Fade,

    /// Creates a gap - this one is mostly useful during
    /// drag and drop operations.
    Gap,

    /// Animates in or out by sliding upwards.
    SlideUp,
    
    /// Animates in or out by sliding down.
    SlideDown,

    /// Animates in or out by sliding left.
    SlideLeft,

    /// Animates in or out by sliding right.
    SlideRight
}

impl Into<NSUInteger> for RowAnimation {
    fn into(self) -> NSUInteger {
        match self {
            RowAnimation::None => 0x0,
            RowAnimation::Fade => 0x1,
            RowAnimation::Gap => 0x2,
            RowAnimation::SlideUp => 0x10,
            RowAnimation::SlideDown => 0x20,
            RowAnimation::SlideLeft => 0x30,
            RowAnimation::SlideRight => 0x40
        }
    }
}

#[derive(Debug)]
pub enum RowEdge {
    Leading,
    Trailing
}

impl Into<RowEdge> for NSInteger {
    fn into(self) -> RowEdge {
        match self {
            0 => RowEdge::Leading,
            1 => RowEdge::Trailing,

            // @TODO: This *should* be unreachable, provided macOS doesn't start
            // letting people swipe from vertical directions to reveal stuff. Have to 
            // feel like there's a better way to do this, though...
            _ => { unreachable!(); }
        }
    }
}
