//! Wraps the application lifecycle across platforms.
//!
//! This is where the bulk of your application logic starts out from. macOS and iOS are driven
//! heavily by lifecycle events - in this case, your boilerplate would look something like this:
//!
//! ```rust,no_run
//! use cacao::appkit::{App, AppDelegate};
//! use cacao::appkit::window::Window;
//!
//! #[derive(Default)]
//! struct BasicApp;
//!
//! impl AppDelegate for BasicApp {
//!     fn did_finish_launching(&self) {
//!         // Your program in here
//!     }
//! }
//!
//! fn main() {
//!     App::new("com.my.app", BasicApp::default()).run();
//! }
//! ```
//!
//! ## Why do I need to do this?
//! A good question. Cocoa does many things for you (e.g, setting up and managing a runloop,
//! handling the view/window heirarchy, and so on). This requires certain things happen before your
//! code can safely run, which `App` in this framework does for you.
//!
//! - It ensures that the `sharedApplication` is properly initialized with your delegate.
//! - It ensures that Cocoa is put into multi-threaded mode, so standard POSIX threads work as they
//! should.
//!
//! ### Platform specificity
//! Certain lifecycle events are specific to certain platforms. Where this is the case, the
//! documentation makes every effort to note.

use std::fmt;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::id_shim::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::appkit::menu::Menu;
use crate::foundation::{id, nil, AutoReleasePool, NSUInteger, NO, YES};
use crate::invoker::TargetActionHandler;
use crate::notification_center::Dispatcher;
use crate::utils::activate_cocoa_multithreading;

//use crate::bundle::set_bundle_id;

mod class;
use class::register_app_class;

mod delegate;
use delegate::register_app_delegate_class;

mod enums;
pub use enums::*;

mod traits;
pub use traits::AppDelegate;

use super::window::Window;

pub(crate) static APP_PTR: &str = "rstAppPtr";

/// A handler to make some boilerplate less annoying.
#[inline]
pub(crate) fn shared_application<T, F: Fn(id) -> T>(handler: F) -> T {
    let app: id = unsafe { msg_send![register_app_class(), sharedApplication] };
    handler(app)
}

/// A wrapper for `NSApplication` in AppKit/Cocoa, and `UIApplication` in UIKit/Cocoa Touch.
///
/// It holds (retains) a pointer to the Objective-C runtime shared application object, as well as
/// handles setting up a few necessary pieces:
///
/// - It injects an `NSObject` subclass to act as a delegate for lifecycle events.
/// - It ensures that Cocoa, where appropriate, is operating in multi-threaded mode so POSIX
/// threads work as intended.
///
/// This also enables support for dispatching a message, `M`. Your `AppDelegate` can optionally
/// implement the `Dispatcher` trait to receive messages that you might dispatch from deeper in the
/// application.
pub struct App<T = (), M = ()> {
    /// The underlying Objective-C Object.
    pub objc: Id<Object>,

    /// The underlying Objective-C Object, which in this case is a delegate that forwards to the
    /// app delegate.
    pub objc_delegate: Id<Object>,

    /// The stored `AppDelegate`.
    pub delegate: Box<T>,

    /// The main-thread AutoReleasePool. Drains on app exit.
    pub pool: AutoReleasePool,

    _message: std::marker::PhantomData<M>
}

impl<T, M> fmt::Debug for App<T, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delegate = format!("{:p}", self.delegate);

        f.debug_struct("App")
            .field("objc", &self.objc)
            .field("objc_delegate", &self.objc_delegate)
            .field("delegate", &delegate)
            .field("pool", &self.pool)
            .finish()
    }
}

impl<T> App<T> {
    /// Kicks off the NSRunLoop for the NSApplication instance. This blocks when called.
    /// If you're wondering where to go from here... you need an `AppDelegate` that implements
    /// `did_finish_launching`. :)
    pub fn run(&self) {
        unsafe {
            //let current_app: id = msg_send![class!(NSRunningApplication), currentApplication];
            let shared_app: id = msg_send![register_app_class(), sharedApplication];
            let _: () = msg_send![shared_app, run];
            self.pool.drain();
        }
    }
}

impl<T> App<T>
where
    T: AppDelegate + 'static
{
    /// Creates an NSAutoReleasePool, configures various NSApplication properties (e.g, activation
    /// policies), injects an `NSObject` delegate wrapper, and retains everything on the
    /// Objective-C side of things.
    pub fn new(_bundle_id: &str, delegate: T) -> Self {
        //set_bundle_id(bundle_id);

        activate_cocoa_multithreading();

        let pool = AutoReleasePool::new();

        let objc = unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            Id::from_ptr(app)
        };

        let app_delegate = Box::new(delegate);

        let objc_delegate = unsafe {
            let delegate_class = register_app_delegate_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
            let delegate_ptr: *const T = &*app_delegate;
            (&mut *delegate).set_ivar(APP_PTR, delegate_ptr as usize);
            let _: () = msg_send![&*objc, setDelegate: delegate];
            Id::from_ptr(delegate)
        };

        App {
            objc,
            objc_delegate,
            delegate: app_delegate,
            pool,
            _message: std::marker::PhantomData
        }
    }
}

