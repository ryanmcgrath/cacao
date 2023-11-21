//! Implements `FileSavePanel`, which allows the user to select where a file should be saved.
//! It currently doesn't implement _everything_ necessary, but it's functional
//! enough for general use.

use block::ConcreteBlock;

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, nil, NSInteger, NSString};

#[derive(Debug)]
pub struct FileSavePanel {
    /// The internal Objective C `NSOpenPanel` instance.
    pub panel: Id<Object, Shared>,

    /// The internal `NSObject` that routes delegate callbacks around.
    pub delegate: Id<Object, Shared>,

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
                msg_send_id![cls, savePanel]
            },

            delegate: unsafe { msg_send_id![class!(NSObject), new] },

            can_create_directories: true
        }
    }

    /// @TODO: Do we even need this?
    pub fn set_delegate(&mut self) {}

    /// Sets a suggested filename for the save dialog. The user can still change this if they
    /// choose to, but it's generally best practice to call this.
    pub fn set_suggested_filename<S: AsRef<str>>(&mut self, suggested_filename: S) {
        unsafe {
            let filename = NSString::new(suggested_filename.as_ref());
            let _: () = msg_send![&*self.panel, setNameFieldStringValue:&*filename];
        }
    }

    /// Set the message text displayed in the panel.
    pub fn set_message<S: AsRef<str>>(&mut self, message: S) {
        unsafe {
            let message = NSString::new(message.as_ref());
            let _: () = msg_send![&*self.panel, setMessage:&*message];
        }
    }

    /// Sets whether directories can be created by the user.
    pub fn set_can_create_directories(&mut self, can_create: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setCanCreateDirectories: can_create];
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
            //let _: () = msg_send![&*self.panel, beginWithCompletionHandler: &*completion.copy()];
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
            Some(NSString::retain(path).to_string())
        }
    }
}
