use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, load_or_register_class, ClassMap, NSString};

use crate::uikit::scene::SessionRole;

/// A wrapper for UISceneConfiguration.
///
/// Due to the way we have to implement this, you likely never need to touch this.
#[derive(Debug)]
pub struct SceneConfig(Id<Object, Owned>);

impl SceneConfig {
    /// Creates a new `UISceneConfiguration` with the specified name and session role, retains it,
    /// and returns it.
    pub fn new(name: &str, role: SessionRole) -> Self {
        let delegate_class = ClassMap::static_load("RSTWindowSceneDelegate", Some("UIResponder"))
            .expect("A crucial iOS step was missed - the scene delegate class is either not loaded or misnamed");

        SceneConfig(unsafe {
            let name = NSString::new(name);
            let role = NSString::from(role);

            let cls = class!(UISceneConfiguration);
            let mut config = msg_send_id![cls, configurationWithName: &*name, sessionRole: &*role];

            let _: () = msg_send![&mut config, setSceneClass: class!(UIWindowScene)];

            // TODO: use register_window_scene_delegate_class rather than load_or_register_class.
            let window_delegate = load_or_register_class("UIResponder", "RSTWindowSceneDelegate", |decl| unsafe {});
            let _: () = msg_send![&mut config, setDelegateClass: window_delegate];

            config
        })
    }

    /// Consumes and returns the underlying `UISceneConfiguration`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}
