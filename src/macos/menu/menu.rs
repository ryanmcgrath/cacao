//! Wraps NSMenu and handles instrumenting necessary delegate pieces.

use std::sync::{Arc, Mutex};

use objc_id::{Id, ShareId};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSString};
use crate::macos::menu::item::MenuItem;
use crate::invoker::TargetActionHandler;

/// A struct that represents an `NSMenu`. It takes ownership of items, and handles instrumenting
/// them throughout the application lifecycle.
#[derive(Debug)]
pub struct Menu {
    pub inner: Id<Object>,
    pub actions: Vec<TargetActionHandler>
}

impl Menu {
    /// Creates a new `Menu` with the given title, and uses the passed items as submenu items.
    ///
    /// This method effectively does three things:
    ///
    ///     - Consumes the MenuItem Vec, and pulls out handlers we need to cache
    ///     - Configures the menu items appropriately, and wires them up
    ///     - Drops the values we no longer need, and returns only what's necessary
    ///         to get the menu functioning.
    ///
    pub fn new(title: &str, items: Vec<MenuItem>) -> Self {
        let inner = unsafe {
            let cls = class!(NSMenu);
            let alloc: id = msg_send![cls, alloc];
            let title = NSString::new(title);
            let inner: id = msg_send![alloc, initWithTitle:title];
            Id::from_ptr(inner)
        };

        let mut actions = vec![];

        for item in items {
            match item {
                MenuItem::Entry((item, action)) => {
                    unsafe {
                        let _: () = msg_send![&*inner, addItem:item];
                    }

                    if action.is_some() {
                        actions.push(action.unwrap());
                    }
                },

                MenuItem::Separator => {
                    unsafe {
                        let cls = class!(NSMenuItem);
                        let separator: id = msg_send![cls, separatorItem];
                        let _: () = msg_send![&*inner, addItem:separator];
                    }
                }
            }
        }

        Menu {
            inner: inner,
            actions: actions
        }
    }
}
