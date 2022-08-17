//! This module includes wrappers for `CKShare` and `CKShareMetaData`.

use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::id;

/// A wrapper for `CKShareMetaData`, which describes details about a `CKShare`. You might use this
/// to, say, handle accepting an invite for a share.
#[derive(Clone, Debug)]
pub struct CKShareMetaData {
    pub inner: ShareId<Object>,
}

impl CKShareMetaData {
    /// Internal method for wrapping a system-provided `CKShareMetaData` object.
    pub(crate) fn with_inner(object: id) -> Self {
        CKShareMetaData {
            inner: unsafe { ShareId::from_ptr(object) },
        }
    }
}
