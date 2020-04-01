//! Wraps the application lifecycle across platforms.
//!
//! This is where the bulk of your application logic starts out from. macOS and iOS are driven
//! heavily by lifecycle events - in this case, your boilerplate would look something like this:
//!
//! ```rust,no_run
//! use cacao::app::{App, AppDelegate};
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

use libc::{c_char, c_int};
use std::ffi::CString;

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSString, NSUInteger, AutoReleasePool};
use crate::notification_center::Dispatcher;

mod class;
use class::register_app_class;

mod delegate;
use delegate::{register_app_delegate_class};

mod enums;
pub use enums::*;

mod traits;
pub use traits::AppDelegate;

pub(crate) static APP_PTR: &str = "rstAppPtr";
pub(crate) static mut APP_DELEGATE: usize = 0;

//#[link(name="UIApplicationMain")]
extern "C" {
    fn UIApplicationMain(
        argc: c_int,
        argv: *const *const c_char,
        principal_class_name: id,
        delegate_class_name: id
    );
}

/// A handler to make some boilerplate less annoying.
#[inline]
fn shared_application<F: Fn(id)>(handler: F) {
    let app: id = unsafe { msg_send![register_app_class(), sharedApplication] };
    handler(app);
}

/// A helper method for ensuring that Cocoa is running in multi-threaded mode.
///
/// Why do we need this? According to Apple, if you're going to make use of standard POSIX threads,
/// you need to, before creating and using a POSIX thread, first create and immediately detach a
/// `NSThread`. This ensures that Cocoa utilizes proper locking in certain places where it might
/// not be doing so for performance reasons.
///
/// In general, you should aim to just start all of your work inside of your `AppDelegate` methods.
/// There are some cases where you might want to do things before that, though - and if you spawn a
/// thread there, just call this first... otherwise you may have some unexpected issues later on.
///
/// _(This is called inside the `App::new()` construct for you already, so as long as you're doing
/// nothing before your `AppDelegate`, you can pay this no mind)._
pub fn activate_cocoa_multithreading() {
    unsafe {
        let thread: id = msg_send![class!(NSThread), new];
        let _: () = msg_send![thread, start];
    }
}

/// A wrapper for `NSApplication` on macOS, and `UIApplication` on iOS.
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
    //pub inner: Id<Object>,
    //pub objc_delegate: Id<Object>,
    pub delegate: Box<T>,
    pub pool: AutoReleasePool,
    _t: std::marker::PhantomData<M>
}

impl<T> App<T> {  
    /// Kicks off the NSRunLoop for the NSApplication instance. This blocks when called.
    /// If you're wondering where to go from here... you need an `AppDelegate` that implements
    /// `did_finish_launching`. :)
    pub fn run(&self) {
        unsafe {
            // create a vector of zero terminated strings
            let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
            
            // convert the strings to raw pointers
            let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
            
            let s = NSString::new("RSTApplication");
            let s2 = NSString::new("RSTAppDelegate");
            UIApplicationMain(c_args.len() as c_int, c_args.as_ptr(), s.into_inner(), s2.into_inner());
            self.pool.drain();
        }
    }
}

impl<T> App<T> where T: AppDelegate + 'static {
    /// Creates an NSAutoReleasePool, configures various NSApplication properties (e.g, activation
    /// policies), injects an `NSObject` delegate wrapper, and retains everything on the
    /// Objective-C side of things.
    pub fn new(_bundle_id: &str, delegate: T) -> Self {
        // set_bundle_id(bundle_id);
        
        activate_cocoa_multithreading();
        
        let pool = AutoReleasePool::new();
        let cls = register_app_class();
        let dl = register_app_delegate_class::<T>();
        let app_delegate = Box::new(delegate);
        let delegate_ptr: *const T = &*app_delegate;
        
        unsafe {
            APP_DELEGATE = delegate_ptr as usize;
        }
        //(&mut *delegate).set_ivar(APP_PTR, delegate_ptr as usize);
 

        /*let inner = unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let _: () = msg_send![app, setActivationPolicy:0];
            println!("1");
            Id::from_ptr(app)
        };
        
        let objc_delegate = unsafe {
            let delegate_class = register_app_delegate_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
           let _: () = msg_send![&*inner, setDelegate:delegate];
            println!("2");
            Id::from_ptr(delegate)
        };*/

        App {
            delegate: app_delegate,
            pool: pool,
            _t: std::marker::PhantomData
        }
    }
} 

// This is a hack and should be replaced with an actual messaging pipeline at some point. :)
impl<T, M> App<T, M> where M: Send + Sync + 'static, T: AppDelegate + Dispatcher<Message = M> {
    /// Dispatches a message by grabbing the `sharedApplication`, getting ahold of the delegate,
    /// and passing back through there. All messages are currently dispatched on the main thread.
    pub fn dispatch(message: M) {
        /*
        let queue = dispatch::Queue::main();
        
        queue.exec_async(move || unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let app_delegate: id = msg_send![app, delegate];
            let delegate_ptr: usize = *(*app_delegate).get_ivar(APP_PTR);
            let delegate = delegate_ptr as *const T;
            (&*delegate).on_message(message);
        });*/
    }
}
