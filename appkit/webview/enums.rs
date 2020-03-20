//! Various enums used throughout the `webview` module.

use crate::foundation::NSInteger;

/// Describes a navigation type from within the `WebView`.
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

impl From<NSInteger> for NavigationType {
    fn from(i: NSInteger) -> Self {
        match i {
            -1 => NavigationType::Other,
            0 => NavigationType::LinkActivated,
            1 => NavigationType::FormSubmitted,
            2 => NavigationType::BackForward,
            3 => NavigationType::Reload,
            4 => NavigationType::FormResubmitted,
            e => { panic!("Unsupported navigation type: {}", e); }
        }
    }
}

/// Describes the policy for a given navigation.
pub enum NavigationPolicy {
    /// Should be canceled.
    Cancel,

    /// Allowed.
    Allow
}

impl From<NavigationPolicy> for NSInteger {
    fn into(self) -> Self {
        match self {
            NavigationPolicy::Cancel => 0,
            NavigationPolicy::Allow => 1
        }
    }
}

/// Describes a response policy for a given navigation.
pub enum NavigationResponsePolicy {
    /// Should be canceled.
    Cancel,

    /// Allowed.
    Allow,

    /// This is a private API, and likely won't make it into the App Store. Will only be available
    /// if you opt in via the `webview-downloading` feature.
    #[cfg(feature = "webview-downloading")]
    BecomeDownload
}

impl From<NavigationResponsePolicy> for NSInteger {
    fn into(self) -> Self {
        match self {
            NavigationResponsePolicy::Cancel => 0,
            NavigationResponsePolicy::Allow => 1,
            
            #[cfg(feature = "webview-downloading")]
            NavigationResponsePolicy::BecomeDownload => 2
        }
    }
}

/// Dictates where a given user script should be injected.
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
