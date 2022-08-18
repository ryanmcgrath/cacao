//! Enums used in notifications - e.g, for customizing registration or appearance.

use crate::foundation::NSUInteger;

#[derive(Debug)]
pub enum NotificationAuthOption {
    Badge,
    Sound,
    Alert
}

impl From<NotificationAuthOption> for NSUInteger {
    fn from(option: NotificationAuthOption) -> Self {
        match option {
            NotificationAuthOption::Badge => 1 << 0,
            NotificationAuthOption::Sound => 1 << 1,
            NotificationAuthOption::Alert => 1 << 2
        }
    }
}

impl From<&NotificationAuthOption> for NSUInteger {
    fn from(option: &NotificationAuthOption) -> Self {
        match option {
            NotificationAuthOption::Badge => 1 << 0,
            NotificationAuthOption::Sound => 1 << 1,
            NotificationAuthOption::Alert => 1 << 2
        }
    }
}
