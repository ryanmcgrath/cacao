//! Various types used for Toolbar configuration.

use crate::foundation::{id, NSString, NSUInteger};

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

/// Represents an item identifier for items in a Toolbar.
#[derive(Clone, Debug)]
pub enum ItemIdentifier {
    /// Represents a custom item. Use this when you need to handle your own item types.
    Custom(&'static str),

    /// Represents a standard cloud-sharing icon. Available from 10.12 onwards.
    CloudSharing,

    /// A flexible space identifier. Fills space, flexibly.
    FlexibleSpace,

    /// A standard print toolbar item. Will send the necessary print calls to the first responder.
    Print,

    /// A standard identifier for showing the colors panel.
    Colors,

    /// A standard identifier for showing the fonts panel.
    Fonts,

    /// A standard identifier for showing blank space.
    Space,

    /// Standard toolbar item identifier for a sidebar. Will handle automatically hiding and
    /// showing a SplitViewController sidebar if it's the window content view controller and the
    /// first responder.
    ///
    /// Note that this API was introduced in Big Sur (11.0), and you may need to check against this
    /// at runtime to ensure behavior is appropriate on older OS versions (if you support them).
    ToggleSidebar,

    /// Standard toolbar item for a spot that tracks the sidebar border. In your delegate, use this
    /// to indicate what items should be on the side of the sidebar and content.
    ///
    /// For example:
    ///
    /// ``` rust
    /// vec![ItemIdentifier::ToggleSidebar, ItemIdentifier::SidebarTracker, ItemIdentifier::Print]
    /// ```
    ///
    /// Would result in the toggle sidebar item showing up in the sidebar on the left, and the
    /// print item showing up in the content area on the right.
    ///
    /// Note that this API was introduced in Big Sur (11.0), and you may need to check against this
    /// at runtime to ensure behavior is appropriate on older OS versions (if you support them).
    ///
    SidebarTracker
}

extern "C" {
    static NSToolbarToggleSidebarItemIdentifier: id;
    static NSToolbarCloudSharingItemIdentifier: id;
    static NSToolbarFlexibleSpaceItemIdentifier: id;
    static NSToolbarPrintItemIdentifier: id;
    static NSToolbarShowColorsItemIdentifier: id;
    static NSToolbarShowFontsItemIdentifier: id;
    static NSToolbarSpaceItemIdentifier: id;
    static NSToolbarSidebarTrackingSeparatorItemIdentifier: id;
}

impl ItemIdentifier {
    /// Returns the NSString necessary for the toolbar to operate.
    pub(crate) fn to_nsstring(&self) -> id {
        unsafe {
            match self {
                Self::Custom(s) => NSString::new(s).into(),
                Self::CloudSharing => NSToolbarCloudSharingItemIdentifier,
                Self::FlexibleSpace => NSToolbarFlexibleSpaceItemIdentifier,
                Self::Print => NSToolbarPrintItemIdentifier,
                Self::Colors => NSToolbarShowColorsItemIdentifier,
                Self::Fonts => NSToolbarShowFontsItemIdentifier,
                Self::Space => NSToolbarSpaceItemIdentifier,
                Self::ToggleSidebar => NSToolbarToggleSidebarItemIdentifier,
                Self::SidebarTracker => NSToolbarSidebarTrackingSeparatorItemIdentifier
            }
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
