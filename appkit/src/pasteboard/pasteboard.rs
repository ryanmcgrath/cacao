//! A wrapper for NSPasteBoard, which is the interface for copy/paste and general transferring
//! (think: drag and drop between applications). It exposes a Rust interface that tries to be
//! complete, but might not cover everything 100% right now - feel free to pull request.

use std::error::Error;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;
use url::Url;

use crate::error::AppKitError;
use crate::pasteboard::types::{PasteboardName, PasteboardType};
use crate::utils::str_from;

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

    /// Retrieves the system Pasteboard for the given name/type.
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

    /// Looks inside the pasteboard contents and extracts what FileURLs are there, if any.
    pub fn get_file_urls(&self) -> Result<Vec<Url>, Box<dyn Error>> {
        unsafe {
            let mut i = 0;
            
            let class: id = msg_send![class!(NSURL), class];
            let classes: id = NSArray::arrayWithObjects(nil, &[class]);
            let contents: id = msg_send![&*self.inner, readObjectsForClasses:classes options:nil];
            
            // This can happen if the Pasteboard server has an error in returning items.
            // In our case, we'll bubble up an error by checking the pasteboard.
            if contents == nil {
                // This error is not necessarily "correct", but in the event of an error in
                // Pasteboard server retrieval I'm not sure where to check... and this stuff is
                // kinda ancient and has conflicting docs in places. ;P
                return Err(Box::new(AppKitError {
                    code: 666,
                    domain: "com.appkit-rs.pasteboard".to_string(),
                    description: "Pasteboard server returned no data.".to_string()
                }));
            }

            let count: usize = msg_send![contents, count];
            let mut urls: Vec<Url> = Vec::with_capacity(count);
            
            loop {
                let nsurl: id = msg_send![contents, objectAtIndex:i];
                let path: id = msg_send![nsurl, path];
                let s = str_from(path);
                urls.push(Url::parse(&format!("file://{}", s))?);

                i += 1;
                if i == count { break; }
            }

            Ok(urls)
        }
    }
/*
    /// Retrieves the pasteboard contents as a string. This can be `None` (`nil` on the Objective-C
    /// side) if the pasteboard data doesn't match the requested type, so check accordingly.
    ///
    /// Note: In macOS 10.6 and later, if the receiver contains multiple items that can provide string, 
    /// RTF, or RTFD data, the text data from each item is returned as a combined result separated by newlines.
    /// This Rust wrapper is a quick pass, and could be improved. ;P
    pub fn contents_for(&self, pasteboard_type: PasteboardType) -> Option<String> {
        unsafe {
            let contents: id = msg_send![&*self.inner, stringForType:pasteboard_type.to_nsstring()];
            if contents != nil {
                return Some(str_from(contents).to_string());
            }
        }

        None
    }*/
}
