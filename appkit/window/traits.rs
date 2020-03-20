//! A module to house the traits used throughout this window
//! module. There's a few different ones, and it's just... cleaner, if
//! it's organized here.

use crate::window::Window;

/// Lifecycle events for anything that `impl Window`'s. These map to the standard Cocoa
/// lifecycle methods, but mix in a few extra things to handle offering configuration tools
/// in lieu of subclasses.
pub trait WindowDelegate {    
    /// Fires when this window has loaded in memory, and is about to display. This is a good point
    /// to set up your views and what not.
    ///
    /// If you're coming from the web, you can think of this as `DOMContentLoaded`.
    fn did_load(&mut self, _window: Window) {}

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