//  This is a very basic "dispatch" mechanism. In macOS, it's critical that UI work happen on the
//  UI ("main") thread. We can hook into the standard mechanism for this by dispatching on
//  queues; in our case, we'll just offer two points - one for a background queue, and one
//  for the main queue. They automatically forward through to our registered `AppDelegate`.
//
//  One thing I don't like about GCD is that detecting incorrect thread usage has historically been
//  a bit... annoying. Here, the `Dispatcher` trait explicitly requires implementing two methods -
//  one for UI messages, and one for background messages. I think that this helps separate intent
//  on the implementation side, and makes it a bit easier to detect when a message has come in on
//  the wrong side.
//
//  This is definitely, absolutely, 100% not a performant way to do things - but at the same time,
//  ObjC and such is fast enough that for a large class of applications this is workable.
//
//  tl;dr: This is all a bit of a hack, and should go away eventually. :)
impl<T, M> App<T, M>
where
    M: Send + Sync + 'static,
    T: AppDelegate + Dispatcher<Message = M>
{
    /// Dispatches a message by grabbing the `sharedApplication`, getting ahold of the delegate,
    /// and passing back through there.
    pub fn dispatch_main(message: M) {
        let queue = dispatch::Queue::main();

        queue.exec_async(move || unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let app_delegate: id = msg_send![app, delegate];
            let delegate_ptr: usize = *(*app_delegate).get_ivar(APP_PTR);
            let delegate = delegate_ptr as *const T;
            (&*delegate).on_ui_message(message);
        });
    }

    /// Dispatches a message by grabbing the `sharedApplication`, getting ahold of the delegate,
    /// and passing back through there.
    pub fn dispatch_background(message: M) {
        let queue = dispatch::Queue::main();

        queue.exec_async(move || unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let app_delegate: id = msg_send![app, delegate];
            let delegate_ptr: usize = *(*app_delegate).get_ivar(APP_PTR);
            let delegate = delegate_ptr as *const T;
            (&*delegate).on_background_message(message);
        });
    }
}

impl App {
    /// Registers for remote notifications from APNS.
    pub fn register_for_remote_notifications() {
        shared_application(|app| unsafe {
            let _: () = msg_send![app, registerForRemoteNotifications];
        });
    }

    /// Unregisters for remote notifications from APNS.
    pub fn unregister_for_remote_notifications() {
        shared_application(|app| unsafe {
            let _: () = msg_send![app, unregisterForRemoteNotifications];
        });
    }

    /// Sets whether this application should relaunch at login.
    pub fn set_relaunch_on_login(relaunch: bool) {
        shared_application(|app| unsafe {
            if relaunch {
                let _: () = msg_send![app, enableRelaunchOnLogin];
            } else {
                let _: () = msg_send![app, disableRelaunchOnLogin];
            }
        });
    }

    /// Respond to a termination request. Generally done after returning `TerminateResponse::Later`
    /// from your trait implementation of `should_terminate()`.
    pub fn reply_to_termination_request(should_terminate: bool) {
        shared_application(|app| unsafe {
            let _: () = msg_send![app, replyToApplicationShouldTerminate:match should_terminate {
                true => YES,
                false => NO
            }];
        });
    }

    /// An optional call that you can use for certain scenarios surrounding opening/printing files.
    pub fn reply_to_open_or_print(response: AppDelegateResponse) {
        shared_application(|app| unsafe {
            let r: NSUInteger = response.into();
            let _: () = msg_send![app, replyToOpenOrPrint: r];
        });
    }

    /// Sets a set of `Menu`'s as the top level Menu for the current application. Note that behind
    /// the scenes, Cocoa/AppKit make a copy of the menu you pass in - so we don't retain it, and
    /// you shouldn't bother to either.
    pub fn set_menu(mut menus: Vec<Menu>) {
        let main_menu = unsafe {
            let menu_cls = class!(NSMenu);
            let item_cls = class!(NSMenuItem);
            let main_menu: id = msg_send![menu_cls, new];

            for menu in menus.iter_mut() {
                let item: id = msg_send![item_cls, new];
                let _: () = msg_send![item, setSubmenu:&*menu.0];
                let _: () = msg_send![main_menu, addItem: item];
            }

            main_menu
        };

        shared_application(move |app| unsafe {
            let _: () = msg_send![app, setMainMenu: main_menu];
        });
    }

    /// For nib-less applications (which, if you're here, this is) need to call the activation
    /// routines after the NSMenu has been set, otherwise it won't be interact-able without
    /// switching away from the app and then coming back.
    ///
    /// @TODO: Accept an ActivationPolicy enum or something.
    pub fn activate() {
        shared_application(|app| unsafe {
            let _: () = msg_send![app, setActivationPolicy:0];
            let current_app: id = msg_send![class!(NSRunningApplication), currentApplication];
            let _: () = msg_send![current_app, activateWithOptions:1<<1];
        });
    }

    pub fn main_window() -> Window {
        shared_application(|app| unsafe {
            let window: id = msg_send![app, mainWindow];
            Window::existing(window)
        })
    }

    /// Terminates the application, firing the requisite cleanup delegate methods in the process.
    ///
    /// This is typically called when the user chooses to quit via the App menu.
    pub fn terminate() {
        shared_application(|app| unsafe {
            let _: () = msg_send![app, terminate: nil];
        });
    }
}
