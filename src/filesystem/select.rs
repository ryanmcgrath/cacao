//! Implements `FileSelectPanel`, which allows the user to select files for processing and hands you
//! urls to work with. It currently doesn't implement _everything_ necessary, but it's functional
//! enough for general use.

use std::path::PathBuf;

use block::ConcreteBlock;

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::filesystem::enums::ModalResponse;
use crate::foundation::{id, nil, NSInteger, NSString, NSURL};

#[cfg(feature = "appkit")]
use crate::appkit::window::{Window, WindowDelegate};

#[derive(Debug)]
pub struct FileSelectPanel {
    /// The internal Objective C `NSOpenPanel` instance.
    pub panel: Id<Object, Shared>,

    /// The internal `NSObject` that routes delegate callbacks around.
    pub delegate: Id<Object, Shared>,

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
                msg_send_id![cls, openPanel]
            },

            delegate: unsafe { msg_send_id![class!(NSObject), new] },

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
            let _: () = msg_send![&*self.panel, setCanChooseFiles: can_choose];
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
            let _: () = msg_send![&*self.panel, setCanChooseDirectories: can_choose];
        }

        self.can_choose_directories = can_choose;
    }

    /// Sets whether the panel resolves aliases.
    pub fn set_resolves_aliases(&mut self, resolves: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setResolvesAliases: resolves];
        }

        self.resolves_aliases = resolves;
    }

    /// Sets whether the panel allows multiple selections.
    pub fn set_allows_multiple_selection(&mut self, allows: bool) {
        unsafe {
            let _: () = msg_send![&*self.panel, setAllowsMultipleSelection: allows];
        }

        self.allows_multiple_selection = allows;
    }

    /// Shows the panel as a modal.
    ///
    /// Note that this clones the underlying `NSOpenPanel` pointer. This is theoretically safe as
    /// the system runs and manages that in another process, and we're still abiding by the general
    /// retain/ownership rules here.
    ///
    /// This is offered for scenarios where you don't necessarily have a Window (e.g, a shell
    /// script) or can't easily pass one to use as a sheet.
    pub fn show<F>(&self, handler: F)
    where
        F: Fn(Vec<NSURL>) + 'static
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
            let _: () = msg_send![&*self.panel, beginWithCompletionHandler: &*completion.copy()];
        }
    }

    /// As panels descend behind the scenes from `NSWindow`, we can call through to close it.
    ///
    /// You should really prefer to utilize sheets to display panels; this is offered as a
    /// convenience for rare cases where you might need to retain a panel and close it later on.
    pub fn close(&self) {
        unsafe {
            let _: () = msg_send![&*self.panel, close];
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
        F: Fn(Vec<NSURL>) + 'static
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
            let _: () = msg_send![
                &*self.panel,
                beginSheetModalForWindow: &*window.objc,
                completionHandler: &*completion.copy(),
            ];
        }
    }
}

/// Retrieves the selected URLs from the provided panel.
/// This is currently a bit ugly, but it's also not something that needs to be the best thing in
/// the world as it (ideally) shouldn't be called repeatedly in hot spots.
///
/// (We mostly do this to find the sweet spot between Rust constructs and necessary Foundation
/// interaction patterns)
fn get_urls(panel: &Object) -> Vec<NSURL> {
    unsafe {
        let urls: id = msg_send![&*panel, URLs];
        let count: usize = msg_send![urls, count];

        (0..count)
            .map(|index| NSURL::retain(msg_send![urls, objectAtIndex: index]))
            .collect()
    }
}
