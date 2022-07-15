//! A module to house the traits used throughout this window
//! module. There's a few different ones, and it's just... cleaner, if
//! it's organized here.

use crate::appkit::app::PresentationOption;
use crate::appkit::window::Window;

/// Lifecycle events for anything that `impl Window`'s. These map to the standard Cocoa
/// lifecycle methods, but mix in a few extra things to handle offering configuration tools
/// in lieu of subclasses.
pub trait WindowDelegate {
    /// Used to cache subclass creations on the Objective-C side.
    /// You can just set this to be the name of your view type. This
    /// value *must* be unique per-type.
    const NAME: &'static str;

    /// You should rarely (read: probably never) need to implement this yourself.
    /// It simply acts as a getter for the associated `NAME` const on this trait.
    fn subclass_name(&self) -> &'static str {
        Self::NAME
    }

    /// Fires when this window has loaded in memory, and is about to display. This is a good point
    /// to set up your views and what not.
    ///
    /// If you're coming from the web, you can think of this as `DOMContentLoaded`.
    fn did_load(&mut self, _window: Window) {}

    /// Called when the user has attempted to close the window. NOT called when a user quits the
    /// application. Return false here if you need to handle the edge case.
    fn should_close(&self) -> bool {
        true
    }

    /// Fires when a window is going to close. You might opt to, say, clean up things here -
    /// perhaps you have a long running task, or something that should be removed.
    fn will_close(&self) {}

    /// Fired when the window is about to move.
    fn will_move(&self) {}

    /// Fired after the window has moved.
    fn did_move(&self) {}

    /// Fired before the window resizes, passing you the width and height.
    /// To avoid resizing, return the current size. To resize to a different size, return the
    /// desired size.
    ///
    /// The default implementation of this method returns `None`, indicating the system should just
    /// do its thing. If you implement it, you probably want that.
    fn will_resize(&self, width: f64, height: f64) -> (f64, f64) {
        (width, height)
    }

    /// Fired after the window has resized.
    fn did_resize(&self) {}

    /// Fired when the window is going to live resize.
    fn will_start_live_resize(&self) {}

    /// Fired when the window has ended live resizing.
    fn did_end_live_resize(&self) {}

    /// Fired when the window changes screens - you might find this useful for certain scenarios,
    /// such as rendering in retina vs non-retina environments.
    fn did_change_screen(&self) {}

    /// Fired when the window profile changes screens - you might find this useful for certain scenarios,
    /// such as rendering in retina vs non-retina environments.
    fn did_change_screen_profile(&self) {}

    /// Fired when the window backing properties change - you might find this useful for certain scenarios,
    /// such as rendering in retina vs non-retina environments. It's rare to need this though.
    fn did_change_backing_properties(&self) {}

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

    /// Fires when the system is moving a window to full screen and wants to know what content size
    /// to use. By default, this just returns the system-provided content size, but you can
    /// override it if need be.
    fn content_size_for_full_screen(&self, proposed_width: f64, proposed_height: f64) -> (f64, f64) {
        (proposed_width, proposed_height)
    }

    /// Specify options for when this window goes full screen.
    /// By default, this returns `None`, which tells the system to proceed as it normally would
    /// without customization.
    fn presentation_options_for_full_screen(&self) -> Option<&[PresentationOption]> {
        None
    }

    /// Fires when this window is about to go full screen.
    fn will_enter_full_screen(&self) {}

    /// Fires when this window entered full screen.
    fn did_enter_full_screen(&self) {}

    /// Fires when this window is about to exit full screen.
    fn will_exit_full_screen(&self) {}

    /// Fires when this window exited full screen.
    fn did_exit_full_screen(&self) {}

    /// Fires when this window failed to enter full screen.
    fn did_fail_to_enter_full_screen(&self) {}

    /// Fires when this window failed to exit full screen.
    fn did_fail_to_exit_full_screen(&self) {}

    /// Fired when the occlusion state for this window has changed. Similar in nature to the
    /// app-level event, just for a Window.
    fn did_change_occlusion_state(&self) {}

    /// Fired when the Window receives a `didExpose` message from higher up in the chain.
    fn did_expose(&self) {}

    /// Fired when the Window receives an `update` message from higher up in the chain.
    fn did_update(&self) {}

    /// If you want your window to close when the `ESC` key is hit, implement this.
    /// This is mostly useful for windows that present as modal sheets.
    fn cancel(&self) {}
}
