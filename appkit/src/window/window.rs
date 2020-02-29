//! Implements an `NSWindow` wrapper for MacOS, backed by Cocoa and associated widgets. This also handles looping back
//! lifecycle events, such as window resizing or close events.

use std::rc::Rc;
use std::cell::RefCell;

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::NSString;

use objc_id::Id;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::{ViewController, ViewWrapper};
use crate::toolbar::{Toolbar, ToolbarDelegate};
use crate::window::WindowController;
use crate::window::controller::{register_window_controller_class};

static WINDOW_CONTROLLER_PTR: &str = "rstWindowController";

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
#[derive(Default)]
pub struct WindowInner {
    pub controller: Option<Id<Object>>,
    pub toolbar: Option<Toolbar>
}

pub enum WindowTitleVisibility {
    Visible,
    Hidden
}

impl From<WindowTitleVisibility> for usize {
    fn from(visibility: WindowTitleVisibility) -> usize {
        match visibility {
            WindowTitleVisibility::Visible => 0,
            WindowTitleVisibility::Hidden => 1
        }
    }
}

impl WindowInner {
    /// Configures the `NSWindow` to know about our delegate.
    ///
    /// Now, you may look at this and go "hey, the hell is going on here - why don't you make the
    /// `NSWindow` in `[NSWindowController loadWindow]`?
    ///
    /// This is a great question. It's because NSWindowController is... well, broken or buggy -
    /// pick a term, either works. It's optimized for loading from xib/nib files, and attempting to
    /// get loadWindow to fire properly is a pain in the rear (you're fighting a black box).
    ///
    /// This is why we do this work here, but for things subclassing `NSViewController`, we go with
    /// the route of implementing `loadView`.
    ///
    /// APPKIT!
    pub fn configure<T: WindowController + 'static>(&mut self, window_controller: &T) {
        let autosave_name = window_controller.autosave_name();
        
        let window = window_controller.config().0;

        self.controller = Some(unsafe {
            let window_controller_class = register_window_controller_class::<T>();
            let controller_alloc: id = msg_send![window_controller_class, alloc];
            let controller: id = msg_send![controller_alloc, initWithWindow:window];
            (&mut *controller).set_ivar(WINDOW_CONTROLLER_PTR, window_controller as *const T as usize);
            
            let window: id = msg_send![controller, window];
            let _: () = msg_send![window, setDelegate:controller]; 
            
            // Now we need to make sure to re-apply the NSAutoSaveName, as initWithWindow
            // strips it... for some reason. We want it applied as it does nice things like
            // save the window position in the Defaults database, which is what users expect.
            let autosave = NSString::alloc(nil).init_str(autosave_name);
            let _: () = msg_send![window, setFrameAutosaveName:autosave]; 

            Id::from_ptr(controller)
        });
    }

    /// Handles setting the title on the underlying window. Allocates and passes an `NSString` over
    /// to the Objective C runtime.
    pub fn set_title(&mut self, title: &str) {
        if let Some(controller) = &self.controller {
            unsafe {
                let title = NSString::alloc(nil).init_str(title);
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setTitle:title];
            }
        }
    }

    /// Sets the title visibility for the underlying window.
    pub fn set_title_visibility(&mut self, visibility: usize) {
        if let Some(controller) = &self.controller {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setTitleVisibility:visibility];
            }
        }
    }

    /// Used for configuring whether the window is movable via the background.
    pub fn set_movable_by_background(&self, movable: bool) {
        if let Some(controller) = &self.controller {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setMovableByWindowBackground:match movable {
                    true => YES,
                    false => NO
                }];
            }
        }
    }

    /// Used for setting whether this titlebar appears transparent.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        if let Some(controller) = &self.controller {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setTitlebarAppearsTransparent:match transparent {
                    true => YES,
                    false => NO
                }];
            }
        }
    }

    /// Used for setting a toolbar on this window. Note that this takes ownership of whatever
    /// `ToolbarDelegate` you pass! The underlying `NSToolbar` is a bit... old, and it's just
    /// easier to do things this way.
    ///
    /// If you find yourself in a position where you need your toolbar after the fact, you 
    /// probably have bigger issues.
    pub fn set_toolbar<T: ToolbarDelegate + 'static>(&mut self, identifier: &str, toolbar: T) {
        let toolbar = Toolbar::new(identifier, toolbar);
        
        if let Some(controller) = &self.controller {
            unsafe {
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setToolbar:&*toolbar.inner];
            }
        }

        self.toolbar = Some(toolbar);
    }

    /// Used for setting the content view controller for this window.
    pub fn set_content_view<T: ViewController + ViewWrapper + 'static>(&mut self, view_controller: &T) {
        if let Some(controller) = &self.controller {
            unsafe {
                if let Some(vc) = view_controller.get_handle() {
                    let _: () = msg_send![*controller, setContentViewController:&*vc];
                }
            }
        }
    }

    /// On macOS, calling `show()` is equivalent to calling `makeKeyAndOrderFront`. This is the
    /// most common use case, hence why this method was chosen - if you want or need something
    /// else, feel free to open an issue to discuss.
    ///
    /// You should never be calling this yourself, mind you - Alchemy core handles this for you.
    pub fn show(&self) {
        if let Some(controller) = &self.controller {
            unsafe {
                let _: () = msg_send![*controller, showWindow:nil];
            }
        }
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    pub fn close(&self) {
        if let Some(controller) = &self.controller {
            unsafe {
                let _: () = msg_send![*controller, close];
            }
        }
    }
}

