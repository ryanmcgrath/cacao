use crate::foundation::NSUInteger;

/// This enum represents the different stock animations possible
/// for ListView row operations. You can pass it to `insert_rows`
/// and `remove_rows` - reloads don't get animations.
pub enum ListViewAnimation {
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

impl Into<NSUInteger> for ListViewAnimation {
    fn into(self) -> NSUInteger {
        match self {
            ListViewAnimation::None => 0x0,
            ListViewAnimation::Fade => 0x1,
            ListViewAnimation::Gap => 0x2,
            ListViewAnimation::SlideUp => 0x10,
            ListViewAnimation::SlideDown => 0x20,
            ListViewAnimation::SlideLeft => 0x30,
            ListViewAnimation::SlideRight => 0x40
        }
    }
}
