//! This module handles providing a special subclass of `NSApplication`.
//!
//! Now, I know what you're thinking: this is dumb.
//!
//! And sure, maybe. But if you've ever opened Xcode and wondered why the hell
//! you have a xib/nib in your macOS project, it's (partly) because *that* handles
//! the NSMenu architecture for you... an architecture that, supposedly, is one of the
//! last Carbon pieces still laying around.
//!
//! And I gotta be honest, I ain't about the xib/nib life. SwiftUI will hopefully clear
//! that mess up one day, but in the meantime, we'll do this.
//!
//! Now, what we're *actually* doing here is relatively plain - on certain key events,
//! we want to make sure cut/copy/paste/etc are sent down the event chain. Usually, the
//! xib/nib stuff handles this for you... but this'll mostly do the same.

use std::sync::Once;

use cocoa::base::{id, nil};

use objc::declare::ClassDecl;
use objc::runtime::Class;
use objc::{class, msg_send, sel, sel_impl};

/// Used for injecting a custom NSApplication. Currently does nothing.
pub(crate) fn register_app_class() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSApplication").unwrap();
        let decl = ClassDecl::new("RSTApplication", superclass).unwrap();
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
