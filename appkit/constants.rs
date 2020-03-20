//! Constants typically used for referencing around in the Objective-C runtime.
//! Specific to this crate.

pub(crate) static TOOLBAR_PTR: &str = "rstToolbarPtr";

#[cfg(feature = "webview")]
pub(crate) static WEBVIEW_CONFIG_VAR: &str = "rstWebViewConfig";

#[cfg(feature = "webview")]
pub(crate) static WEBVIEW_VAR: &str = "rstWebView";

#[cfg(feature = "webview")]
pub(crate) static WEBVIEW_CONTROLLER_PTR: &str = "rstWebViewControllerPtr";
