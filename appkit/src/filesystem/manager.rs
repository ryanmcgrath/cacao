//! A wrapper for `NSFileManager`, which is necessary for macOS/iOS (the sandbox makes things
//! tricky, and this transparently handles it for you).

use std::error::Error;
use std::sync::RwLock;

use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSString, NSUInteger};
use objc_id::Id;
use objc::runtime::{BOOL, Object};
use objc::{class, msg_send, sel, sel_impl};
use url::Url;

use crate::error::AppKitError;
use crate::filesystem::enums::{SearchPathDirectory, SearchPathDomainMask};
use crate::utils::str_from;

pub struct FileManager {
    pub manager: RwLock<Id<Object>>
}

impl Default for FileManager {
    /// Returns a default file manager, which maps to the default system file manager. For common
    /// and simple tasks, with no callbacks, you might want this.
    fn default() -> Self {
        FileManager {
            manager: RwLock::new(unsafe {
                let manager: id = msg_send![class!(NSFileManager), defaultManager];
                Id::from_ptr(manager)
            })
        }
    }
}

impl FileManager {
    /// Returns a new FileManager that opts in to delegate methods.
    pub fn new() -> Self {
        FileManager {
            manager: RwLock::new(unsafe {
                let manager: id = msg_send![class!(NSFileManager), new];
                Id::from_ptr(manager)
            })
        }
    }

    /// Given a directory/domain combination, will attempt to get the directory that matches.
    /// Returns a PathBuf that wraps the given location. If there's an error on the Objective-C
    /// side, we attempt to catch it and bubble it up.
    pub fn get_directory(&self, directory: SearchPathDirectory, in_domain: SearchPathDomainMask) -> Result<Url, Box<dyn Error>> {
        let dir: NSUInteger = directory.into();
        let mask: NSUInteger = in_domain.into();

        let directory = unsafe {
            let manager = self.manager.read().unwrap();
            let dir: id = msg_send![&**manager, URLForDirectory:dir 
                inDomain:mask
                appropriateForURL:nil
                create:NO
                error:nil];

            let s: id = msg_send![dir, absoluteString];
            str_from(s)
        };
        
        Url::parse(directory).map_err(|e| e.into())
    }

    /// Given two paths, moves file (`from`) to the location specified in `to`. This can result in
    /// an error on the Objective-C side, which we attempt to handle and bubble up as a result if
    /// so.
    pub fn move_item(&self, from: Url, to: Url) -> Result<(), Box<dyn Error>> {
        unsafe {
            let s = NSString::alloc(nil).init_str(from.as_str());
            let from_url: id = msg_send![class!(NSURL), URLWithString:s];

            let s2 = NSString::alloc(nil).init_str(to.as_str());
            let to_url: id = msg_send![class!(NSURL), URLWithString:s2];

            // This should potentially be write(), but the backing class handles this logic
            // already, so... going to leave it as read.
            let manager = self.manager.read().unwrap();

            let error: id = nil;
            let result: BOOL = msg_send![&**manager, moveItemAtURL:from_url toURL:to_url error:&error];
            if result == NO {
                return Err(AppKitError::new(error));
            }
        }

        Ok(())
    }
}
