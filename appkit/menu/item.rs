//! A wrapper for NSMenuItem. Currently only supports menus going
//! one level deep; this could change in the future but is fine for
//! now.

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Object, Sel};
use objc_id::ShareId;

use crate::foundation::{id, nil, NSString, NSUInteger};
use crate::events::EventModifierFlag;

/// Internal method (shorthand) for generating `NSMenuItem` holders.
fn make_menu_item(
    title: &str,
    key: Option<&str>,
    action: Option<Sel>,
    modifiers: Option<&[EventModifierFlag]>
) -> MenuItem {
    unsafe {
        let cls = class!(NSMenuItem);
        let alloc: id = msg_send![cls, alloc];
        let title = NSString::new(title);

        // Note that AppKit requires a blank string if nil, not nil.
        let key = NSString::new(match key {
            Some(s) => s,
            None => ""
        });

        let item = ShareId::from_ptr(match action {
            Some(a) => msg_send![alloc, initWithTitle:title action:a keyEquivalent:key],
            None => msg_send![alloc, initWithTitle:title action:nil keyEquivalent:key]
        });

        if let Some(modifiers) = modifiers {
            let mut key_mask: NSUInteger = 0;

            for modifier in modifiers {
                let y: NSUInteger = modifier.into();
                key_mask = key_mask | y;
            }

            let _: () = msg_send![&*item, setKeyEquivalentModifierMask:key_mask];
        }

        MenuItem::Action(item)
    }
}

/// Represents varying `NSMenuItem` types - e.g, a separator vs an action.
#[derive(Debug)]
pub enum MenuItem {
    /// Represents a Menu item that's not a separator - for all intents and purposes, you can consider
    /// this the real `NSMenuItem`.
    Action(ShareId<Object>),

    /// Represents a Separator. You can't do anything with this, but it's useful nonetheless for
    /// separating out pieces of the `NSMenu` structure.
    Separator
}

impl MenuItem {
    /// Creates and returns a `MenuItem::Action` with the specified title.
    pub fn action(title: &str) -> Self {
        make_menu_item(title, None, None, None)
    }

    /// Configures the menu item, if it's not a separator, to support a key equivalent.
    pub fn key(self, key: &str) -> Self {
        match self {
            MenuItem::Separator => MenuItem::Separator,

            MenuItem::Action(item) => {
                unsafe {
                    let key = NSString::new(key);
                    let _: () = msg_send![&*item, setKeyEquivalent:key];
                }

                MenuItem::Action(item)
            }
        }
    }
    
    /// Returns a standard "About" item.
    pub fn about(name: &str) -> Self {
        let title = format!("About {}", name);
        make_menu_item(&title, None, Some(sel!(orderFrontStandardAboutPanel:)), None)
    }
    
    /// Returns a standard "Hide" item.
    pub fn hide() -> Self {
        make_menu_item("Hide", Some("h"), Some(sel!(hide:)), None)
    }

    /// Returns the standard "Services" item. This one does some extra work to link in the default
    /// Services submenu.
    pub fn services() -> Self {
        match make_menu_item("Services", None, None, None) {
            // Link in the services menu, which is part of NSApp
            MenuItem::Action(item) => {
                unsafe {
                    let app: id = msg_send![class!(RSTApplication), sharedApplication];
                    let services: id = msg_send![app, servicesMenu];
                    let _: () = msg_send![&*item, setSubmenu:services];
                }

                MenuItem::Action(item)
            },

            // Should never be hit
            MenuItem::Separator => MenuItem::Separator
        }
    }
    
    /// Returns a standard "Hide" item.
    pub fn hide_others() -> Self {
        make_menu_item(
            "Hide Others",
            Some("h"),
            Some(sel!(hide:)),
            Some(&[EventModifierFlag::Command, EventModifierFlag::Option])
        )
    }

    /// Returns a standard "Hide" item.
    pub fn show_all() -> Self {
        make_menu_item("Show All", None, Some(sel!(unhideAllApplications:)), None)
    }

    /// Returns a standard "Close Window" item.
    pub fn close_window() -> Self {
        make_menu_item("Close Window", Some("w"), Some(sel!(performClose:)), None)
    }

    /// Returns a standard "Quit" item.
    pub fn quit() -> Self {
        make_menu_item("Quit", Some("q"), Some(sel!(terminate:)), None)
    }

    /// Returns a standard "Copy" item.
    pub fn copy() -> Self {
        make_menu_item("Copy", Some("c"), Some(sel!(copy:)), None)
    }
    
    /// Returns a standard "Undo" item.
    pub fn undo() -> Self {
        make_menu_item("Undo", Some("z"), Some(sel!(undo:)), None)
    }

    /// Returns a standard "Enter Full Screen" item
    pub fn enter_full_screen() -> Self {
        make_menu_item(
            "Enter Full Screen",
            Some("f"),
            Some(sel!(toggleFullScreen:)),
            Some(&[EventModifierFlag::Command, EventModifierFlag::Control])
        )
    }

    /// Returns a standard "Miniaturize" item
    pub fn minimize() -> Self {
        make_menu_item(
            "Minimize",
            Some("m"),
            Some(sel!(performMiniaturize:)),
            None
        )
    }

    /// Returns a standard "Zoom" item
    pub fn zoom() -> Self {
        make_menu_item(
            "Zoom",
            None,
            Some(sel!(performZoom:)),
            None
        )
    }

    /// Returns a standard "Redo" item.
    pub fn redo() -> Self {
        make_menu_item("Redo", Some("Z"), Some(sel!(redo:)), None)
    }

    /// Returns a standard "Cut" item.
    pub fn cut() -> Self {
        make_menu_item("Cut", Some("x"), Some(sel!(cut:)), None)
    }

    /// Returns a standard "Select All" item.
    pub fn select_all() -> Self {
        make_menu_item("Select All", Some("a"), Some(sel!(selectAll:)), None)
    }

    /// Returns a standard "Paste" item.
    pub fn paste() -> Self {
        make_menu_item("Paste", Some("v"), Some(sel!(paste:)), None)
    }
}