impl Drop for WindowInner {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    fn drop(&mut self) {
        if let Some(controller) = &self.controller {
            unsafe { 
                let window: id = msg_send![*controller, window];
                let _: () = msg_send![window, setDelegate:nil];
            }
        }
    }
}

/// A Window wraps `NSWindowController`, using interior mutability to handle configuration and calling
/// through to it.
///
/// Why `NSWindowController` and not `NSWindow`, you ask? The former has lifecycle events we're
/// interested in, the latter is... well, just the window.
#[derive(Default)]
pub struct Window(Rc<RefCell<WindowInner>>);

impl Window {
    /// Sets the window title.
    pub fn set_title(&self, title: &str) {
        let mut window = self.0.borrow_mut();
        window.set_title(title);
    }

    /// Sets the window title visibility.
    pub fn set_title_visibility(&self, visibility: usize) {
        let mut window = self.0.borrow_mut();
        window.set_title_visibility(visibility);
    }

    /// Sets whether the window is movable by the background or not.
    pub fn set_movable_by_background(&self, movable: bool) {
        let window = self.0.borrow();
        window.set_movable_by_background(movable);
    }

    /// Sets whether the titlebar appears transparent or not.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        let window = self.0.borrow();
        window.set_titlebar_appears_transparent(transparent);
    }

    /// Sets the Toolbar for this window. Note that this takes ownership of the toolbar! 
    pub fn set_toolbar<T: ToolbarDelegate + 'static>(&self, identifier: &str, toolbar: T) {
        let mut window = self.0.borrow_mut();
        window.set_toolbar(identifier, toolbar);
    }

    /// Sets the content view controller for the window.
    pub fn set_content_view<T: ViewController + ViewWrapper + 'static>(&self, view: &T) {
        let mut window = self.0.borrow_mut();
        window.set_content_view(view);
    }

    /// Shows the window, running a configuration pass if necessary.
    pub fn show<T: WindowController + 'static>(&self, controller: &T) {
        let did_load = {
            let mut window = self.0.borrow_mut();
            
            if window.controller.is_none() {
                window.configure(controller);
                true
            } else {
                false
            }
        };
        
        if did_load {
            controller.did_load();
        }

        let window = self.0.borrow();
        window.show();
    }

    /// Closes the window.
    pub fn close(&self) {
        let window = self.0.borrow();
        window.close();
    }
}
