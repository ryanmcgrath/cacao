//! Generic enums that don't fit anywhere else yet.

use crate::foundation::{NSUInteger};

/// Used to set whether and/or how a view or cell draws a focus ring.
#[cfg(feature = "macos")]
#[derive(Debug)]
pub enum FocusRingType {
    /// Whatever the default is.
    Default,
    
    /// None.
    None,

    /// Standard focus ring.
    Exterior,

    // Should never be used, but used as a placeholder in case the underlying
    // system framework ever opts to return something we don't expect.
    Unknown(NSUInteger)
}

#[cfg(feature = "macos")]
impl From<FocusRingType> for NSUInteger {
    fn from(ring_type: FocusRingType) -> Self {
        match ring_type {
            FocusRingType::Default => 0,
            FocusRingType::None => 1,
            FocusRingType::Exterior => 2,
            FocusRingType::Unknown(i) => i
        }
    }
}

impl From<NSUInteger> for FocusRingType {
    fn from(i: NSUInteger) -> Self {
        match i {
            0 => Self::Default,
            1 => Self::None,
            2 => Self::Exterior,
            i => Self::Unknown(i)
        }
    }
}
