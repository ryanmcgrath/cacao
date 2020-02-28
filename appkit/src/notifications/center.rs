//! Wraps UNUserNotificationCenter for macOS. Note that this uses the newer
//! `UserNotifications.framework` API, which requires that your application be properly signed.

use block::ConcreteBlock;

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

use objc::{class, msg_send, sel, sel_impl};

use crate::notifications::Notification;
use crate::utils::str_from;

#[allow(non_upper_case_globals, non_snake_case)]
pub mod NotificationAuthOption {
    pub const Badge: i32 = 1 << 0;
    pub const Sound: i32 = 1 << 1;
    pub const Alert: i32 = 1 << 2;
}

/// Acts as a central interface to the Notification Center on macOS.
pub struct NotificationCenter;

impl NotificationCenter {
    /// Requests authorization from the user to send them notifications.
    pub fn request_authorization(options: i32) {
        unsafe {
            let block = ConcreteBlock::new(|_: id, error: id| {
                let msg: id = msg_send![error, localizedDescription];
                
                let localized_description = str_from(msg); 
                if localized_description != "" {
                    println!("{:?}", localized_description);
                }
            });
            
            let center: id = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
            let _: () = msg_send![center, requestAuthorizationWithOptions:options completionHandler:block.copy()];
        }
    }

    /// Queues up a `Notification` to be displayed to the user.
    pub fn notify(notification: Notification) {
        let uuidentifier = format!("{}", uuid::Uuid::new_v4());

        unsafe {
            let identifier = NSString::alloc(nil).init_str(&uuidentifier);
            let request: id = msg_send![class!(UNNotificationRequest), requestWithIdentifier:identifier content:&*notification.inner trigger:nil];

            let center: id = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
            let _: () = msg_send![center, addNotificationRequest:request];
        }
    }

    /// Removes all notifications that have been delivered (e.g, in the notification center).
    pub fn remove_all_delivered_notifications() {
        unsafe {
            let center: id = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
            let _: () = msg_send![center, removeAllDeliveredNotifications];
        }
    }
}
