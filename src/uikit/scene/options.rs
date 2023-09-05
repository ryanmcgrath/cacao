use crate::id_shim::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSString};

/// A wrapper for UISceneConfiguration.
///
/// Due to the way we have to implement this, you likely never need to touch this.
#[derive(Debug)]
pub struct SceneConnectionOptions(Id<Object>);

impl SceneConnectionOptions {
    pub fn with(opts: id) -> Self {
        SceneConnectionOptions(unsafe { Id::from_ptr(opts) })
    }

    /// Consumes and returns the underlying `UISceneConfiguration`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
