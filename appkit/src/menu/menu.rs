//! Wraps NSMenu and handles instrumenting necessary delegate pieces.

use cocoa::base::{id, nil, YES};
use cocoa::foundation::NSString;

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::menu::item::MenuItem;

/// A struct that represents an `NSMenu`. It takes ownership of items, and handles instrumenting
/// them throughout the application lifecycle.
#[derive(Debug)]
pub struct Menu {
    pub inner: Id<Object>,
    pub items: Vec<MenuItem>
}

impl Menu {
    /// Creates a new `Menu` with the given title, and uses the passed items as submenu items.
    pub fn new(title: &str, items: Vec<MenuItem>) -> Self {
        let inner = unsafe {
            let cls = class!(NSMenu);
            let alloc: id = msg_send![cls, alloc];
            let title = NSString::alloc(nil).init_str(title);
            let inner: id = msg_send![alloc, initWithTitle:title];
            Id::from_ptr(inner)
        };

        for item in items.iter() {
            match item {
                MenuItem::Action(item) => {
                    unsafe {
                        let _: () = msg_send![&*inner, addItem:item.clone()];
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
            items: items
        }
    }
}
