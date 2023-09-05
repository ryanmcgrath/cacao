use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, sel};

use crate::foundation::{id, NSString};

/// A wrapper for UISceneConfiguration.
///
/// Due to the way we have to implement this, you likely never need to touch this.
#[derive(Debug)]
pub struct SceneConnectionOptions(Id<Object, Owned>);

impl SceneConnectionOptions {
    pub fn with(opts: id) -> Self {
        SceneConnectionOptions(unsafe { Id::retain(opts).unwrap() })
    }

    /// Consumes and returns the underlying `UISceneConfiguration`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
