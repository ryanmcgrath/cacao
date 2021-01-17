
use crate::foundation::{NSInteger, NSUInteger};

pub enum TextAlign {
    Left,
    Right,
    Center,
    Justified,
    Natural
}

impl From<TextAlign> for NSInteger {
    fn from(alignment: TextAlign) -> Self {
        match alignment {
            TextAlign::Left => 0,
            TextAlign::Center => 1,
            TextAlign::Right => 2,
            TextAlign::Justified => 3,
            TextAlign::Natural => 4
        }
    }
}
