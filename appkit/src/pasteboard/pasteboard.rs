//! A wrapper for NSPasteBoard, which is the interface for copy/paste and general transferring
//! (think: drag and drop between applications). It exposes a Rust interface that tries to be
//! complete, but might not cover everything 100% right now - feel free to pull request.

use cocoa::base::id;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::pasteboard::types::PasteboardName;

/// Represents an `NSPasteboard`, enabling you to handle copy/paste/drag and drop.
pub struct Pasteboard {
    /// The internal pointer to the Objective-C side.
    pub inner: Id<Object>
}

impl Default for Pasteboard {
    fn default() -> Self {
        Pasteboard {
            inner: unsafe { Id::from_ptr(msg_send![class!(NSPasteboard), generalPasteboard]) }
        }
    }
}

impl Pasteboard {
    /// Used internally for wrapping a Pasteboard returned from operations (say, drag and drop).
    pub(crate) fn with(existing: id) -> Self {
        Pasteboard {
            inner: unsafe { Id::from_ptr(existing) }
        }
    }

    /// Should be pasteboardname enum!
    pub fn named(name: PasteboardName) -> Self {
        Pasteboard {
            inner: unsafe {
                let name = name.to_nsstring();
                Id::from_ptr(msg_send![class!(NSPasteboard), pasteboardWithName:name])
            }
        }
    }

    /// Creates and returns a new pasteboard with a name that is guaranteed to be unique with 
    /// respect to other pasteboards in the system.
    pub fn unique() -> Self {
        Pasteboard {
            inner: unsafe { Id::from_ptr(msg_send![class!(NSPasteboard), pasteboardWithUniqueName]) }
        }
    }

    /// Releases the receiver’s resources in the pasteboard server. It's rare-ish to need to use
    /// this, but considering this stuff happens on the Objective-C side you may need it.
    pub fn release_globally(&self) {
        unsafe {
            let _: () = msg_send![&*self.inner, releaseGlobally];
        }
    }

    /// Clears the existing contents of the pasteboard.
    pub fn clear_contents(&self) {
        unsafe {
            let _: () = msg_send![&*self.inner, clearContents];
        }
    }
}
