//! Implements `FileSelectPanel`, which allows the user to select files for processing and hands you
//! urls to work with. It currently doesn't implement _everything_ necessary, but it's functional
//! enough for general use.

use std::path::PathBuf;

use block::ConcreteBlock;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, YES, NO, NSInteger, NSString};
use crate::filesystem::enums::ModalResponse;

use crate::macos::window::{Window, WindowDelegate};

#[derive(Debug)]
pub struct FileSelectPanel {
    /// The internal Objective C `NSOpenPanel` instance.
    pub panel: ShareId<Object>,

    /// The internal `NSObject` that routes delegate callbacks around.
    pub delegate: ShareId<Object>,

    /// Whether the user can choose files. Defaults to `true`.
    pub can_choose_files: bool,

    /// Whether the user can choose directories. Defaults to `false`.
    pub can_choose_directories: bool,

    /// When the value of this property is true, dropping an alias on the panel or asking 
    /// for filenames or URLs returns the resolved aliases. The default value of this property 
    /// is true. When this value is false, selecting an alias returns the alias instead of the 
    /// file or directory it represents.
    pub resolves_aliases: bool,

    /// When the value of this property is true, the user may select multiple items from the 
    /// browser. Defaults to `false`.
    pub allows_multiple_selection: bool
}

impl Default for FileSelectPanel {
    fn default() -> Self {
        FileSelectPanel::new()
    }
}

impl FileSelectPanel {
    /// Creates and returns a `FileSelectPanel`, which holds pointers to the Objective C runtime for
    /// instrumenting the dialog.
    pub fn new() -> Self {
        FileSelectPanel {
            panel: unsafe {
                let cls = class!(NSOpenPanel);
                let x: id = msg_send![cls, openPanel];
                ShareId::from_ptr(x)
            },

            delegate: unsafe {
                ShareId::from_ptr(msg_send![class!(NSObject), new])
            },

            can_choose_files: true,
            can_choose_directories: false,
            resolves_aliases: true,
            allows_multiple_selection: true
        }
    }

    pub fn set_delegate(&mut self) {}

    /// Sets whether files can be chosen by the user.
    pub fn set_can_choose_files(&mut self, can_choose: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setCanChooseFiles:match can_choose {
                true => YES,
                false => NO
            }];
        }

        self.can_choose_files = can_choose;
    }

    /// Set the message text displayed in the panel.
    pub fn set_message<S: AsRef<str>>(&mut self, message: S) {
        unsafe {
            let message = NSString::new(message.as_ref());
            let _: () = msg_send![&*self.panel, setMessage:&*message];
        }
    }

    /// Sets whether the user can choose directories.
    pub fn set_can_choose_directories(&mut self, can_choose: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setCanChooseDirectories:match can_choose {
                true => YES,
                false => NO
            }];
        }

        self.can_choose_directories = can_choose;
    }

    /// Sets whether the panel resolves aliases.
    pub fn set_resolves_aliases(&mut self, resolves: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setResolvesAliases:match resolves {
                true => YES,
                false => NO
            }];
        }

        self.resolves_aliases = resolves;
    }

    /// Sets whether the panel allows multiple selections.
    pub fn set_allows_multiple_selection(&mut self, allows: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setAllowsMultipleSelection:match allows {
                true => YES,
                false => NO
            }];
        }

        self.allows_multiple_selection = allows;
    }

    /// Shows the panel as a modal. Currently sheets are not supported, but you're free (and able
    /// to) thread the Objective C calls yourself by using the panel field on this struct.
    ///
    /// Note that this clones the underlying `NSOpenPanel` pointer. This is theoretically safe as
    /// the system runs and manages that in another process, and we're still abiding by the general
    /// retain/ownership rules here.
    ///
    /// This is offered for scenarios where you don't necessarily have a Window (e.g, a shell
    /// script) or can't easily pass one to use as a sheet.
    pub fn show<F>(&self, handler: F)
    where
        F: Fn(Vec<PathBuf>) + 'static
    {
        let panel = self.panel.clone();
        let completion = ConcreteBlock::new(move |result: NSInteger| {
            let response: ModalResponse = result.into();

            handler(match response {
                ModalResponse::Ok => get_urls(&panel),
                _ => Vec::new()
            });
        });

        unsafe {
            let _: () = msg_send![&*self.panel, beginWithCompletionHandler:completion.copy()];
        }
    }

    /// Shows the panel as a modal. Currently, this method accepts `Window`s which use a delegate.
    /// If you're using a `Window` without a delegate, you may need to opt to use the `show()`
    /// method.
    ///
    /// Note that this clones the underlying `NSOpenPanel` pointer. This is theoretically safe as
    /// the system runs and manages that in another process, and we're still abiding by the general
    /// retain/ownership rules here.
    pub fn begin_sheet<T, F>(&self, window: &Window<T>, handler: F)
    where
        F: Fn(Vec<PathBuf>) + 'static
    {
        let panel = self.panel.clone();
        let completion = ConcreteBlock::new(move |result: NSInteger| {
            let response: ModalResponse = result.into();

            handler(match response {
                ModalResponse::Ok => get_urls(&panel),
                _ => Vec::new()
            });
        });

        unsafe {
            let _: () = msg_send![&*self.panel, beginSheetModalForWindow:&*window.objc completionHandler:completion.copy()];
        }
    }
}

/// Retrieves the selected URLs from the provided panel.
/// This is currently a bit ugly, but it's also not something that needs to be the best thing in
/// the world as it (ideally) shouldn't be called repeatedly in hot spots.
pub fn get_urls(panel: &Object) -> Vec<PathBuf> {
    unsafe {
        let urls: id = msg_send![&*panel, URLs];
        let count: usize = msg_send![urls, count];

        (0..count).map(|index| {
            let url: id = msg_send![urls, objectAtIndex:index];
            let path = NSString::retain(msg_send![url, path]).to_string();
            path.into()
        }).collect()
    }
}
