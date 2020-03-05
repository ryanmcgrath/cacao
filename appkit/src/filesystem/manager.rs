//! A wrapper for `NSFileManager`, which is necessary for macOS/iOS (the sandbox makes things
//! tricky, and this transparently handles it for you).

use std::rc::Rc;
use std::cell::RefCell;

use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSInteger, NSUInteger};

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::filesystem::enums::{SearchPathDirectory, SearchPathDomainMask};
use crate::utils::str_from;

pub struct FileManagerInner {
    pub manager: Id<Object>
}

impl Default for FileManagerInner {
    fn default() -> Self {
        FileManagerInner {
            manager: unsafe {
                let manager: id = msg_send![class!(NSFileManager), defaultManager];
                Id::from_ptr(manager)
            }
        }
    }
}

impl FileManagerInner {
    pub fn get_path(&self, directory: SearchPathDirectory, in_domain: SearchPathDomainMask) -> Result<String, Box<dyn std::error::Error>> {
        let dir: NSUInteger = directory.into();
        let mask: NSUInteger = in_domain.into();

        unsafe {
            let dir: id = msg_send![&*self.manager, URLForDirectory:dir 
                inDomain:mask
                appropriateForURL:nil
                create:NO
                error:nil];

            let s: id = msg_send![dir, path];
            Ok(str_from(s).to_string())
        }
    }
}

#[derive(Default)]
pub struct FileManager(Rc<RefCell<FileManagerInner>>);

impl FileManager {
    pub fn new() -> Self {
        FileManager(Rc::new(RefCell::new(FileManagerInner {
            manager: unsafe {
                let manager: id = msg_send![class!(NSFileManager), new];
                Id::from_ptr(manager)
            }
        })))
    }

    pub fn get_path(&self, directory: SearchPathDirectory, in_domain: SearchPathDomainMask) -> Result<String, Box<dyn std::error::Error>> {
        let manager = self.0.borrow();
        manager.get_path(directory, in_domain)
    }

    //pub fn contents_of(directory: &str, properties: &[
}
