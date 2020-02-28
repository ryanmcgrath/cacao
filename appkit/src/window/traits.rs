//! A module to house the traits used throughout this window
//! module. There's a few different ones, and it's just... cleaner, if
//! it's organized here.

use crate::window::WindowConfig;

/// `WindowController` is a trait that handles providing higher level methods
/// that map into platform specific methods. Typically, you won't want to (or at least, won't need
/// to) implement this yourself - simply derive `WindowWrapper` and it'll work for you.
///
/// By deriving or implementing this, you get usable methods on your struct - for example:
///
/// ```
/// use appkit::{AppDelegate, Window, WindowController, WindowWrapper};
///
/// #[derive(Default, WindowWrapper)]
/// struct MyWindow {
///     window: Window
/// }
///
/// impl WindowController for MyWindow {
///     // The default implementation is actually okay!
/// }
/// 
/// #[derive(Default)]
/// struct MyApp {
///     window: MyWindow
/// }
///
/// impl AppDelegate for MyApp {
///     fn did_finish_launching(&mut self) {
///         window.show();
///     }
/// }
///
/// fn main() {
///     let app = App::new("com.myapp.lol", MyApp::default());
///     app.run();
/// }
/// ```
pub trait WindowWrapper {
    /// Sets the title for the underlying window.
    fn set_title(&self, title: &str);

    /// Calls through to the NSWindow show method (technically, `[NSWindowController showWindow:]`.
    /// Notable, this handles passing the implementing entity as the delegate, ensuring that
    /// callbacks work appropriately.
    ///
    /// We're technically setting the delegate later than is ideal, but in practice it works fine
    /// in most cases due to the underlying implementation of `NSWindow` deferring things until
    /// needed.
    fn show(&self);

    /// Calls through to the native NSwindow close implementation.
    fn close(&self);
}

/// Lifecycle events for anything that `impl Window`'s. These map to the standard Cocoa
/// lifecycle methods, but mix in a few extra things to handle offering configuration tools
/// in lieu of subclasses.
pub trait WindowController {
    /// `NSWindow` has a lovely usability feature wherein it'll cache the position in
    /// `UserDefaults` when a window closes. This is generally nice for a lot of cases (e.g,
    /// documents) but needs a key to work with. A blank key, the default, will not cache - so
    /// you can implement this and return your own key per window delegate to cache accordingly.
    fn autosave_name(&self) -> &str { "" }
    
    /// The framework offers a standard, modern `NSWindow` by default - but sometimes you want
    /// something else. Implement this and return your desired Window configuration.
    fn config(&self) -> WindowConfig { WindowConfig::default() }

    /// Fires when this window has loaded in memory, and is about to display. This is a good point
    /// to set up your views and what not.
    ///
    /// If you're coming from the web, you can think of this as `DOMContentLoaded`.
    fn did_load(&self) {}

    /// Fires when a window is going to close. You might opt to, say, clean up things here -
    /// perhaps you have a long running task, or something that should be removed.
    fn will_close(&self) {}

    /// Fired when the window is about to move. 
    fn will_move(&self) {}

    /// Fired after the window has moved.
    fn did_move(&self) {}

    /// Fired when the window changes screens - you might find this useful for certain scenarios,
    /// such as rendering in retina vs non-retina environments.
    fn did_change_screen(&self) {}

    /// Fires when this window is about to become the key window.
    fn did_become_key(&self) {}

    /// Fires when this window is about to resign key window status.
    fn did_resign_key(&self) {}

    /// Fires when this window is about to become the main window.
    fn did_become_main(&self) {}

    /// Fires when this window is about to resign main status.
    fn did_resign_main(&self) {}

    /// Fires when the window is about to miniaturize (e.g, to the Dock).
    fn will_miniaturize(&self) {}

    /// Fires when this window miniaturized (e.g, to the Dock).
    fn did_miniaturize(&self) {}

    /// Fires when this window de-miniaturized (e.g, from the Dock).
    fn did_deminiaturize(&self) {}

    /// Fires when this window is about to go full screen.
    fn will_enter_fullscreen(&self) {}

    /// Fires when this window entered full screen.
    fn did_enter_fullscreen(&self) {}

    /// Fires when this window is about to exit full screen.
    fn will_exit_fullscreen(&self) {}

    /// Fires when this window exited full screen.
    fn did_exit_fullscreen(&self) {}

    /// Fires when this window failed to enter full screen.
    fn did_fail_to_enter_fullscreen(&self) {}

    /// Fires when this window failed to exit full screen.
    fn did_fail_to_exit_fullscreen(&self) {}
}
