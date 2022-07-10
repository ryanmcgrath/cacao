//! Wraps `NSWindow` on macOS and `UIWindow` on iOS.
//!
//! Using `WindowDelegate`, you're able to handle lifecycle events on your `Window` (such as
//! resizing, closing, and so on). Note that interaction patterns are different between macOS and
//! iOS windows, so your codebase may need to differ quite a bit here.
//!
//! Of note: on macOS, in places where things are outright deprecated, this framework will opt to
//! not bother providing access to them. If you require functionality like that, you're free to use
//! the `objc` field on a `Window` to instrument it with the Objective-C runtime on your own.

use block::ConcreteBlock;

use core_graphics::base::CGFloat;
use core_graphics::geometry::{CGRect, CGSize};

use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::appkit::toolbar::{Toolbar, ToolbarDelegate};
use crate::color::Color;
use crate::foundation::{id, nil, to_bool, YES, NO, NSString, NSInteger, NSUInteger};
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::{os, Controller};

mod class;
use class::register_window_class_with_delegate;

mod config;
pub use config::WindowConfig;

mod controller;
pub use controller::WindowController;

mod enums;
pub use enums::*;

mod traits;
pub use traits::WindowDelegate;

pub(crate) static WINDOW_DELEGATE_PTR: &str = "rstWindowDelegate";

/// A `Window` represents your way of interacting with an `NSWindow`. It wraps the various moving
/// pieces to enable you to focus on reacting to lifecycle methods and doing your thing.
#[derive(Debug)]
pub struct Window<T = ()> {
    /// Represents an `NS/UIWindow` in the Objective-C runtime.
    pub objc: ShareId<Object>,

    /// A delegate for this window.
    pub delegate: Option<Box<T>>
}

impl Default for Window {
    /// Returns a default `Window`, with a default `WindowConfig`.
    fn default() -> Self {
        Window::new(WindowConfig::default())
    }
}

impl Window {
    /// Constructs a new `Window`. You can use this instead of the `default()` method if you'd like
    /// to customize the appearance of a `Window`.
    ///
    /// Why the config? Well, certain properties of windows are really not meant to be altered
    /// after we initialize the backing `NSWindow`.
    pub fn new(config: WindowConfig) -> Window {
        let objc = unsafe {
            // This behavior might make sense to keep as default (YES), but I think the majority of
            // apps that would use this toolkit wouldn't be tab-oriented...
            let _: () = msg_send![class!(NSWindow), setAllowsAutomaticWindowTabbing:NO];

            let alloc: id = msg_send![class!(NSWindow), alloc];

            // Other types of backing (Retained/NonRetained) are archaic, dating back to the
            // NeXTSTEP era, and are outright deprecated... so we don't allow setting them.
            let buffered: NSUInteger = 2;
            let dimensions: CGRect = config.initial_dimensions.into();
            let window: id = msg_send![alloc, initWithContentRect:dimensions
                styleMask:config.style
                backing:buffered
                defer:match config.defer {
                    true => YES,
                    false => NO
                }
            ];

            let _: () = msg_send![window, autorelease];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync by releasing the window out from underneath of
            // us.
            let _: () = msg_send![window, setReleasedWhenClosed:NO];

            let _: () = msg_send![window, setRestorable:NO];

            // This doesn't exist prior to Big Sur, but is important to support for Big Sur.
            //
            // Why this isn't a setting on the Toolbar itself I'll never know.
            if os::is_minimum_version(11) {
                let toolbar_style: NSUInteger = config.toolbar_style.into();
                let _: () = msg_send![window, setToolbarStyle:toolbar_style];
            }

            ShareId::from_ptr(window)
        };

        Window {
            objc: objc,
            delegate: None
        }
    }
}

