//! Certain properties of an `NSWindow` cannot be changed after initialization (e.g, the style
//! mask). This configuration object acts as a way to orchestrate enabling customization before the
//! window object is created - it's returned in your `WindowDelegate` object.

use crate::foundation::NSUInteger;
use crate::geometry::Rect;
use crate::macos::window::enums::WindowStyle;

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
    pub defer: bool
}

impl Default for WindowConfig {
    fn default() -> Self {
        let mut config = WindowConfig {
            style: 0,
            initial_dimensions: Rect::new(100., 100., 1024., 768.),
            defer: true
        };

        config.set_styles(&[
            WindowStyle::Resizable, WindowStyle::Miniaturizable, WindowStyle::UnifiedTitleAndToolbar,
            WindowStyle::Closable, WindowStyle::Titled
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
}
