
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, nil};
use crate::utils::Controller;
use crate::window::{Window, WindowConfig, WindowDelegate, WINDOW_DELEGATE_PTR};

mod class;
use class::register_window_controller_class;

/// A `Window` represents your way of interacting with an `NSWindow`. It wraps the various moving
/// pieces to enable you to focus on reacting to lifecycle methods and doing your thing.
pub struct WindowController<T> {
    pub objc: ShareId<Object>,
    pub window: Window<T>
}

impl<T> WindowController<T> where T: WindowDelegate + 'static {
    /// Allocates and configures an `NSWindowController` in the Objective-C/Cocoa runtime that maps over
    /// to your supplied delegate.
    pub fn with(config: WindowConfig, delegate: T) -> Self {
        let mut window = Window::with(config, delegate);

        let objc = unsafe {
            let window_controller_class = register_window_controller_class::<T>();
            let controller_alloc: id = msg_send![window_controller_class, alloc];
            let controller: id = msg_send![controller_alloc, initWithWindow:&*window.objc];

            if let Some(ptr) = window.internal_callback_ptr {
                (&mut *controller).set_ivar(WINDOW_DELEGATE_PTR, ptr as usize);
            }

            ShareId::from_ptr(controller)
        };

        if let Some(window_delegate) = &mut window.delegate {
            let mut window_delegate = window_delegate.borrow_mut();
            
            (*window_delegate).did_load(Window {
                internal_callback_ptr: None,
                delegate: None,
                objc: window.objc.clone()
            });
        }

        WindowController {
            objc: objc,
            window: window
        }
    }

    /// Given a view, sets it as the content view controller for this window.
    pub fn set_content_view_controller<C: Controller + 'static>(&self, controller: &C) {
        let backing_node = controller.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, setContentViewController:&*backing_node];
        }
    }

    /// Shows the window, running a configuration pass if necessary.
    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, showWindow:nil];
        }
    }

    /// Closes the window.
    pub fn close(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, close];
        }
    }
}

/*impl<T> Drop for Window<T> {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    ///
    /// We also clean up our loopback pointer that we use for callbacks.
    fn drop(&mut self) {
        unsafe { 
                let window: id = msg_send![*objc_controller, window];
                let _: () = msg_send![window, setDelegate:nil];

            let _ = Rc::from_raw(self.internal_callback_ptr);
        }
    }
}*/
