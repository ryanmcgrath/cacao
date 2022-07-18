//! Various enums used throughout the `webview` module.

use crate::foundation::NSInteger;

/// Describes a navigation type from within the `WebView`.
#[derive(Clone, Copy, Debug)]
pub enum NavigationType {
    /// A user activated a link.
    LinkActivated,

    /// A user submitted a form.
    FormSubmitted,

    /// A user went backwards or fowards.
    BackForward,

    /// A user reloaded the webview.
    Reload,

    /// A user resubmitted a form.
    FormResubmitted,

    /// Other.
    Other
}

// For whatever reason, impl From<> below doesn't generate the reciprocal impl Into<> we need.
// So I guess we'll do it ourselves.
//
// This panic will be removed and is for testing purposes only right now.
impl Into<NavigationType> for NSInteger {
    fn into(self) -> NavigationType {
        match self {
            -1 => NavigationType::Other,
            0 => NavigationType::LinkActivated,
            1 => NavigationType::FormSubmitted,
            2 => NavigationType::BackForward,
            3 => NavigationType::Reload,
            4 => NavigationType::FormResubmitted,
            _ => {
                panic!("Unsupported WKWebView NavigationType value found!");
            }
        }
    }
}

impl From<NavigationType> for NSInteger {
    fn from(nav_type: NavigationType) -> Self {
        match nav_type {
            NavigationType::Other => -1,
            NavigationType::LinkActivated => 0,
            NavigationType::FormSubmitted => 1,
            NavigationType::BackForward => 2,
            NavigationType::Reload => 3,
            NavigationType::FormResubmitted => 4
        }
    }
}

/// Describes the policy for a given navigation.
#[derive(Clone, Copy, Debug)]
pub enum NavigationPolicy {
    /// Should be canceled.
    Cancel,

    /// Allowed.
    Allow
}

impl From<NavigationPolicy> for NSInteger {
    fn from(policy: NavigationPolicy) -> Self {
        match policy {
            NavigationPolicy::Cancel => 0,
            NavigationPolicy::Allow => 1
        }
    }
}

/// Describes a response policy for a given navigation.
#[derive(Clone, Copy, Debug)]
pub enum NavigationResponsePolicy {
    /// Should be canceled.
    Cancel,

    /// Allowed.
    Allow,

    /// This is a private API, and likely won't make it into the App Store. Will only be available
    /// if you opt in via the `webview-downloading` feature.
    #[cfg(feature = "webview-downloading-macos")]
    BecomeDownload
}

impl From<NavigationResponsePolicy> for NSInteger {
    fn from(policy: NavigationResponsePolicy) -> Self {
        match policy {
            NavigationResponsePolicy::Cancel => 0,
            NavigationResponsePolicy::Allow => 1,

            #[cfg(feature = "webview-downloading-macos")]
            NavigationResponsePolicy::BecomeDownload => 2
        }
    }
}

/// Dictates where a given user script should be injected.
#[derive(Clone, Copy, Debug)]
pub enum InjectAt {
    /// Inject at the start of the document.
    Start = 0,

    /// Inject at the end of the document.
    End = 1
}

impl From<InjectAt> for NSInteger {
    fn from(at: InjectAt) -> Self {
        match at {
            InjectAt::Start => 0,
            InjectAt::End => 1
        }
    }
}
