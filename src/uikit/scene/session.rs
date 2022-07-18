use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, NSString};
use crate::uikit::scene::enums::SessionRole;

#[derive(Debug)]
pub struct SceneSession(pub Id<Object>);

impl SceneSession {
    pub fn with(session: id) -> Self {
        SceneSession(unsafe { Id::from_ptr(session) })
    }

    pub fn role(&self) -> SessionRole {
        NSString::from_retained(unsafe { msg_send![&*self.0, role] }).into()
    }
}