impl<T> Window<T> where T: WindowDelegate + 'static {
    /// Constructs a new Window with a `config` and `delegate`. Using a `WindowDelegate` enables
    /// you to respond to window lifecycle events - visibility, movement, and so on. It also
    /// enables easier structure of your codebase, and in a way simulates traditional class based
    /// architectures... just without the subclassing.
    pub fn with(config: WindowConfig, delegate: T) -> Self {
        let class = register_window_class_with_delegate::<T>(&delegate);
        let mut delegate = Box::new(delegate);

        let objc = unsafe {
            // This behavior might make sense to keep as default (YES), but I think the majority of
            // apps that would use this toolkit wouldn't be tab-oriented...
            let _: () = msg_send![class!(NSWindow), setAllowsAutomaticWindowTabbing:NO];

            let alloc: id = msg_send![class, alloc];

            // Other types of backing (Retained/NonRetained) are archaic, dating back to the
            // NeXTSTEP era, and are outright deprecated... so we don't allow setting them.
            let buffered: NSUInteger = 2;
            let dimensions: CGRect = config.initial_dimensions.into();
            let window: id = msg_send![alloc, initWithContentRect:dimensions
                styleMask:config.style
                backing:buffered
                defer:match config.defer {
                    true => YES,
                    false => NO
                }
            ];

            let delegate_ptr: *const T = &*delegate;
            (&mut *window).set_ivar(WINDOW_DELEGATE_PTR, delegate_ptr as usize);

            let _: () = msg_send![window, autorelease];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync by releasing the window out from underneath of
            // us.
            let _: () = msg_send![window, setReleasedWhenClosed:NO];

            // We set the window to be its own delegate - this is cleaned up inside `Drop`.
            let _: () = msg_send![window, setDelegate:window];

            let _: () = msg_send![window, setRestorable:NO];

            // This doesn't exist prior to Big Sur, but is important to support for Big Sur.
            //
            // Why this isn't a setting on the Toolbar itself I'll never know.
            if os::is_minimum_version(11) {
                let toolbar_style: NSUInteger = config.toolbar_style.into();
                let _: () = msg_send![window, setToolbarStyle:toolbar_style];
            }

            ShareId::from_ptr(window)
        };

        {
            (&mut delegate).did_load(Window {
                delegate: None,
                objc: objc.clone()
            });
        }

        Window {
            objc: objc,
            delegate: Some(delegate)
        }
    }
}

impl<T> Window<T> {
    /// Handles setting the title on the underlying window. Allocates and passes an `NSString` over
    /// to the Objective C runtime.
    pub fn set_title(&self, title: &str) {
        unsafe {
            let title = NSString::new(title);
            let _: () = msg_send![&*self.objc, setTitle:title];
        }
    }

    /// Sets the title visibility for the underlying window.
    pub fn set_title_visibility(&self, visibility: TitleVisibility) {
        unsafe {
            let v = NSInteger::from(visibility);
            let _: () = msg_send![&*self.objc, setTitleVisibility:v];
        }
    }

