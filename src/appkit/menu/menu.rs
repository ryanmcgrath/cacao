//! Wraps NSMenu and handles instrumenting necessary delegate pieces.

use std::sync::{Arc, Mutex};

use objc_id::{Id, ShareId};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSInteger, NSString};
use crate::appkit::menu::item::MenuItem;

/// A struct that represents an `NSMenu`. It takes ownership of items, and handles instrumenting
/// them throughout the application lifecycle.
#[derive(Debug)]
pub struct Menu(pub Id<Object>);

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
        Menu(unsafe {
            let cls = class!(NSMenu);
            let alloc: id = msg_send![cls, alloc];
            let title = NSString::new(title);
            let menu: id = msg_send![alloc, initWithTitle:&*title];

            for item in items.into_iter() {
                let objc = item.to_objc();
                let _: () = msg_send![menu, addItem:&*objc];
            }

            Id::from_retained_ptr(menu)
        })
    }

    /// Given a set of `MenuItem`s, merges them into an existing Menu (e.g, for a context menu on a
    /// view).
    pub fn append(menu: id, items: Vec<MenuItem>) -> id {
        // You might look at the code below and wonder why we can't just call `removeAllItems`.
        //
        // Basically: that doesn't seem to properly decrement the retain count on the underlying
        // menu item, and we wind up leaking any callbacks for the returned `MenuItem` instances.
        //
        // Walking them and calling release after removing them from the underlying store gives us
        // the correct behavior.
        unsafe {
            let mut count: NSInteger = msg_send![menu, numberOfItems];

            while count != 0 {
                count -= 1;
                let item: id = msg_send![menu, itemAtIndex:count];
                let _: () = msg_send![menu, removeItemAtIndex:count];
                let _: () = msg_send![item, release];
            }
        }

        for item in items.into_iter() {
            unsafe {
                let objc = item.to_objc();
                let _: () = msg_send![menu, addItem:&*objc];
            }
        }

        menu
    }

    /// Convenience method for the bare-minimum NSMenu structure that "just works" for all
    /// applications, as expected.
    pub fn standard() -> Vec<Menu> {
        vec![
            Menu::new("", vec![
                MenuItem::Services,
                MenuItem::Separator,
                MenuItem::Hide,
                MenuItem::HideOthers,
                MenuItem::ShowAll,
                MenuItem::Separator,
                MenuItem::Quit
            ]),

            Menu::new("File", vec![
                MenuItem::CloseWindow
            ]),

            Menu::new("Edit", vec![
                MenuItem::Undo,
                MenuItem::Redo,
                MenuItem::Separator,
                MenuItem::Cut,
                MenuItem::Copy,
                MenuItem::Paste,
                MenuItem::Separator,
                MenuItem::SelectAll
            ]),
     
            Menu::new("View", vec![
                MenuItem::EnterFullScreen
            ]),

            Menu::new("Window", vec![
                MenuItem::Minimize,
                MenuItem::Zoom,
                MenuItem::Separator,
                MenuItem::new("Bring All to Front")
            ])
        ]
    }
}
