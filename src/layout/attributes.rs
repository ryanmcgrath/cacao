use crate::foundation::{NSInteger, NSUInteger};

/// Represents whether a layout is vertical or horizontal.
#[derive(Debug)]
pub enum LayoutConstraintOrientation {
    /// Horizontal orientation.
    Horizontal,

    /// Vertical orientation.
    Vertical,

    /// Represents an unknown value. This should never be constructed, but acts as a guard against
    /// a change in representation on the framework side. If a new value was ever introduced, it's
    /// caught here, and applications can handle it themselves if need be.
    Unknown(NSInteger)
}

impl From<NSInteger> for LayoutConstraintOrientation {
    fn from(i: NSInteger) -> Self {
        match i {
            0 => Self::Horizontal,
            1 => Self::Vertical,
            i => Self::Unknown(i)
        }
    }
}

/// Represents a relation between layout constraints. Used mostly internally.
#[derive(Debug)]
pub enum LayoutRelation {
    /// Relation is less than or equal to another specified relation.
    LessThanOrEqual,

    /// Relation is  equal to another specified relation.
    Equal,

    /// Relation is greater than or equal to another specified relation.
    GreaterThanOrEqual,

    /// Represents an unknown value. This should never be constructed, but acts as a guard against
    /// a change in representation on the framework side. If a new value was ever introduced, it's
    /// caught here, and applications can handle it themselves if need be.
    Unknown(NSInteger)
}

impl From<NSInteger> for LayoutRelation {
    fn from(i: NSInteger) -> Self {
        match i {
            -1 => Self::LessThanOrEqual,
            0 => Self::Equal,
            1 => Self::GreaterThanOrEqual,
            i => Self::Unknown(i)
        }
    }
}

/// Represents attributes for various layouts and constraints.
///
/// Note that this only covers attributes that are shared across platforms. In general, this is enough
/// to build apps that work everywhere - but if you need to specify something else, you can handle
/// it yourself with the `Unknown` variant.
#[derive(Debug)]
pub enum LayoutAttribute {
    /// The left side of the object’s alignment rectangle.
    Left,

    /// The right side of the object’s alignment rectangle.
    Right,

    /// The top of the object’s alignment rectangle.
    Top,

    /// The bottom of the object’s alignment rectangle.
    Bottom,

    /// The leading edge of the object’s alignment rectangle.
    Leading,

    /// The trailing edge of the object’s alignment rectangle.
    Trailing,

    /// The width of the object’s alignment rectangle.
    Width,

    /// The height of the object’s alignment rectangle.
    Height,

    /// The center along the x-axis of the object’s alignment rectangle.
    CenterX,

    /// The center along the y-axis of the object’s alignment rectangle.
    CenterY,

    /// The object’s baseline. For objects with more than one line of text, 
    /// this is the baseline for the bottommost line of text.
    LastBaseline,

    /// The object’s baseline. For objects with more than one line of text, 
    /// this is the baseline for the topmost line of text.
    FirstBaseline,

    /// A placeholder value that is used to indicate that the constraint’s 
    /// second item and second attribute are not used in any calculations. 
    ///
    /// This can be useful constraint that assigns a constant to an attribute. 
    NotAnAttribute,

    /// Represents an unknown value. This should never be constructed, but acts as a guard against
    /// a change in representation on the framework side. If a new value was ever introduced, it's
    /// caught here, and applications can handle it themselves if need be.
    Unknown(NSInteger)
}

impl From<NSInteger> for LayoutAttribute {
    fn from(i: NSInteger) -> Self {
        match i {
            1 => Self::Left,
            2 => Self::Right,
            3 => Self::Top,
            4 => Self::Bottom,
            5 => Self::Leading,
            6 => Self::Trailing,
            7 => Self::Width,
            8 => Self::Height,
            9 => Self::CenterX,
            10 => Self::CenterY,
            11 => Self::LastBaseline,
            12 => Self::FirstBaseline,
            0 => Self::NotAnAttribute,
            i => Self::Unknown(i)
        }
    }
}

/// Represents a layout format.
///
/// Note that this only covers formats that are shared across platforms. In general, this is enough
/// to build apps that work everywhere - but if you need to specify something else, you can handle
/// it yourself with the `Unknown` variant.
#[derive(Debug)]
pub enum LayoutFormat {
    /// Align all specified interface elements using NSLayoutAttributeLeft on each.
    AlignAllLeft,

    /// Align all specified interface elements using NSLayoutAttributeRight on each.
    AlignAllRight,

    /// Align all specified interface elements using NSLayoutAttributeTop on each.
    AlignAllTop,

    /// Align all specified interface elements using NSLayoutAttributeBottom on each.
    AlignAllBottom,

    /// Align all specified interface elements using NSLayoutAttributeLeading on each.
    AlignAllLeading,

    /// Align all specified interface elements using NSLayoutAttributeTrailing on each.
    AlignAllTrailing,

    /// Align all specified interface elements using NSLayoutAttributeCenterX on each.
    AlignAllCenterX,

    /// Align all specified interface elements using NSLayoutAttributeCenterY on each.
    AlignAllCenterY,

    /// Align all specified interface elements using the last baseline of each one.
    AlignAllLastBaseline,

    /// Arrange objects in order based on the normal text flow for the current user 
    /// interface language. In left-to-right languages (like English), this arrangement 
    /// results in the first object being placed farthest to the left, the next one to 
    /// its right, and so on. In right-to-left languages (like Arabic or Hebrew), the 
    /// ordering is reversed.
    DirectionLeadingToTrailing,

    /// Arrange objects in order from left to right.
    DirectionLeftToRight,

    /// Arrange objects in order from right to left.
    DirectionRightToLeft,

    /// Represents an unknown value. This should never be constructed, but acts as a guard against
    /// a change in representation on the framework side. If a new value was ever introduced, it's
    /// caught here, and applications can handle it themselves if need be.
    Unknown(NSUInteger)
}

impl From<NSUInteger> for LayoutFormat {
    fn from(i: NSUInteger) -> Self {
        match i {
            2 => Self::AlignAllLeft,
            4 => Self::AlignAllRight,
            8 => Self::AlignAllTop,
            16 => Self::AlignAllBottom,
            32 => Self::AlignAllLeading,
            64 => Self::AlignAllTrailing,
            512 => Self::AlignAllCenterX,
            1024 => Self::AlignAllCenterY,
            2048 => Self::AlignAllLastBaseline,
            0 => Self::DirectionLeadingToTrailing,
            65536 => Self::DirectionLeftToRight,
            131072 => Self::DirectionRightToLeft,
            i => Self::Unknown(i)
        }
    }
}

/// Specifies layout priority.
#[derive(Debug)]
pub enum LayoutPriority {
    /// Highest priority.
    Required,

    /// High priority. Will bend if absolutely necessary.
    High,

    /// Low priority.
    Low
}
