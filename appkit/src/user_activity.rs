//! A module wrapping `NSUserActivity`.

use cocoa::base::id;
use objc::runtime::Object;
use objc_id::ShareId;

/// Represents an `NSUserActivity`, which acts as a lightweight method to capture the state of your
/// app. 
pub struct UserActivity {
    pub inner: ShareId<Object>
}

impl UserActivity {
    /// An internal method for wrapping a system-provided activity.
    pub(crate) fn with_inner(object: id) -> Self {
        UserActivity {
            inner: unsafe { ShareId::from_ptr(object) }
        }
    }
}
