//! Represents settings for printing items. Backed by an `NSDictionary` in Objective-C, this struct
//! aims to make it easier to query/process printing operations.

use objc::rc::{Id, Shared};
use objc::runtime::Object;

use crate::foundation::id;

/// `PrintSettings` represents options used in printing, typically passed to you by the
/// application/user.
#[derive(Clone, Debug)]
pub struct PrintSettings {
    pub inner: Id<Object, Shared>
}

impl PrintSettings {
    /// Internal method, constructs a wrapper around the backing `NSDictionary` print settings.
    pub(crate) fn with_inner(inner: id) -> Self {
        PrintSettings {
            inner: unsafe { Id::retain(inner).unwrap() }
        }
    }
}
