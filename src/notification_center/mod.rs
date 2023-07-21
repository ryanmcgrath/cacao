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
pub use notification::Notification;

mod traits;
pub use traits::Dispatcher;

use crate::foundation::{id, nil, NSString, Retainable};

// Wraps a reference to an `NSNotificationCenter` instance. Currently this only supports the
// default center; in the future it should aim to support custom variants.
#[derive(Debug)]
pub struct NotificationCenter(ShareId<Object>);

impl Default for NotificationCenter {
    /// Returns a wrapper over `[NSNotificationCenter defaultCenter]`. From here you can handle
    /// observing, removing, and posting notifications.
    fn default() -> Self {
        NotificationCenter(unsafe { ShareId::from_ptr(msg_send![class!(NSNotificationCenter), defaultCenter]) })
    }
}

impl NotificationCenter {
    /// Adds an entry to the notification center to receive notifications that passed to the provided block.
    /// Corresponds to `addObserverForName:object:queue:usingBlock:`
    ///
    /// TODO: Missing `object` and `queue` properties, so this receives all notifications matching the name, and
    /// on the same thread as the notification was posted
    pub fn observe<F: Fn(Notification) -> () + Send + Sync + 'static>(
        &self,
        name: Option<NotificationName>,
        block: F,
    ) -> NotificationObserver {
        let block = ConcreteBlock::new(move |ctx| {
            let notification = Notification::retain(ctx);
            block(notification);
        });
        let block = block.copy();

        let id: id = if let Some(name) = name {
            let name: NSString = name.into();

            unsafe { msg_send![self.0, addObserverForName: name object: nil queue: nil usingBlock: block] }
        } else {
            unsafe { msg_send![self.0, addObserverForName: nil object: nil queue: nil usingBlock: block] }
        };

        NotificationObserver::new(id, self)
    }

    /// Posts a given notification to the notification center. Corresponds to `postNotification:`
    pub fn post(&self, notification: Notification) {
        unsafe { msg_send![self.0, postNotification: notification.0] }
    }
}

impl Retainable for NotificationCenter {
    fn retain(handle: id) -> Self {
        NotificationCenter(unsafe { Id::from_ptr(handle) })
    }

    fn from_retained(handle: id) -> Self {
        NotificationCenter(unsafe { Id::from_retained_ptr(handle) })
    }
}

#[derive(Debug)]
pub struct NotificationObserver {
    objc: ShareId<Object>,
    notification_center: ShareId<Object>,
}

impl NotificationObserver {
    fn new(observer: id, notification_center: &NotificationCenter) -> Self {
        NotificationObserver {
            objc: unsafe { ShareId::from_ptr(observer) },
            notification_center: notification_center.0.clone(),
        }
    }

    /// Removes matching entries from the notification center's dispatch table. Corresponds to removeObserver:name:object:
    ///
    /// TODO: Missing object property
    pub fn remove(self, name: Option<NotificationName>) {
        if let Some(name) = name {
            let name: NSString = name.into();

            unsafe { msg_send![self.notification_center, removeObserver: &*self.objc name: name object: nil] }
        } else {
            unsafe { msg_send![self.notification_center, removeObserver: &*self.objc name: nil object: nil] }
        }
    }
}
