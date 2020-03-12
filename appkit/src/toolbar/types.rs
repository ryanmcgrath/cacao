//! Various types used for Toolbar configuration.

use cocoa::foundation::NSUInteger;

/// Represents the display mode(s) a Toolbar can render in.
#[derive(Clone, Copy, Debug)]
pub enum ToolbarDisplayMode {
    /// The default display mode.
    Default,

    /// Show icon and label.
    IconAndLabel,

    /// Show icon only.
    IconOnly,

    /// Show label only.
    LabelOnly
}

impl From<ToolbarDisplayMode> for NSUInteger {
    fn from(mode: ToolbarDisplayMode) -> Self {
        match mode {
            ToolbarDisplayMode::Default => 0,
            ToolbarDisplayMode::IconAndLabel => 1,
            ToolbarDisplayMode::IconOnly => 2,
            ToolbarDisplayMode::LabelOnly => 3
        }
    }
}

/// Represents the size mode a Toolbar can use.
#[derive(Clone, Copy, Debug)]
pub enum ToolbarSizeMode {
    /// The default size mode.
    Default,

    /// The regular size mode.
    Regular,

    /// The small size mode.
    Small
}

impl From<ToolbarSizeMode> for NSUInteger {
    fn from(mode: ToolbarSizeMode) -> Self {
        match mode {
            ToolbarSizeMode::Default => 0,
            ToolbarSizeMode::Regular => 1,
            ToolbarSizeMode::Small => 2
        }
    }
}
