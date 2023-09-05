use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{msg_send, msg_send_id, sel};

use crate::foundation::{id, NSString};
use crate::uikit::scene::enums::SessionRole;

#[derive(Debug)]
pub struct SceneSession(pub Id<Object, Owned>);

impl SceneSession {
    pub fn with(session: id) -> Self {
        SceneSession(unsafe { Id::retain(session).unwrap() })
    }

    pub fn role(&self) -> SessionRole {
        NSString::from_id(unsafe { msg_send_id![&*self.0, role] }).into()
    }
}
