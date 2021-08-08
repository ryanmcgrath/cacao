//! A wrapper around various pieces of the iOS13+ UIScene API.
//!
//! This is required for things like having multiple instances of your app in the app switcher on
//! iPad. In general, you probably won't need to tweak this though.

use core_graphics::geometry::CGRect;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::id;
use crate::geometry::Rect;

mod delegate;
pub(crate) use delegate::*;

mod config;
pub use config::SceneConfig;

mod enums;
pub use enums::*;

mod traits;
pub use traits::*;

mod options;
pub use options::*;

mod session;
pub use session::*;

#[derive(Debug)]
pub struct Scene(pub Id<Object>);

impl Scene {
    pub fn with(scene: id) -> Self {
        Scene(unsafe {
            Id::from_ptr(scene)
        })
    }

    // This is temporary - I'm not wrapping `coordinateSpace` until I'm happy with the ergonomics
    // of everything.
    pub fn get_bounds(&self) -> Rect {
        unsafe {
            let coordinate_space: id = msg_send![&*self.0, coordinateSpace];
            let rect: CGRect = msg_send![coordinate_space, bounds];
            rect
        }.into()
    }

    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
