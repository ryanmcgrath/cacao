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
//!
//! // Set file url to the clipboard
//! pasteboard.set_files(vec!["/bin/ls".parse().unwrap(), "/bin/cat".parse().unwrap()]).unwrap();
//! ```

use std::path::PathBuf;

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};
use url::Url;

use crate::error::Error;
use crate::foundation::{id, nil, NSArray, NSString, NSURL};

mod types;
pub use types::{PasteboardName, PasteboardType};

/// Represents an `NSPasteboard`, enabling you to handle copy/paste/drag and drop.
#[derive(Debug)]
pub struct Pasteboard(pub Id<Object, Shared>);

impl Default for Pasteboard {
    /// Returns the default system pasteboard (the "general" pasteboard).
    fn default() -> Self {
        Pasteboard(unsafe { msg_send_id![class!(NSPasteboard), generalPasteboard] })
    }
}

impl Pasteboard {
    /// Used internally for wrapping a Pasteboard returned from operations (say, drag and drop).
    pub(crate) fn with(existing: id) -> Self {
        Pasteboard(unsafe { Id::retain(existing).unwrap() })
    }

    /// Retrieves the system Pasteboard for the given name/type.
    pub fn named(name: PasteboardName) -> Self {
        Pasteboard(unsafe {
            let name: NSString = name.into();
            msg_send_id![class!(NSPasteboard), pasteboardWithName:&*name]
        })
    }

    /// Creates and returns a new pasteboard with a name that is guaranteed to be unique with
    /// respect to other pasteboards in the system.
    pub fn unique() -> Self {
        Pasteboard(unsafe { msg_send_id![class!(NSPasteboard), pasteboardWithUniqueName] })
    }

    /// A shorthand helper method for copying some text to the clipboard.
    pub fn copy_text<S: AsRef<str>>(&self, text: S) {
        let contents = NSString::new(text.as_ref());
        let ptype: NSString = PasteboardType::String.into();

        unsafe {
            let _: () = msg_send![&*self.0, setString: &*contents, forType: &*ptype];
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
            let contents: id = msg_send![&*self.0, readObjectsForClasses: &*classes, options: nil];

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

    /// Write a list of path to the pasteboard
    pub fn set_files(&self, mut paths: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let url_arr: NSArray = paths
                .iter_mut()
                .map(|path| {
                    let url_str = format!("file://{}", path.display());
                    let url = NSURL::with_str(&url_str);
                    url.objc.autorelease_return()
                })
                .collect::<Vec<id>>()
                .into();

            let _: id = msg_send![&*self.0, clearContents];
            let succ: bool = msg_send![&*self.0, writeObjects: &*url_arr];

            if succ {
                Ok(())
            } else {
                Err(Box::new(Error {
                    code: 666,
                    domain: "com.cacao-rs.pasteboard".to_string(),
                    description: "Pasteboard server set urls fail.".to_string()
                }))
            }
        }
    }
}

#[cfg(test)]
mod pasteboard_test {
    use std::path::PathBuf;

    use super::Pasteboard;

    #[test]
    fn paste_files() {
        let pb = Pasteboard::unique();
        let paths: Vec<PathBuf> = vec!["/bin/ls", "/bin/cat"].into_iter().map(|s| s.parse().unwrap()).collect();

        pb.set_files(paths.clone()).unwrap();
        let urls = pb.get_file_urls().unwrap();
        let got: Vec<PathBuf> = urls.into_iter().map(|u| u.pathbuf()).collect();
        assert_eq!(got, paths);
        println!("successful!");
    }
}
