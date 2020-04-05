use crate::foundation::{id, NSString};

/// Represents the types of sessions a Scene is for.
#[derive(Clone, Copy, Debug)]
pub enum SessionRole {
    /// The scene displays interactive windows on the device's main screen.
    Application,

    /// Noninteractive windows on an external display.
    ExternalDisplay,

    // Interactive content on a CarPlay screen.
    //CarPlayApplication
}

impl From<SessionRole> for NSString {
    fn from(role: SessionRole) -> Self {
        NSString::new(match role {
            SessionRole::Application => "UIWindowSceneSessionRoleApplication",
            SessionRole::ExternalDisplay => "UIWindowSceneSessionRoleExternalDisplay",
            //SessionRole::CarPlayApplication => ""
        })
    }
}

impl From<NSString> for SessionRole {
    fn from(value: NSString) -> Self {
        match value.to_str() {
            "UIWindowSceneSessionRoleApplication" => SessionRole::Application,
            "UIWindowSceneSessionRoleExternalDisplay" => SessionRole::ExternalDisplay,
            _ => SessionRole::Application
        }
    }
}
