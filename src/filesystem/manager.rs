//! A wrapper for `NSFileManager`, which is necessary for macOS/iOS (the sandbox makes things
//! tricky, and this transparently handles it for you).

use std::error::Error;
use std::sync::{Arc, RwLock};

use objc::rc::{Id, Owned};
use objc::runtime::{Object, BOOL};
use objc::{class, msg_send, msg_send_id, sel};
use url::Url;

use crate::error::Error as AppKitError;
use crate::filesystem::enums::{SearchPathDirectory, SearchPathDomainMask};
use crate::foundation::{id, nil, NSString, NSUInteger, NO};

/// A FileManager can be used for file operations (moving files, etc).
///
/// If your app is not sandboxed, you can use your favorite Rust library -
/// but if you _are_ operating in the sandbox, there's a good chance you'll want to use this.
///
/// @TODO: Couldn't this just be a Id<Object, Shared>?
#[derive(Clone, Debug)]
pub struct FileManager(pub Arc<RwLock<Id<Object, Owned>>>);

impl Default for FileManager {
    /// Returns a default file manager, which maps to the default system file manager. For common
    /// and simple tasks, with no callbacks, you might want this.
    fn default() -> Self {
        FileManager(Arc::new(RwLock::new(unsafe {
            msg_send_id![class!(NSFileManager), defaultManager]
        })))
    }
}

impl FileManager {
    /// Returns a new FileManager that opts in to delegate methods.
    pub fn new() -> Self {
        FileManager(Arc::new(RwLock::new(unsafe { msg_send_id![class!(NSFileManager), new] })))
    }

    /// Given a directory/domain combination, will attempt to get the directory that matches.
    /// Returns a PathBuf that wraps the given location. If there's an error on the Objective-C
    /// side, we attempt to catch it and bubble it up.
    pub fn get_directory(&self, directory: SearchPathDirectory, in_domain: SearchPathDomainMask) -> Result<Url, Box<dyn Error>> {
        let dir: NSUInteger = directory.into();
        let mask: NSUInteger = in_domain.into();

        let directory = unsafe {
            let manager = self.0.read().unwrap();
            let dir: id = msg_send![&**manager, URLForDirectory:dir
                inDomain:mask
                appropriateForURL:nil
                create:NO
                error:nil];

            NSString::retain(msg_send![dir, absoluteString])
        };

        Url::parse(directory.to_str()).map_err(|e| e.into())
    }

    /// Given two paths, moves file (`from`) to the location specified in `to`. This can result in
    /// an error on the Objective-C side, which we attempt to handle and bubble up as a result if
    /// so.
    pub fn move_item(&self, from: Url, to: Url) -> Result<(), Box<dyn Error>> {
        let from = NSString::new(from.as_str());
        let to = NSString::new(to.as_str());

        unsafe {
            let from_url: id = msg_send![class!(NSURL), URLWithString:&*from];
            let to_url: id = msg_send![class!(NSURL), URLWithString:&*to];

            // This should potentially be write(), but the backing class handles this logic
            // already, so... going to leave it as read.
            let manager = self.0.read().unwrap();

            let error: id = nil;
            let result: BOOL = msg_send![&**manager, moveItemAtURL:from_url toURL:to_url error:&error];
            if result == NO {
                return Err(AppKitError::new(error).into());
            }
        }

        Ok(())
    }
}
