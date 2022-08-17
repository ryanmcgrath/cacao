use crate::foundation::NSUInteger;

/// The type of spinner style you're after.
#[derive(Debug)]
pub enum ProgressIndicatorStyle {
    /// A loading bar.
    Bar,

    /// A spinning circle.
    Spinner,
}

impl From<ProgressIndicatorStyle> for NSUInteger {
    fn from(style: ProgressIndicatorStyle) -> Self {
        match style {
            ProgressIndicatorStyle::Bar => 0,
            ProgressIndicatorStyle::Spinner => 1,
        }
    }
}
