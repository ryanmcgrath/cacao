//! Enums used in Window construction and handling.

use crate::foundation::{NSInteger, NSUInteger};

/// Describes window styles that can be displayed.
#[derive(Clone, Copy, Debug)]
pub enum WindowStyle {
    /// Window has no border. You generally do not want this.
    Borderless,

    /// Window supports title.
    Titled,

    /// Window is closable.
    Closable,

    /// Window can be shrunk.
    Miniaturizable,

    /// Window can be resized.
    Resizable,

    /// Window does not separate area between title and toolbar.
    UnifiedTitleAndToolbar,

    /// Window is full screen.
    FullScreen,

    /// Window does not buffer content view below title/toolbar.
    FullSizeContentView,

    /// Utility window.
    Utility,

    /// Modal window for doc.
    DocModalWindow,

    /// Non-activating panel.
    NonActivatingPanel,

    /// A HUD window.
    HUDWindow,
}

impl From<WindowStyle> for NSUInteger {
    fn from(style: WindowStyle) -> Self {
        match style {
            WindowStyle::Borderless => 0,
            WindowStyle::Titled => 1 << 0,
            WindowStyle::Closable => 1 << 1,
            WindowStyle::Miniaturizable => 1 << 2,
            WindowStyle::Resizable => 1 << 3,
            WindowStyle::UnifiedTitleAndToolbar => 1 << 12,
            WindowStyle::FullScreen => 1 << 14,
            WindowStyle::FullSizeContentView => 1 << 15,
            WindowStyle::Utility => 1 << 4,
            WindowStyle::DocModalWindow => 1 << 6,
            WindowStyle::NonActivatingPanel => 1 << 7,
            WindowStyle::HUDWindow => 1 << 13,
        }
    }
}

impl From<&WindowStyle> for NSUInteger {
    fn from(style: &WindowStyle) -> Self {
        match style {
            WindowStyle::Borderless => 0,
            WindowStyle::Titled => 1 << 0,
            WindowStyle::Closable => 1 << 1,
            WindowStyle::Miniaturizable => 1 << 2,
            WindowStyle::Resizable => 1 << 3,
            WindowStyle::UnifiedTitleAndToolbar => 1 << 12,
            WindowStyle::FullScreen => 1 << 14,
            WindowStyle::FullSizeContentView => 1 << 15,
            WindowStyle::Utility => 1 << 4,
            WindowStyle::DocModalWindow => 1 << 6,
            WindowStyle::NonActivatingPanel => 1 << 7,
            WindowStyle::HUDWindow => 1 << 13,
        }
    }
}

/// Describes whether a window shows a title or not.
#[derive(Clone, Copy, Debug)]
pub enum TitleVisibility {
    /// Title is visible.
    Visible,

    /// Title is hidden.
    Hidden,
}

impl From<TitleVisibility> for NSInteger {
    fn from(visibility: TitleVisibility) -> Self {
        match visibility {
            TitleVisibility::Visible => 0,
            TitleVisibility::Hidden => 1,
        }
    }
}

/// Represents the styles a Toolbar can have. This setting is specific to macOS 11.0+ (Big Sur and
/// onwards); setting it won't change versions prior to Big Sur.
#[derive(Clone, Copy, Debug)]
pub enum WindowToolbarStyle {
    /// The default display mode. This will change the appearance based on whether it's 10.15 and
    /// earlier. In most cases, this is fine.
    Automatic,

    /// The style from macOS pre-11.0. Toolbar items will always be located underneath the
    /// titlebar.
    Expanded,

    /// A style specifically for Preferences windows. Toolbar items will be under the titlebar, and
    /// centered.
    Preferences,

    /// The Big Sur (11.0+) style. Titles appear next to controls.
    Unified,

    /// The Big Sur (11.0+) style, but more compact. Toolbar flushes up against the title and
    /// spacing is reduced.
    UnifiedCompact,
}

impl From<WindowToolbarStyle> for NSUInteger {
    fn from(mode: WindowToolbarStyle) -> Self {
        match mode {
            WindowToolbarStyle::Automatic => 0,
            WindowToolbarStyle::Expanded => 1,
            WindowToolbarStyle::Preferences => 2,
            WindowToolbarStyle::Unified => 3,
            WindowToolbarStyle::UnifiedCompact => 4,
        }
    }
}
