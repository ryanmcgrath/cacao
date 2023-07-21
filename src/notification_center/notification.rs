use std::collections::HashMap;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, nil, NSMutableDictionary, NSString, Retainable};

use super::NotificationName;

#[derive(Debug)]
pub struct Notification(Id<Object>);

impl Notification {
    /// Returns a notification object with a specified name, object, and user information. Corresponds to notificationWithName:object:userInfo:
    ///
    /// Due to Rust typing limitations, `user_info` is only over `String` keys and values
    pub fn new(name: NotificationName, object: Option<id>, user_info: Option<&HashMap<String, String>>) -> Self {
        let name: NSString = name.into();

        let user_info = user_info.and_then(|user_info| {
            let user_info = NSMutableDictionary::from(user_info);

            Some(user_info)
        });

        let id: id = match (object, user_info) {
            (None, None) => unsafe { msg_send![class!(NSNotification), notificationWithName: name object: nil] },
            (None, Some(user_info)) => unsafe {
                msg_send![class!(NSNotification), notificationWithName: name object: nil userInfo: user_info]
            },
            (Some(object), None) => unsafe { msg_send![class!(NSNotification), notificationWithName: name object: object] },
            (Some(object), Some(user_info)) => unsafe {
                msg_send![class!(NSNotification), notificationWithName: name object: object userInfo: user_info]
            },
        };

        Notification::retain(id)
    }

    /// The name of the notification.
    pub fn name(&self) -> NotificationName {
        let name = NSString::retain(unsafe { msg_send![self.0, name] });

        name.into()
    }

    /// The object associated with the notification.
    pub fn object(&self) -> id {
        unsafe { msg_send![self.0, object] }
    }

    /// The user information dictionary associated with the notification.
    ///
    /// Due to complexity in possible key-value pairs, this returns the entire dictionary to consumers
    pub fn user_info(&self) -> NSMutableDictionary where {
        NSMutableDictionary::retain(unsafe { msg_send![self.0, userInfo] })
    }
}

impl Retainable for Notification {
    fn retain(handle: id) -> Self {
        Notification(unsafe { Id::from_ptr(handle) })
    }

    fn from_retained(handle: id) -> Self {
        Notification(unsafe { Id::from_retained_ptr(handle) })
    }
}
