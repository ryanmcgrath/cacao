//! Wraps UNUserNotificationCenter for macOS. Note that this uses the newer
//! `UserNotifications.framework` API, which requires that your application be properly signed.
//!
//! To use this module, you must specify the `user-notifications` feature flag in your
//! `Cargo.toml`.

use block::ConcreteBlock;

use objc::{class, msg_send, sel, sel_impl};
use uuid::Uuid;

use crate::foundation::{id, nil, NSString, NSUInteger};

pub mod enums;
pub use enums::NotificationAuthOption;

pub mod notifications;
pub use notifications::Notification;

/// Acts as a central interface to the Notification Center on macOS.
#[derive(Debug)]
pub struct NotificationCenter;

impl NotificationCenter {
    /// Requests authorization from the user to send them notifications.
    pub fn request_authorization(options: &[NotificationAuthOption]) {
        unsafe {
            // @TODO: Revisit.
            let block = ConcreteBlock::new(|_: id, error: id| {
                let localized_description = NSString::new(msg_send![error, localizedDescription]);
                let e = localized_description.to_str();
                if e != "" {
                    println!("{:?}", e);
                }
            });

            let mut opts: NSUInteger = 0;
            for opt in options {
                let o: NSUInteger = opt.into();
                opts = opts << o;
            }

            let center: id = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
            let _: () = msg_send![center, requestAuthorizationWithOptions:opts completionHandler:block.copy()];
        }
    }

    /// Queues up a `Notification` to be displayed to the user.
    pub fn notify(notification: Notification) {
        let uuidentifier = format!("{}", Uuid::new_v4());

        unsafe {
            let identifier = NSString::new(&uuidentifier);
            let request: id = msg_send![class!(UNNotificationRequest), requestWithIdentifier:identifier content:&*notification.0 trigger:nil];
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
