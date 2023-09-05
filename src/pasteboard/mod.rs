//! A wrapper for NSPasteBoard, which is the interface for copy/paste and general transferring
//! (think: drag and drop between applications). It exposes a Rust interface that tries to be
//! complete, but might not cover everything 100% right now - feel free to pull request.
//!
//! ## Example
//! ```rust,no_run
//! use cacao::pasteboard::Pasteboard;
//!
//! // Get the default system pasteboard
//! let pasteboard = Pasteboard::default();
//!
//! // Copy a piece of text to the clipboard
//! pasteboard.copy_text("My message here");
//! ```

use std::path::PathBuf;

use crate::id_shim::ShareId;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use url::Url;

use crate::error::Error;
use crate::foundation::{id, nil, NSArray, NSString, NSURL};

mod types;
pub use types::{PasteboardName, PasteboardType};

/// Represents an `NSPasteboard`, enabling you to handle copy/paste/drag and drop.
#[derive(Debug)]
pub struct Pasteboard(pub ShareId<Object>);

impl Default for Pasteboard {
    /// Returns the default system pasteboard (the "general" pasteboard).
    fn default() -> Self {
        Pasteboard(unsafe { ShareId::from_ptr(msg_send![class!(NSPasteboard), generalPasteboard]) })
    }
}

impl Pasteboard {
    /// Used internally for wrapping a Pasteboard returned from operations (say, drag and drop).
    pub(crate) fn with(existing: id) -> Self {
        Pasteboard(unsafe { ShareId::from_ptr(existing) })
    }

    /// Retrieves the system Pasteboard for the given name/type.
    pub fn named(name: PasteboardName) -> Self {
        Pasteboard(unsafe {
            let name: NSString = name.into();
            ShareId::from_ptr(msg_send![class!(NSPasteboard), pasteboardWithName:&*name])
        })
    }

    /// Creates and returns a new pasteboard with a name that is guaranteed to be unique with
    /// respect to other pasteboards in the system.
    pub fn unique() -> Self {
        Pasteboard(unsafe { ShareId::from_ptr(msg_send![class!(NSPasteboard), pasteboardWithUniqueName]) })
    }

    /// A shorthand helper method for copying some text to the clipboard.
    pub fn copy_text<S: AsRef<str>>(&self, text: S) {
        let contents = NSString::new(text.as_ref());
        let ptype: NSString = PasteboardType::String.into();

        unsafe {
            let _: () = msg_send![&*self.0, setString:&*contents forType:ptype];
        }
    }

    /// Releases the receiver’s resources in the pasteboard server. It's rare-ish to need to use
    /// this, but considering this stuff happens on the Objective-C side you may need it.
    pub fn release_globally(&self) {
        unsafe {
            let _: () = msg_send![&*self.0, releaseGlobally];
        }
    }

    /// Clears the existing contents of the pasteboard.
    pub fn clear_contents(&self) {
        unsafe {
            let _: () = msg_send![&*self.0, clearContents];
        }
    }

    /// Looks inside the pasteboard contents and extracts what FileURLs are there, if any.
    ///
    /// _Note that this method returns a list of `Url` entities, in an attempt to be closer to how
    /// Cocoa & co operate. This method may go away in the future if it's determined that people
    /// wind up just using `get_file_paths()`._
    pub fn get_file_urls(&self) -> Result<Vec<NSURL>, Box<dyn std::error::Error>> {
        unsafe {
            let class: id = msg_send![class!(NSURL), class];
            let classes = NSArray::new(&[class]);
            let contents: id = msg_send![&*self.0, readObjectsForClasses:classes options:nil];

            // This can happen if the Pasteboard server has an error in returning items.
            // In our case, we'll bubble up an error by checking the pasteboard.
            if contents == nil {
                // This error is not necessarily "correct", but in the event of an error in
                // Pasteboard server retrieval I'm not sure where to check... and this stuff is
                // kinda ancient and has conflicting docs in places. ;P
                return Err(Box::new(Error {
                    code: 666,
                    domain: "com.cacao-rs.pasteboard".to_string(),
                    description: "Pasteboard server returned no data.".to_string()
                }));
            }

            let urls = NSArray::retain(contents).iter().map(|url| NSURL::retain(url)).collect();

            Ok(urls)
        }
    }
}