    /// Used for configuring whether the window is movable via the background.
    pub fn set_movable_by_background(&self, movable: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setMovableByWindowBackground:match movable {
                true => YES,
                false => NO
            }];
        }
    }

    /// Used for setting whether this titlebar appears transparent.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitlebarAppearsTransparent:match transparent {
                true => YES,
                false => NO
            }];
        }
    }

    /// Used for setting this Window autosave name.
    pub fn set_autosave_name(&self, name: &str) {
        unsafe {
            let autosave = NSString::new(name);
            let _: () = msg_send![&*self.objc, setFrameAutosaveName:autosave];
        }
    }

    /// Sets the content size for this window.
    pub fn set_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setContentSize:size];
        }
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setContentMinSize:size];
        }
    }

    /// Sets the maximum size this window can shrink to.
    pub fn set_maximum_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setContentMaxSize:size];
        }
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize::new(width.into(), height.into());
            let _: () = msg_send![&*self.objc, setMinSize:size];
        }
    }

    /// Used for setting a toolbar on this window.
    pub fn set_toolbar<TC: ToolbarDelegate>(&self, toolbar: &Toolbar<TC>) {
        unsafe {
            let _: () = msg_send![&*self.objc, setToolbar:&*toolbar.objc];
        }
    }

    /// Toggles whether the toolbar is shown for this window. Has no effect if no toolbar exists on
    /// this window.
    pub fn toggle_toolbar_shown(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, toggleToolbarShown:nil];
        }
    }

    /// Set whether the toolbar toggle button is shown. Has no effect if no toolbar exists on this
    /// window.
    pub fn set_shows_toolbar_button(&self, shows: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setShowsToolbarButton:match shows {
                true => YES,
                false => NO
            }];
        }
    }

    /// Given a view, sets it as the content view for this window.
    pub fn set_content_view<L: Layout + 'static>(&self, view: &L) {
        view.with_backing_obj_mut(|backing_node| unsafe {
            let _: () = msg_send![&*self.objc, setContentView:&*backing_node];
        });
    }

    /// Given a view, sets it as the content view controller for this window.
    pub fn set_content_view_controller<VC: Controller + 'static>(&self, controller: &VC) {
        let backing_node = controller.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, setContentViewController:&*backing_node];
        }
    }

    /// Shows the window.
    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyAndOrderFront:nil];
        }
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    pub fn close(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, close];
        }
    }

    /// Toggles a Window being full screen or not.
    pub fn toggle_full_screen(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, toggleFullScreen:nil];
        }
    }

    /// Sets the background color for the window. You generally don't want to do this often.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();

        unsafe {
            let _: () = msg_send![&*self.objc, setBackgroundColor:color];
        }
    }

    /// Returns whether this window is opaque or not.
    pub fn is_opaque(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, isOpaque]
        })
    }

    /// Returns whether this window is miniaturized or not.
    pub fn is_miniaturized(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, isMiniaturized]
        })
    }

    /// Miniaturize this window.
    pub fn miniaturize(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, miniaturize];
        }
    }

    /// De-mimizes this window.
    pub fn deminiaturize(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, deminiaturize];
        }
    }

    /// Runs the print panel, and if the user does anything except cancel, prints the window and
    /// its contents.
    pub fn print(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, print];
        }
    }

    /// Indicates whether the window is on a currently active space.
    ///
    /// From Apple's documentation:
    ///
    /// _The value of this property is YES if the window is on the currently active space; otherwise, NO.
    /// For visible windows, this property indicates whether the window is currently visible on the active
    /// space. For nonvisible windows, it indicates whether ordering the window onscreen would cause it to
    /// be on the active space._
    pub fn is_on_active_space(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, isOnActiveSpace]
        })
    }

    /// Returns whether this window is visible or not.
    pub fn is_visible(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, isVisible]
        })
    }

    /// Returns whether this window is the key or not.
    pub fn is_key(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, isKeyWindow]
        })
    }

    /// Returns whether this window can become the key window.
    pub fn can_become_key(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, canBecomeKeyWindow]
        })
    }

    /// Make this window the key window.
    pub fn make_key_window(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyWindow];
        }
    }

    /// Make the this window the key window and bring it to the front. Calling `show` does this for
    /// you.
    pub fn make_key_and_order_front(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyAndOrderFront:nil];
        }
    }

    /// Returns if this is the main window or not.
    pub fn is_main_window(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, isMainWindow]
        })
    }

    /// Returns if this can become the main window.
    pub fn can_become_main_window(&self) -> bool {
        to_bool(unsafe {
            msg_send![&*self.objc, canBecomeMainWindow]
        })
    }

    /// Set whether this window should be excluded from the top-level "Windows" menu.
    pub fn set_excluded_from_windows_menu(&self, excluded: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setExcludedFromWindowsMenu:match excluded {
                true => YES,
                false => NO
            }];
        }
    }

    /// Sets the separator style for this window.
    pub fn set_titlebar_separator_style(&self, style: crate::foundation::NSInteger) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitlebarSeparatorStyle:style];
        }
    }

    /// Returns the backing scale (e.g, `1.0` for non retina, `2.0` for retina) used on this
    /// window.
    ///
    /// Note that Apple recommends AGAINST using this in most cases. It's exposed here for the rare
    /// cases where you DO need it.
    pub fn backing_scale_factor(&self) -> f64 {
        unsafe {
            let scale: CGFloat = msg_send![&*self.objc, backingScaleFactor];
            scale as f64
        }
    }

    /// Given a window and callback handler, will run it as a "sheet" (model-ish) and then run the
    /// handler once the sheet is dismissed.
    ///
    /// This is a bit awkward due to Rust semantics; you have to use the same type of Window as the
    /// one you're presenting on, but in practice this isn't too bad since you rarely want a Window
    /// without a WindowDelegate.
    pub fn begin_sheet<F, W>(&self, window: &Window<W>, completion: F)
    where
        F: Fn() + Send + Sync + 'static,
        W: WindowDelegate + 'static
    {
        let block = ConcreteBlock::new(move |_response: NSInteger| {
            completion();
        });
        let block = block.copy();

        unsafe {
            let _: () = msg_send![&*self.objc, beginSheet:&*window.objc completionHandler:block];
        }
    }

    /// Closes a sheet.
    pub fn end_sheet<W>(&self, window: &Window<W>)
    where
        W: WindowDelegate + 'static
    {
        unsafe {
            let _: () = msg_send![&*self.objc, endSheet:&*window.objc];
        }
    }
}

impl<T> Drop for Window<T> {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    ///
    /// Note that only the originating `Window<T>` carries the delegate, and we
    /// intentionally don't provide this when cloning it as a handler. This ensures that we only
    /// release the backing Window when the original `Window<T>` is dropped.
    ///
    /// Well, theoretically.
    fn drop(&mut self) {
        if self.delegate.is_some() {
            unsafe {
                // Break the delegate - this shouldn't be an issue, but we should strive to be safe
                // here anyway.
                let _: () = msg_send![&*self.objc, setDelegate:nil];
            }
        }
    }
}
