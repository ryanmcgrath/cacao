//! Implements `FileSavePanel`, which allows the user to select where a file should be saved.
//! It currently doesn't implement _everything_ necessary, but it's functional
//! enough for general use.

use block::ConcreteBlock;

use cocoa::base::{id, nil, YES, NO, BOOL};
use cocoa::foundation::{NSInteger, NSUInteger, NSString};

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::file_panel::enums::ModalResponse;
use crate::utils::str_from;

#[derive(Debug)]
pub struct FileSavePanel {
    /// The internal Objective C `NSOpenPanel` instance.
    pub panel: ShareId<Object>,

    /// The internal `NSObject` that routes delegate callbacks around.
    pub delegate: ShareId<Object>,

    /// Whether the user can choose files. Defaults to `true`.
    pub can_create_directories: bool
}

impl Default for FileSavePanel {
    fn default() -> Self {
        FileSavePanel::new()
    }
}

impl FileSavePanel {
    /// Creates and returns a `FileSavePanel`, which holds pointers to the Objective C runtime for
    /// instrumenting the dialog.
    pub fn new() -> Self {
        FileSavePanel {
            panel: unsafe {
                let cls = class!(NSSavePanel);
                let x: id = msg_send![cls, savePanel];
                ShareId::from_ptr(x)
            },

            delegate: unsafe {
                ShareId::from_ptr(msg_send![class!(NSObject), new])
            },

            can_create_directories: true
        }
    }

    pub fn set_delegate(&mut self) {}

    pub fn set_suggested_filename(&mut self, suggested_filename: &str) {
        unsafe {
            let filename = NSString::alloc(nil).init_str(suggested_filename);
            let _: () = msg_send![&*self.panel, setNameFieldStringValue:filename];
        }
    }

    /// Sets whether directories can be created by the user.
    pub fn set_can_create_directories(&mut self, can_create: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setCanCreateDirectories:match can_create {
                true => YES,
                false => NO
            }];
        }

        self.can_create_directories = can_create;
    }

    /// Shows the panel as a modal. Currently sheets are not supported, but you're free (and able
    /// to) thread the Objective C calls yourself by using the panel field on this struct.
    ///
    /// Note that this clones the underlying `NSOpenPanel` pointer. This is theoretically safe as
    /// the system runs and manages that in another process, and we're still abiding by the general
    /// retain/ownership rules here.
    pub fn show<F: Fn(Option<String>) + 'static>(&self, handler: F) {
        let panel = self.panel.clone();
        let completion = ConcreteBlock::new(move |_result: NSInteger| {
            //let response: ModalResponse = result.into();
            handler(get_url(&panel));
        });
        let completion = completion.copy();

        unsafe {
            let _: () = msg_send![&*self.panel, runModal];
            completion.call((1,));
            //beginWithCompletionHandler:completion.copy()];
            //let _: () = msg_send![&*self.panel, beginWithCompletionHandler:completion.copy()];
        }
    }
}

/// Retrieves the selected URLs from the provided panel.
/// This is currently a bit ugly, but it's also not something that needs to be the best thing in
/// the world as it (ideally) shouldn't be called repeatedly in hot spots.
pub fn get_url(panel: &Object) -> Option<String> {
    unsafe {
        let url: id = msg_send![&*panel, URL];
        if url == nil {
            None
        } else {
            let path: id = msg_send![url, path];
            Some(str_from(path).to_string())
        }
    }
}
