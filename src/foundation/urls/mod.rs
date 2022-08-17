use std::error::Error;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, nil, NSData, NSString, NSUInteger};

mod bookmark_options;
pub use bookmark_options::{NSURLBookmarkCreationOption, NSURLBookmarkResolutionOption};

mod resource_keys;
pub use resource_keys::{NSURLFileResource, NSURLResourceKey, NSUbiquitousItemDownloadingStatus};

/// Wraps `NSURL` for use throughout the framework.
///
/// This type may also be returned to users in some callbacks (e.g, file manager/selectors) as it's
/// a core part of the macOS/iOS experience and bridging around it is arguably blocking people from
/// being able to actually build useful things.
///
/// For pure-Rust developers who have no interest in the Objective-C underpinnings, there's a
/// `pathbuf()` method that returns an `std::path::PathBuf` for working with. Note that this will
/// prove less useful in sandboxed applications, and if the underlying file that the PathBuf points
/// to moves, you'll be responsible for figuring out exactly what you do there.
///
/// Otherwise, this struct bridges enough of NSURL to be useful (loading, using, and bookmarks).
/// Pull requests for additional functionality are welcome.
#[derive(Clone, Debug)]
pub struct NSURL<'a> {
    /// A reference to the backing `NSURL`.
    pub objc: ShareId<Object>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> NSURL<'a> {
    /// In cases where we're vended an `NSURL` by the system, this can be used to wrap and
    /// retain it.
    pub fn retain(object: id) -> Self {
        NSURL {
            objc: unsafe { ShareId::from_ptr(object) },
            phantom: PhantomData,
        }
    }

    /// In some cases, we want to wrap a system-provided NSURL without retaining it.
    pub fn from_retained(object: id) -> Self {
        NSURL {
            objc: unsafe { ShareId::from_retained_ptr(object) },
            phantom: PhantomData,
        }
    }

    /// Creates and returns a URL object by calling through to `[NSURL URLWithString]`.
    pub fn with_str(url: &str) -> Self {
        let url = NSString::new(url);

        Self {
            objc: unsafe { ShareId::from_ptr(msg_send![class!(NSURL), URLWithString:&*url]) },

            phantom: PhantomData,
        }
    }

    /// Returns the absolute string path that this URL points to.
    ///
    /// Note that if the underlying file moved, this won't be accurate - you likely want to
    /// research URL bookmarks.
    pub fn absolute_string(&self) -> String {
        let abs_str = NSString::retain(unsafe { msg_send![&*self.objc, absoluteString] });

        abs_str.to_string()
    }

    /// Creates and returns a Rust `PathBuf`, for users who don't need the extra pieces of NSURL
    /// and just want to write Rust code.
    pub fn pathbuf(&self) -> PathBuf {
        let path = NSString::retain(unsafe { msg_send![&*self.objc, path] });

        path.to_str().into()
    }

    /// Returns bookmark data for this URL. Will error if the underlying API errors.
    ///
    /// Bookmarks are useful for sandboxed applications, as well as situations where you might want
    /// to later resolve the true location of a file (e.g, if the user moved it between when you
    /// got the URL and when you need to use it).
    pub fn bookmark_data(
        &self,
        options: &[NSURLBookmarkCreationOption],
        resource_value_keys: &[NSURLResourceKey],
        relative_to_url: Option<NSURL>,
    ) -> Result<NSData, Box<dyn Error>> {
        let mut opts: NSUInteger = 0;
        for mask in options {
            let i: NSUInteger = mask.into();
            opts = opts | i;
        }

        // Build NSArray of resource keys
        let resource_keys = nil;

        // Mutability woes mean we just go through a match here to satisfy message passing needs.
        let bookmark_data = NSData::retain(match relative_to_url {
            Some(relative_url) => unsafe {
                msg_send![&*self.objc, bookmarkDataWithOptions:opts
                    includingResourceValuesForKeys:resource_keys
                    relativeToURL:relative_url
                    error:nil
                ]
            },

            None => unsafe {
                msg_send![&*self.objc, bookmarkDataWithOptions:opts
                    includingResourceValuesForKeys:resource_keys
                    relativeToURL:nil
                    error:nil
                ]
            },
        });

        // Check for errors...
        //Err("LOL".into())

        Ok(bookmark_data)
    }

    /// Converts bookmark data into a URL.
    pub fn from_bookmark_data(
        data: NSData,
        options: &[NSURLBookmarkResolutionOption],
        relative_to_url: Option<NSURL>,
        data_is_stale: bool,
    ) -> Result<Self, Box<dyn Error>> {
        Err("LOL".into())
    }

    /// In an app that has adopted App Sandbox, makes the resource pointed to by a security-scoped URL available to the app.
    ///
    /// More information can be found at:
    /// [https://developer.apple.com/documentation/foundation/nsurl/1417051-startaccessingsecurityscopedreso?language=objc]
    pub fn start_accessing_security_scoped_resource(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, startAccessingSecurityScopedResource];
        }
    }

    /// In an app that adopts App Sandbox, revokes access to the resource pointed to by a security-scoped URL.
    ///
    /// More information can be found at:
    /// [https://developer.apple.com/documentation/foundation/nsurl/1413736-stopaccessingsecurityscopedresou?language=objc]
    pub fn stop_accessing_security_scoped_resource(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, stopAccessingSecurityScopedResource];
        }
    }
}

/*impl From<NSString<'_>> for id {
    /// Consumes and returns the pointer to the underlying NSString instance.
    fn from(mut string: NSString) -> Self {
        &mut *string.objc
    }
}*/

impl Deref for NSURL<'_> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.objc
    }
}
