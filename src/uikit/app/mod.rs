//! Wraps the application lifecycle across platforms.
//!
//! This is where the bulk of your application logic starts out from. macOS and iOS are driven
//! heavily by lifecycle events - in this case, your boilerplate would look something like this:
//!
//! ```rust,no_run
//! use cacao::ios::app::{App, AppDelegate};
//! use cacao::window::Window;
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
//!     App::new(BasicApp::default()).run();
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

use libc::{c_char, c_int};
use std::ffi::CString;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, AutoReleasePool, NSString, NSUInteger, NO, YES};
use crate::notification_center::Dispatcher;
use crate::uikit::scene::{register_window_scene_delegate_class, WindowSceneDelegate};
use crate::utils::activate_cocoa_multithreading;

mod class;
use class::register_app_class;

mod delegate;
use delegate::register_app_delegate_class;

mod enums;
pub use enums::*;

mod traits;
pub use traits::AppDelegate;

pub(crate) static mut APP_DELEGATE: usize = 0;
pub(crate) static mut SCENE_DELEGATE_VENDOR: usize = 0;

extern "C" {
    /// Required for iOS applications to initialize.
    fn UIApplicationMain(argc: c_int, argv: *const *const c_char, principal_class_name: id, delegate_class_name: id);
}

/// A handler to make some boilerplate less annoying.
#[inline]
fn shared_application<F: Fn(id)>(handler: F) {
    let app: id = unsafe { msg_send![register_app_class(), sharedApplication] };
    handler(app);
}

/// Wraps `UIApplication` and associated lifecycle pieces.
///
/// Handles setting up a few necessary pieces:
///
/// - It injects an `NSObject` subclass to act as a delegate for lifecycle events.
/// - It ensures that Cocoa, where appropriate, is operating in multi-threaded mode so POSIX
/// threads work as intended.
pub struct App<T = (), W = (), F = ()> {
    pub delegate: Box<T>,
    pub vendor: Box<F>,
    pub pool: AutoReleasePool,
    _w: std::marker::PhantomData<W>
}

// Temporary. ;P
impl<W, T, F> std::fmt::Debug for App<W, T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App<W, T, F>").finish()
    }
}

impl<T, W, F> App<T, W, F>
where
    T: AppDelegate + 'static,
    W: WindowSceneDelegate,
    F: Fn() -> Box<W>
{
    /// iOS manages creating a new Application (`UIApplication`) differently than you'd expect if
    /// you were looking at the macOS side of things.
    ///
    /// In this case, we're primarily concerned with shoving our `AppDelegate` to a place we can
    /// retrieve it later on. While this is unsafe behavior, it's ultimately no different than
    /// shoving the pointer onto the delegate like we do on the macOS side of things.
    ///
    /// Note that this pattern is only fine here due to the fact that there can only be one
    /// AppDelegate at a time.
    ///
    /// This also handles ensuring that our subclasses exist in the Objective-C runtime *before*
    /// `UIApplicationMain` is called.
    pub fn new(delegate: T, scene_delegate_vendor: F) -> Self {
        activate_cocoa_multithreading();

        let pool = AutoReleasePool::new();
        let cls = register_app_class();
        let dl = register_app_delegate_class::<T>();
        let w = register_window_scene_delegate_class::<W, F>();

        let app_delegate = Box::new(delegate);
        let vendor = Box::new(scene_delegate_vendor);

        unsafe {
            let delegate_ptr: *const T = &*app_delegate;
            APP_DELEGATE = delegate_ptr as usize;

            let scene_delegate_vendor_ptr: *const F = &*vendor;
            SCENE_DELEGATE_VENDOR = scene_delegate_vendor_ptr as usize;
        }

        App {
            delegate: app_delegate,
            vendor,
            pool,
            _w: std::marker::PhantomData
        }
    }
}

impl<T, W, F> App<T, W, F>
where
    T: AppDelegate + 'static
{
    /// Handles calling through to `UIApplicationMain()`, ensuring that it's using our custom
    /// `UIApplication` and `UIApplicationDelegate` classes.
    pub fn run(&self) {
        let args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();

        let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

        let cls = register_app_class();
        let dl = register_app_delegate_class::<T>();

        let cls_name: id = unsafe { msg_send![cls, className] };
        let dl_name: id = unsafe { msg_send![dl, className] };

        unsafe {
            UIApplicationMain(c_args.len() as c_int, c_args.as_ptr(), cls_name, dl_name);
        }

        //self.pool.drain();
    }
}

impl<T, W, F> Drop for App<T, W, F> {
    fn drop(&mut self) {
        println!("DROPPING");
        //self.pool.drain();
    }
}
