//! Certain properties of an `NSWindow` cannot be changed after initialization (e.g, the style
//! mask). This configuration object acts as a way to orchestrate enabling customization before the
//! window object is created - it's returned in your `WindowDelegate` object.

use crate::appkit::window::enums::{WindowStyle, WindowToolbarStyle};
use crate::foundation::NSUInteger;
use crate::geometry::Rect;

#[derive(Debug)]
pub struct WindowConfig {
    /// The style the window should have.
    pub style: NSUInteger,

    /// The initial dimensions for the window.
    pub initial_dimensions: Rect,

    /// From the Apple docs:
    ///
    /// _"When true, the window server defers creating the window device
    /// until the window is moved onscreen. All display messages sent to
    /// the window or its views are postponed until the window is created,
    /// just before it’s moved onscreen."_
    ///
    /// You generally just want this to be true, and it's the default for this struct.
    pub defer: bool,

    /// The style of toolbar that should be set here. This one is admittedly odd to be set here,
    /// but that's how the underlying API is designed, so we're sticking to it.
    ///
    /// This property is not used on macOS versions prior to Big Sur. This defaults to
    /// `ToolbarStyle::Automatic`; consult the specified enum
    /// for other variants.
    ///
    /// This setting is notably important for Preferences windows.
    pub toolbar_style: WindowToolbarStyle
}

impl Default for WindowConfig {
    fn default() -> Self {
        let mut config = WindowConfig {
            style: 0,
            initial_dimensions: Rect::new(100., 100., 1024., 768.),
            defer: true,
            toolbar_style: WindowToolbarStyle::Automatic
        };

        config.set_styles(&[
            WindowStyle::Resizable,
            WindowStyle::Miniaturizable,
            WindowStyle::UnifiedTitleAndToolbar,
            WindowStyle::Closable,
            WindowStyle::Titled,
            WindowStyle::FullSizeContentView
        ]);

        config
    }
}

impl WindowConfig {
    /// Given a set of styles, converts them to `NSUInteger` and stores them for later use.
    pub fn set_styles(&mut self, styles: &[WindowStyle]) {
        let mut style: NSUInteger = 0;

        for mask in styles {
            let i: NSUInteger = mask.into();
            style = style | i;
        }

        self.style = style;
    }

    /// Set the initial dimensions of this window.
    pub fn set_initial_dimensions(&mut self, top: f64, left: f64, width: f64, height: f64) {
        self.initial_dimensions = Rect::new(top, left, width, height);
    }

    /// Offered as a convenience API to match the others. You can just set this property directly
    /// if you prefer.
    ///
    /// Sets the Toolbar style. This is not used prior to macOS Big Sur (11.0); consult the
    /// `ToolbarStyle` enum for more information on possible values.
    pub fn set_toolbar_style(&mut self, style: WindowToolbarStyle) {
        self.toolbar_style = style;
    }
}
