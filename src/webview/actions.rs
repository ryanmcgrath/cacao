//! Implements wrappers around `WKNavigationAction` and `WKNavigationActionPolicy`.

use objc::{msg_send, sel};

use crate::foundation::{id, NSInteger};
use crate::networking::URLRequest;
use crate::webview::enums::NavigationType;

#[derive(Debug)]
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

            request: URLRequest::with(unsafe { msg_send![action, request] })
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NavigationResponse {
    pub can_show_mime_type: bool
}

impl NavigationResponse {
    pub fn new(response: id) -> Self {
        NavigationResponse {
            can_show_mime_type: unsafe { msg_send![response, canShowMIMEType] }
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct OpenPanelParameters {
    pub allows_directories: bool,
    pub allows_multiple_selection: bool
}

impl From<id> for OpenPanelParameters {
    fn from(params: id) -> Self {
        OpenPanelParameters {
            allows_directories: unsafe { msg_send![params, allowsDirectories] },

            allows_multiple_selection: unsafe { msg_send![params, allowsMultipleSelection] }
        }
    }
}
