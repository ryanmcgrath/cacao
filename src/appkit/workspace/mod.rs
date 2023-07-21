use std::collections::HashMap;

use block::ConcreteBlock;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    color::Color,
    error::Error,
    foundation::{id, nil, NSArray, NSInteger, NSMutableDictionary, NSString, Retainable, NSURL},
    notification_center::NotificationCenter,
};

use self::running_application::RunningApplication;

pub mod running_application;

#[derive(Debug)]
pub struct Workspace(Id<Object>);

#[derive(Debug)]
pub struct GetFileSystemInfoForPathResponse {
    pub removable: bool,
    pub writable: bool,
    pub unmountable: bool,
    pub description: String,
    pub file_system_type: String,
}

impl Workspace {
    /// The shared workspace object.
    pub fn shared() -> Self {
        let workspace: id = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };

        Workspace::retain(workspace)
    }

    // Accessing the Workspace Notification Center

    /// The notification center for workspace notifications.
    pub fn notification_center(&self) -> NotificationCenter {
        let notification: id = unsafe { msg_send![self.0, notificationCenter] };

        NotificationCenter::retain(notification)
    }

    // Opening URLs

    /// Opens a URL asynchronously using the provided options.
    ///
    /// Maps to `openURL:configuration:completionHandler:`
    pub fn open_url_with_completion<F>(&self, _url: &str, _configuration: &str, _completion_handler: F)
    where
        F: FnOnce() -> (),
    {
        unimplemented!("Missing NSWorkspaceOpenConfiguration implementation. Only >=10.15");
    }

    /// Opens one or more URLs asynchronously in the specified app using the provided options.
    pub fn open_url_with_application_with_completion<F>(
        &self,
        _url: &str,
        _application_url: &str,
        _configuration: &str,
        _completion_handler: F,
    ) where
        F: FnOnce() -> (),
    {
        unimplemented!("Missing NSWorkspaceOpenConfiguration implementation. Only >=10.15");
    }

    /// Opens the location at the specified URL.
    pub fn open_url(&self, url: &str) -> bool {
        let url = NSString::new(url);

        unsafe {
            let url: id = msg_send![class!(NSURL), URLWithString:url];
            msg_send![self.0, openURL: url]
        }
    }

    // Launching and Hiding Apps

    /// Launches the app at the specified URL and asynchronously reports back on the app's status.
    pub fn open_application_at_url<F>(&self, _application_url: &str, _configuration: &str, _completion_handler: F)
    where
        F: FnOnce() -> (),
    {
        unimplemented!("Missing NSWorkspaceOpenConfiguration implementation. Only >=10.15");
    }

    /// Hides all applications other than the sender.
    ///
    /// Must be called on the main thread
    pub fn hide_other_applications(&self) {
        unsafe { msg_send![self.0, hideOtherApplications] }
    }

    /// Duplicates the specified URLS asynchronously in the same manner as the Finder.
    pub fn duplicate_urls<F>(&self, _urls: Vec<&str>, _completion_handler: Option<F>)
    where
        F: Fn(HashMap<String, String>, Error) -> () + Send + Sync + 'static,
    {
        unimplemented!("Missing ability to create NSArrays of NSURL");
        // let urls = urls
        //     .iter()
        //     .map(|u| {
        //         let mut url = NSURL::with_str(u);
        //         &mut *url as id
        //     })
        //     .collect::<Vec<id>>();
        // let urls = NSArray::new(&urls);

        // if let Some(completion_handler) = completion_handler {
        //     let block = ConcreteBlock::new(move |new_urls, error| {
        //         let new_urls = NSMutableDictionary::retain(new_urls);
        //         let new_urls = new_urls.into_hashmap(|u| NSURL::retain(u).absolute_string());

        //         let error = Error::new(error);

        //         completion_handler(new_urls, error);
        //     });

        //     let block = block.copy();

        //     unsafe { msg_send![self.0, duplicateURLs: urls completionHandler: block] }
        // } else {
        //     unsafe { msg_send![self.0, duplicateURLs: urls completionHandler: nil] }
        // }
    }

    /// Moves the specified URLs to the trash in the same manner as the Finder.
    pub fn recycle_urls<F>(&self, _urls: Vec<&str>, _completion_handler: Option<F>)
    where
        F: Fn(HashMap<String, String>, Error) -> () + Send + Sync + 'static,
    {
        unimplemented!("Missing ability to create NSArrays of NSURL");
    }

    /// Activates the Finder, and opens one or more windows selecting the specified files.
    pub fn active_file_viewer_selecting_urls(&self, _urls: Vec<&str>) {
        unimplemented!("Missing ability to create NSArrays of NSURL");
    }

    /// Selects the file at the specified path. Corresponds to `selectFile:inFileViewerRootedAtPath:`
    pub fn select_file(&self, file_path: &str, file_viewer_root_path: &str) -> bool {
        let file_path = NSString::new(file_path);
        let root_path = NSString::new(file_viewer_root_path);

        unsafe { msg_send![self.0, selectFile: file_path inFileViewerRootedAtPath: root_path] }
    }

    // Manipulating Uniform Type Identifier Information

    /// Returns the URL for the app with the specified identifier.
    pub fn url_for_application(&self, bundle_identifier: &str) -> Option<NSURL> {
        let bundle_identifier = NSString::new(bundle_identifier);

        let url: id = unsafe { msg_send![self.0, URLForApplicationWithBundleIdentifier: bundle_identifier] };
        NSURL::retain_nullable(url)
    }

    // Requesting Information

    /// Returns the URL to the default app that would be opened.
    pub fn url_for_application_to_open(&self, url: &str) -> Option<NSURL> {
        let url = NSString::new(url);

        let url: id = unsafe { msg_send![self.0, URLForApplicationToOpenURL: url] };
        NSURL::retain_nullable(url)
    }

    /// Returns information about the file system at the specified path.
    pub fn get_fs_info(&self, path: &str) -> Option<GetFileSystemInfoForPathResponse> {
        let path = NSString::new(path);

        let mut removable = Box::new(false);
        let mut writable = Box::new(false);
        let mut unmountable = Box::new(false);
        let description = NSString::new("");
        let file_system_type = NSString::new("");

        let removable_ptr = removable.as_mut() as *mut bool;
        let writable_ptr = writable.as_mut() as *mut bool;
        let unmountable_ptr = unmountable.as_mut() as *mut bool;

        let returned_data: bool = unsafe {
            msg_send![self.0, getFileSystemInfoForPath:
                path isRemovable: removable_ptr
                isWritable: writable_ptr
                isUnmountable: unmountable_ptr
                description: &description
                type: &file_system_type
            ]
        };

        if returned_data {
            Some(GetFileSystemInfoForPathResponse {
                removable: *removable,
                writable: *writable,
                unmountable: *unmountable,
                description: description.to_string(),
                file_system_type: file_system_type.to_string(),
            })
        } else {
            None
        }
    }

    /// Determines whether the specified path is a file package.
    pub fn is_path_file_package(&self, path: &str) -> bool {
        let path = NSString::new(path);

        unsafe { msg_send![self.0, isFilePackageAtPath: path] }
    }

    /// Returns the frontmost app, which is the app that receives key events.
    pub fn frontmost_application(&self) -> Option<RunningApplication> {
        let id: id = unsafe { msg_send![self.0, frontmostApplication] };

        if !id.is_null() {
            Some(RunningApplication::retain(id))
        } else {
            None
        }
    }

    /// Returns an array of running apps.
    pub fn running_applications(&self) -> Vec<RunningApplication> {
        let apps: id = unsafe { msg_send![self.0, runningApplications] };

        NSArray::retain(apps).iter().map(|a| RunningApplication::retain(a)).collect()
    }

    /// Returns the app that owns the currently displayed menu bar.
    pub fn menu_bar_owning_application(&self) -> Option<RunningApplication> {
        let id: id = unsafe { msg_send![self.0, menuBarOwningApplication] };

        if !id.is_null() {
            Some(RunningApplication::retain(id))
        } else {
            None
        }
    }

    // Managing Icons

    // TODO: Need icon methods

    // Unmounting a Device

    // TODO: Need unmount methods

    // Managing the Desktop Image

    // TODO: Need desktop image methods

    // Performing Finder Spotlight Searches

    /// Displays a Spotlight search results window in Finder for the specified query string.
    pub fn show_finder_results_for_query(&self, query: &str) -> bool {
        let query = NSString::new(query);
        unsafe { msg_send![self.0, showSearchResultsForQueryString: query] }
    }

    // Finder File Labels

    /// The array of file labels, returned as strings.
    pub fn file_labels(&self) -> Vec<String> {
        let id: id = unsafe { msg_send![self.0, fileLabels] };
        NSArray::retain(id).iter().map(|s| NSString::retain(s).to_string()).collect()
    }

    /// The array of colors for the file labels.
    pub fn file_label_colors(&self) -> Vec<Color> {
        unimplemented!("No NSColor constructor from a pointer")
    }

    // Tracking Changes to the File System

    /// Informs the workspace object that the file system changed at the specified path.
    pub fn note_fs_changed(&self, path: &str) {
        let path = NSString::new(path);
        unsafe { msg_send![self.0, noteFileSystemChanged: path] }
    }

    // Requesting Additional Time Before Logout

    /// Requests the system wait for the specified amount of time before turning off the power or logging out the user.
    pub fn extend_power_off(&self, milliseconds: i64) {
        let milliseconds = milliseconds as NSInteger;
        unsafe { msg_send![self.0, extendPowerOffBy: milliseconds] }
    }

    // Suppporting Accessibility

    // TODO: Need accessibility getter/setter methods

    // Performing Priviledged Operations

    // TODO: Needs priviledged op methods - This returns an opaque object that is passed to NSFileManager,
    // so probably not terribly important
}

impl Retainable for Workspace {
    fn retain(handle: id) -> Self {
        Workspace(unsafe { Id::from_ptr(handle) })
    }

    fn from_retained(handle: id) -> Self {
        Workspace(unsafe { Id::from_retained_ptr(handle) })
    }
}
