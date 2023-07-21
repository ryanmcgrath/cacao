//! A wrapper for `NSNotificationCenter`.
//!
//! With this, you can:
//!
//! - Register for notifications, both from the system or posted from your code
//! - Post your own notifications
//! - Clean up and remove your handlers
//!
//! Note that in some cases (e.g, looping) this will be much slower than if you have a handle and
//! can call through to your desired path directly. This control is provided due to the need for
//! integrating with certain aspects of the underlying Cocoa/Foundation/Kit frameworks.
//!
//! ## Example

//use std::sync::Mutex;
//use std::collections::HashMap;

use block::ConcreteBlock;
//use lazy_static::lazy_static;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::{Id, ShareId};

mod name;
pub use name::NotificationName;

mod notification;

mod traits;
pub use traits::Dispatcher;

use crate::foundation::{id, nil, NSString, Retainable};

use self::notification::Notification;

/*lazy_static! {
    pub static ref DefaultNotificationCenter: NotificationCenter = {
        NotificationCenter {
            objc: unsafe {
                ShareId::from_ptr(msg_send![class!(NSNotificationCenter), defaultCenter])
            },

            subscribers: Mutex::new(HashMap::new())
        }
    };
}*/

// Wraps a reference to an `NSNotificationCenter` instance. Currently this only supports the
// default center; in the future it should aim to support custom variants.
#[derive(Debug)]
pub struct NotificationCenter {
    pub objc: ShareId<Object>,
    // pub subscribers: Mutex<HashMap<String, Vec<Dispatcher>>>
}

impl Default for NotificationCenter {
    /// Returns a wrapper over `[NSNotificationCenter defaultCenter]`. From here you can handle
    /// observing, removing, and posting notifications.
    fn default() -> Self {
        NotificationCenter {
            objc: unsafe { ShareId::from_ptr(msg_send![class!(NSNotificationCenter), defaultCenter]) },
        }
    }
}

impl NotificationCenter {
    /// Adds an entry to the notification center to receive notifications that passed to the provided block.
    /// Corresponds to `addObserverForName:object:queue:usingBlock:`
    ///
    /// TODO: Missing `object` and `queue` properties, so this receives all notifications matching the name, and
    /// on the same thread as the notification was posted
    pub fn observe<F: Fn(Notification) -> () + Send + Sync + 'static>(&self, name: Option<NotificationName>, block: F) -> id {
        let block = ConcreteBlock::new(move |ctx| {
            let notification = Notification::retain(ctx);
            block(notification);
        });
        let block = block.copy();

        if let Some(name) = name {
            let name: NSString = name.into();

            let id: id = unsafe { msg_send![self.objc, addObserverForName: name object: nil queue: nil usingBlock: block] };

            id
        } else {
            let id: id = unsafe { msg_send![self.objc, addObserverForName: nil object: nil queue: nil usingBlock: block] };

            id
        }
    }
}

impl Retainable for NotificationCenter {
    fn retain(handle: id) -> Self {
        NotificationCenter {
            objc: unsafe { Id::from_ptr(handle) },
        }
    }

    fn from_retained(handle: id) -> Self {
        NotificationCenter {
            objc: unsafe { Id::from_retained_ptr(handle) },
        }
    }
}

/*impl NotificationCenter {
    pub fn observe<T: Dispatcher>(&self, name: &str, handler: &T) {

    }

    pub fn remove<T: Dispatcher>(&self, name: &str, handler: &T) {

    }

    pub fn post(&self, name: &str) {

    }
}*/
