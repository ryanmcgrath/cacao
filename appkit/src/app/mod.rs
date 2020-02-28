//! A wrapper for `NSApplicationDelegate` on macOS. Handles looping back events and providing a very janky
//! messaging architecture.

use std::sync::Once;

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use cocoa::appkit::{NSRunningApplication};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::menu::Menu;

mod events;
use events::register_app_class;

static APP_PTR: &str = "rstAppPtr";

pub trait AppDelegate {
    type Message: Send + Sync;

    fn did_finish_launching(&self) {}
    fn did_become_active(&self) {}

    fn on_message(&self, message: Self::Message) {}
}

/// A wrapper for `NSApplication`. It holds (retains) pointers for the Objective-C runtime, 
/// which is where our application instance lives. It also injects an `NSObject` subclass,
/// which acts as the Delegate, looping back into our Vaulthund shared application.
pub struct App<T = (), M = ()> {
    pub inner: Id<Object>,
    pub objc_delegate: Id<Object>,
    pub delegate: Box<T>,
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

impl<T, M> App<T, M> where M: Send + Sync + 'static, T: AppDelegate<Message = M> {
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

    /// Creates an NSAutoReleasePool, configures various NSApplication properties (e.g, activation
    /// policies), injects an `NSObject` delegate wrapper, and retains everything on the
    /// Objective-C side of things.
    pub fn new(_bundle_id: &str, delegate: T) -> Self {
        // set_bundle_id(bundle_id);
        
        let _pool = unsafe {
            //msg_send![class!(
            cocoa::foundation::NSAutoreleasePool::new(nil)
        };

        let inner = unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            let _: () = msg_send![app, setActivationPolicy:0];
            //app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyRegular);
            Id::from_ptr(app)
        };
        
        let app_delegate = Box::new(delegate);

        let objc_delegate = unsafe {
            let delegate_class = register_delegate_class::<T>();
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
            _t: std::marker::PhantomData
        }
    }

    /// Kicks off the NSRunLoop for the NSApplication instance. This blocks when called.
    /// If you're wondering where to go from here... you need an `AppDelegate` that implements
    /// `did_finish_launching`. :)
    pub fn run(&self) {
        unsafe {
            let current_app = cocoa::appkit::NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(cocoa::appkit::NSApplicationActivateIgnoringOtherApps);
            let shared_app: id = msg_send![class!(RSTApplication), sharedApplication];
            let _: () = msg_send![shared_app, run];
        }
    }
}

/// Fires when the Application Delegate receives a `applicationDidFinishLaunching` notification.
extern fn did_finish_launching<D: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(APP_PTR);
        let app = app_ptr as *const D;
        (*app).did_finish_launching();
    };
}

/// Fires when the Application Delegate receives a `applicationDidBecomeActive` notification.
extern fn did_become_active<D: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(APP_PTR);
        let app = app_ptr as *const D;
        (*app).did_become_active();
    }
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
fn register_delegate_class<D: AppDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("RSTAppDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(APP_PTR);

        // Add callback methods
        decl.add_method(sel!(applicationDidFinishLaunching:), did_finish_launching::<D> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationDidBecomeActive:), did_become_active::<D> as extern fn(&Object, _, _));

        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
