//! A module wrapping `NSUserActivity`.
//!
//! This is primarily used in handling app handoff between devices.

use crate::id_shim::ShareId;
use objc::runtime::Object;

use crate::foundation::id;

/// Represents an `NSUserActivity`, which acts as a lightweight method to capture
/// the state of your app.
#[derive(Debug)]
pub struct UserActivity(pub ShareId<Object>);

impl UserActivity {
    /// An internal method for wrapping a system-provided activity.
    pub(crate) fn with_inner(object: id) -> Self {
        UserActivity(unsafe { ShareId::from_ptr(object) })
    }
}
