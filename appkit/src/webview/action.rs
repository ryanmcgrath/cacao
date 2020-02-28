//! Implements wrappers around `WKNavigationAction` and `WKNavigationActionPolicy`.

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::NSInteger;

use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{class, msg_send, sel, sel_impl};

use crate::networking::URLRequest;

pub enum NavigationType {
    LinkActivated,
    FormSubmitted,
    BackForward,
    Reload,
    FormResubmitted,
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

pub struct NavigationAction {
    pub navigation_type: NavigationType,
    pub request: URLRequest 
}

impl NavigationAction {
    pub fn new(action: id) -> Self {
        NavigationAction {
            navigation_type: unsafe {
                let nav_type: NSInteger = msg_send![action, navigationType];
                nav_type.into()
            },

            request: URLRequest::with(unsafe {
                msg_send![action, request]
            })
        }
    }
}

pub enum NavigationPolicy {
    Cancel,
    Allow
}

impl Into<NSInteger> for NavigationPolicy {
    fn into(self) -> NSInteger {
        match self {
            NavigationPolicy::Cancel => 0,
            NavigationPolicy::Allow => 1
        }
    }
}

pub struct NavigationResponse {
    pub can_show_mime_type: bool
}

impl NavigationResponse {
    pub fn new(response: id) -> Self {
        NavigationResponse {
            can_show_mime_type: unsafe {
                let canShow: BOOL = msg_send![response, canShowMIMEType];
                if canShow == YES { true } else { false }
            }
        }
    }
}

pub enum NavigationResponsePolicy {
    Cancel = 0,
    Allow = 1,

    // This is a private API!
    BecomeDownload = 2
}

impl Into<NSInteger> for NavigationResponsePolicy {
    fn into(self) -> NSInteger {
        match self {
            NavigationResponsePolicy::Cancel => 0,
            NavigationResponsePolicy::Allow => 1,
            NavigationResponsePolicy::BecomeDownload => 2
        }
    }
}

#[derive(Debug, Default)]
pub struct OpenPanelParameters {
    pub allows_directories: bool,
    pub allows_multiple_selection: bool
}

impl From<id> for OpenPanelParameters {
    fn from(params: id) -> Self {
        OpenPanelParameters {
            allows_directories: unsafe {
                match msg_send![params, allowsDirectories] {
                    YES => true,
                    NO => false,
                    _ => { panic!("Invalid value from WKOpenPanelParameters:allowsDirectories"); }
                }
            },

            allows_multiple_selection: unsafe {
                match msg_send![params, allowsMultipleSelection] {
                    YES => true,
                    NO => false,
                    _ => { panic!("Invalid value from WKOpenPanelParameters:allowsMultipleSelection"); }
                }
            }
        }
    }
}
