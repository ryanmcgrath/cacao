//! Implements an NSToolbar, which is one of those macOS niceties
//! that makes it feel... "proper".
//!
//! UNFORTUNATELY, this is a very old and janky API. So... yeah.

use std::fmt;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, nil, NSString, NSUInteger, NO, YES};

mod class;
use class::register_toolbar_class;

mod item;
pub use item::ToolbarItem;

mod traits;
pub use traits::ToolbarDelegate;

mod enums;
pub use enums::{ItemIdentifier, ToolbarDisplayMode, ToolbarSizeMode};

pub(crate) static TOOLBAR_PTR: &str = "cacaoToolbarPtr";

/// A wrapper for `NSToolbar`. Holds (retains) pointers for the Objective-C runtime
/// where our `NSToolbar` and associated delegate live.
pub struct Toolbar<T = ()> {
    /// An internal identifier used by the toolbar. We cache it here in case users want it.
    pub identifier: String,

    /// The Objective-C runtime toolbar.
    pub objc: ShareId<Object>,

    /// A pointer to the underlying delegate.
    pub objc_delegate: ShareId<Object>,

    /// The user supplied delegate.
    pub delegate: Option<Box<T>>,
}

impl<T> Toolbar<T>
where
    T: ToolbarDelegate + 'static,
{
    /// Creates a new `NSToolbar` instance, configures it appropriately, sets up the delegate
    /// chain, and retains it all.
    pub fn new<S: Into<String>>(identifier: S, delegate: T) -> Self {
        let identifier = identifier.into();
        let cls = register_toolbar_class::<T>(&delegate);
        let mut delegate = Box::new(delegate);

        let (objc, objc_delegate) = unsafe {
            let alloc: id = msg_send![class!(NSToolbar), alloc];
            let identifier = NSString::new(&identifier);
            let toolbar: id = msg_send![alloc, initWithIdentifier: identifier];
            let objc_delegate: id = msg_send![cls, new]; //WithIdentifier:identifier];

            let ptr: *const T = &*delegate;
            (&mut *objc_delegate).set_ivar(TOOLBAR_PTR, ptr as usize);
            let _: () = msg_send![toolbar, setDelegate: objc_delegate];

            (ShareId::from_ptr(toolbar), ShareId::from_ptr(objc_delegate))
        };

        let _ret = &mut delegate.did_load(Toolbar {
            objc: objc.clone(),
            objc_delegate: objc_delegate.clone(),
            identifier: identifier.clone(),
            delegate: None,
        });

        Toolbar {
            identifier,
            objc,
            objc_delegate,
            delegate: Some(delegate),
        }
    }
}

impl<T> Toolbar<T> {
    /// Indicates whether the toolbar shows the separator between the toolbar and the main window
    /// contents.
    pub fn set_shows_baseline_separator(&self, shows: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setShowsBaselineSeparator:match shows {
                true => YES,
                false => NO
            }];
        }
    }

    /// Sets the toolbar's display mode.
    pub fn set_display_mode(&self, mode: ToolbarDisplayMode) {
        let mode: NSUInteger = mode.into();

        unsafe {
            let _: () = msg_send![&*self.objc, setDisplayMode: mode];
        }
    }

    /// Sets the toolbar's size mode.
    pub fn set_size_mode(&self, mode: ToolbarSizeMode) {
        let mode: NSUInteger = mode.into();

        unsafe {
            let _: () = msg_send![&*self.objc, setSizeMode: mode];
        }
    }

    /// Set whether the toolbar is visible or not.
    pub fn set_visible(&self, visibility: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setVisible:match visibility {
                true => YES,
                false => NO
            }];
        }
    }

    /// Sets the item represented by the item identifier to be selected.
    pub fn set_selected(&self, item_identifier: &str) {
        let identifier = NSString::new(item_identifier);

        unsafe {
            let _: () = msg_send![&*self.objc, setSelectedItemIdentifier:&*identifier];
        }
    }
}

impl<T> fmt::Debug for Toolbar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delegate = match &self.delegate {
            Some(d) => format!("Some({:p})", d),
            None => "None".to_string(),
        };

        f.debug_struct("Toolbar")
            .field("identifier", &self.identifier)
            .field("objc", &self.objc)
            .field("objc_delegate", &self.objc_delegate)
            .field("delegate", &delegate)
            .finish()
    }
}

impl<T> Drop for Toolbar<T> {
    /// A bit of extra cleanup for the delegate system. If we have a non-`None` delegate, this is
    /// the OG Toolbar and should be cleaned up for any possible cyclical references.
    fn drop(&mut self) {
        if self.delegate.is_some() {
            unsafe {
                let _: () = msg_send![&*self.objc, setDelegate: nil];
            }
        }
    }
}
