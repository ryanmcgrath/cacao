use crate::foundation::{id, NSString};

/// Represents the types of sessions a Scene is for.
#[derive(Clone, Copy, Debug)]
pub enum SessionRole {
    /// The scene displays interactive windows on the device's main screen.
    Application,

    /// Noninteractive windows on an external display.
    ExternalDisplay // Interactive content on a CarPlay screen.
                    //CarPlayApplication
}

impl From<SessionRole> for NSString<'_> {
    fn from(role: SessionRole) -> Self {
        match role {
            SessionRole::Application => NSString::no_copy("UIWindowSceneSessionRoleApplication"),
            SessionRole::ExternalDisplay => NSString::no_copy("UIWindowSceneSessionRoleExternalDisplay") //SessionRole::CarPlayApplication => ""
        }
    }
}

impl From<NSString<'_>> for SessionRole {
    fn from(value: NSString<'_>) -> Self {
        match value.to_str() {
            "UIWindowSceneSessionRoleApplication" => SessionRole::Application,
            "UIWindowSceneSessionRoleExternalDisplay" => SessionRole::ExternalDisplay,
            _ => SessionRole::Application
        }
    }
}
