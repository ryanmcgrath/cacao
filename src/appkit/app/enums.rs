//! Various types used at the AppController level.

use crate::foundation::NSUInteger;

/// Used for determining how an application should handle quitting/terminating.
/// You return this in your `AppController` `should_terminate` method.
#[derive(Copy, Clone, Debug)]
pub enum TerminateResponse {
    /// Proceed with termination.
    Now,

    /// App should not be terminated.
    Cancel,

    /// It might be fine to proceed with termination later. Returning this value causes
    /// Cocoa to run the run loop until `should_terminate()` returns `true` or `false`.
    ///
    /// This return value is for primarily for cases where you need to provide alerts
    /// in order to decide whether to quit.
    Later,
}

impl From<TerminateResponse> for NSUInteger {
    fn from(response: TerminateResponse) -> NSUInteger {
        match response {
            TerminateResponse::Now => 1,
            TerminateResponse::Cancel => 0,
            TerminateResponse::Later => 2,
        }
    }
}

/// Used for responding to open/print/copy requests.
/// You only really need this for calling `App::reply_to_open_or_print()`.
/// The name is unfortunate, but it covers a variety of things, and by keeping it closer to the
/// `NSApplication` documentation it may help some poor soul who needs to find information about
/// it.
#[derive(Copy, Clone, Debug)]
pub enum AppDelegateResponse {
    /// Cancelled.
    Cancelled,

    /// Success.
    Success,

    /// Failed.
    Failure,
}

impl From<AppDelegateResponse> for NSUInteger {
    fn from(response: AppDelegateResponse) -> Self {
        match response {
            AppDelegateResponse::Cancelled => 1,
            AppDelegateResponse::Success => 0,
            AppDelegateResponse::Failure => 2,
        }
    }
}

/// Used (typically) when handling full-screening an application or window. Use these to instruct
/// how the full screen should work. Note that some may conflict!
///
/// From Apple's documentation, swapped to use the enum values here:
///
/// - _`AutoHideDock` and `HideDock` are mutually exclusive: You may specify one or the other, but
/// not both._
/// - _`AutoHideMenuBar` and `HideMenuBar` are mutually exclusive: You may specify one or the other, but not both._
/// - _If you specify `HideMenuBar`, it must be accompanied by `HideDock`._
/// - _If you specify `AutoHideMenuBar`, it must be accompanied by either `HideDock` or `AutoHideDock`._
/// - _If you specify any of `DisableProcessSwitching`, `DisableForceQuit`, `DisableSessionTermination`, or `DisableMenuBarTransparency`,
/// it must be accompanied by either `HideDock` or `AutoHideDock`._
/// - _`AutoHideToolbar` may be used only when both `FullScreen` and `AutoHideMenuBar` are also set.
#[derive(Copy, Clone, Debug)]
pub enum PresentationOption {
    /// The default mode.
    Default,

    /// Auto hide the dock. Will reappear when moused near.
    AutoHideDock,

    /// Dock is entirely disabled.
    HideDock,

    /// Auto hide the menubar. Will reappear when moused near.
    AutoHideMenuBar,

    /// Menubar is entirely disabled.
    HideMenuBar,

    /// All Apple Menu items are disabled.
    DisableAppleMenu,

    /// The process switching user interface (Command + Tab to cycle through apps) is disabled.
    DisableProcessSwitching,

    /// The force quit panel (displayed by pressing Command + Option + Esc) is disabled
    DisableForceQuit,

    /// The panel that shows the Restart, Shut Down, and Log Out options that are displayed as a result of pushing the power key is disabled.
    DisableSessionTermination,

    /// The app’s "Hide" menu item is disabled.
    DisableHideApplication,

    /// The menu bar transparency appearance is disabled.
    DisableMenuBarTransparency,

    /// The app is in fullscreen mode.
    FullScreen,

    /// When in fullscreen mode the window toolbar is detached from window and hides and shows with autoHidden menuBar.
    AutoHideToolbar,

    /// The behavior that allows the user to shake the mouse to locate the cursor is disabled.
    DisableCursorLocationAssistance,
}

impl From<PresentationOption> for NSUInteger {
    fn from(option: PresentationOption) -> Self {
        match option {
            PresentationOption::Default => 0,
            PresentationOption::AutoHideDock => (1 << 0),
            PresentationOption::HideDock => (1 << 1),
            PresentationOption::AutoHideMenuBar => (1 << 2),
            PresentationOption::HideMenuBar => (1 << 3),
            PresentationOption::DisableAppleMenu => (1 << 4),
            PresentationOption::DisableProcessSwitching => (1 << 5),
            PresentationOption::DisableForceQuit => (1 << 6),
            PresentationOption::DisableSessionTermination => (1 << 7),
            PresentationOption::DisableHideApplication => (1 << 8),
            PresentationOption::DisableMenuBarTransparency => (1 << 9),
            PresentationOption::FullScreen => (1 << 10),
            PresentationOption::AutoHideToolbar => (1 << 11),
            PresentationOption::DisableCursorLocationAssistance => (1 << 12),
        }
    }
}

impl From<&PresentationOption> for NSUInteger {
    fn from(option: &PresentationOption) -> Self {
        match option {
            PresentationOption::Default => 0,
            PresentationOption::AutoHideDock => (1 << 0),
            PresentationOption::HideDock => (1 << 1),
            PresentationOption::AutoHideMenuBar => (1 << 2),
            PresentationOption::HideMenuBar => (1 << 3),
            PresentationOption::DisableAppleMenu => (1 << 4),
            PresentationOption::DisableProcessSwitching => (1 << 5),
            PresentationOption::DisableForceQuit => (1 << 6),
            PresentationOption::DisableSessionTermination => (1 << 7),
            PresentationOption::DisableHideApplication => (1 << 8),
            PresentationOption::DisableMenuBarTransparency => (1 << 9),
            PresentationOption::FullScreen => (1 << 10),
            PresentationOption::AutoHideToolbar => (1 << 11),
            PresentationOption::DisableCursorLocationAssistance => (1 << 12),
        }
    }
}
