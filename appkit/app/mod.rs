//! A wrapper for `NSApplicationDelegate` on macOS. Handles looping back events and providing a very janky
//! messaging architecture.

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, AutoReleasePool};
use crate::constants::APP_PTR;
use crate::menu::Menu;

mod class;
use class::register_app_class;

mod delegate;
use delegate::register_app_delegate_class;

pub mod enums;

pub mod traits;
pub use traits::{AppDelegate, Dispatcher};

/// A wrapper for `NSApplication`. It holds (retains) pointers for the Objective-C runtime, 
/// which is where our application instance lives. It also injects an `NSObject` subclass,
/// which acts as the Delegate, looping back into our Vaulthund shared application.
pub struct App<T = (), M = ()> {
    pub inner: Id<Object>,
    pub objc_delegate: Id<Object>,
    pub delegate: Box<T>,
    pub pool: AutoReleasePool,
    _t: std::marker::PhantomData<M>
}

impl App {
    /// Sets a set of `Menu`'s as the top level Menu for the current application. Note that behind
    /// the scenes, Cocoa/AppKit make a copy of the menu you pass in - so we don't retain it, and
    /// you shouldn't bother to either.
    pub fn set_menu(menus: Vec<Menu>) {
        unsafe {
            let menu_cls = class!(NSMenu);
            let main_menu: id = msg_send![menu_cls, new];

            let item_cls = class!(NSMenuItem);
            for menu in menus.iter() {
                let item: id = msg_send![item_cls, new];
                let _: () = msg_send![item, setSubmenu:&*menu.inner];
                let _: () = msg_send![main_menu, addItem:item];
            }

            let cls = class!(RSTApplication);
            let shared_app: id = msg_send![cls, sharedApplication];
            let _: () = msg_send![shared_app, setMainMenu:main_menu];
        }
    }
}

impl<T> App<T> where T: AppDelegate + 'static {
    /// Creates an NSAutoReleasePool, configures various NSApplication properties (e.g, activation
    /// policies), injects an `NSObject` delegate wrapper, and retains everything on the
    /// Objective-C side of things.
    pub fn new(_bundle_id: &str, delegate: T) -> Self {
        // set_bundle_id(bundle_id);
        
        let pool = AutoReleasePool::new();

        let inner = unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let _: () = msg_send![app, setActivationPolicy:0];
            Id::from_ptr(app)
        };
        
        let app_delegate = Box::new(delegate);

        let objc_delegate = unsafe {
            let delegate_class = register_app_delegate_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
            let delegate_ptr: *const T = &*app_delegate;
            (&mut *delegate).set_ivar(APP_PTR, delegate_ptr as usize);
            let _: () = msg_send![&*inner, setDelegate:delegate];
            Id::from_ptr(delegate)
        };

        App {
            objc_delegate: objc_delegate,
            inner: inner,
            delegate: app_delegate,
            pool: pool,
            _t: std::marker::PhantomData
        }
    }
    
    /// Kicks off the NSRunLoop for the NSApplication instance. This blocks when called.
    /// If you're wondering where to go from here... you need an `AppDelegate` that implements
    /// `did_finish_launching`. :)
    pub fn run(&self) {
        unsafe {
            let current_app: id = msg_send![class!(NSRunningApplication), currentApplication];
            let _: () = msg_send![current_app, activateWithOptions:1<<1];
            let shared_app: id = msg_send![class!(RSTApplication), sharedApplication];
            let _: () = msg_send![shared_app, run];
            self.pool.drain();
        }
    }
} 

impl<T, M> App<T, M> where M: Send + Sync + 'static, T: AppDelegate + Dispatcher<Message = M> {
    /// Dispatches a message by grabbing the `sharedApplication`, getting ahold of the delegate,
    /// and passing back through there. All messages are currently dispatched on the main thread.
    pub fn dispatch(message: M) {
        let queue = dispatch::Queue::main();
        
        queue.exec_async(move || unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let app_delegate: id = msg_send![app, delegate];
            let delegate_ptr: usize = *(*app_delegate).get_ivar(APP_PTR);
            let delegate = delegate_ptr as *const T;
            (&*delegate).on_message(message);
        });
    }
}
