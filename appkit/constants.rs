//! Constants typically used for referencing around in the Objective-C runtime.
//! Specific to this crate.

pub(crate) static APP_PTR: &str = "rstAppPtr";
pub(crate) static BACKGROUND_COLOR: &str = "rstBackgroundColor";
pub(crate) static TOOLBAR_PTR: &str = "rstToolbarPtr";
pub(crate) static VIEW_DELEGATE_PTR: &str = "rstViewDelegatePtr";

#[cfg(feature = "webview")]
pub(crate) static WEBVIEW_CONFIG_VAR: &str = "rstWebViewConfig";

#[cfg(feature = "webview")]
pub(crate) static WEBVIEW_VAR: &str = "rstWebView";

#[cfg(feature = "webview")]
pub(crate) static WEBVIEW_CONTROLLER_PTR: &str = "rstWebViewControllerPtr";
