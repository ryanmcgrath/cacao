//! Traits that an implementing application can conform to. These aim to wrap the general
//! lifecycles across macOS/iOS/etc, while still conforming to a Rust-ish approach.

use url::Url;

use crate::error::AppKitError;
use crate::user_activity::UserActivity;

#[cfg(feature = "cloudkit")]
use crate::cloudkit::share::CKShareMetaData;

/// `AppDelegate` is more or less `NSApplicationDelegate` from the Objective-C/Swift side, just named
/// differently to fit in with the general naming scheme found within this framework. You can
/// implement methods from this trait in order to respond to lifecycle events that the system will
/// fire off.
pub trait AppDelegate {
    /// Fired when the application has finished launching. Unlike most other "load" lifecycle
    /// events in this framework, you don't get a reference to an app here - if you need to call
    /// through to your shared application, then used the `App::shared()` call.
    fn did_finish_launching(&self) {}
}
