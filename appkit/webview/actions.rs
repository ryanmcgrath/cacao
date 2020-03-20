//! Implements wrappers around `WKNavigationAction` and `WKNavigationActionPolicy`.

use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, BOOL, YES, NO, NSInteger};
use crate::networking::URLRequest;

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

            request: URLRequest::with(unsafe {
                msg_send![action, request]
            })
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
            can_show_mime_type: unsafe {
                let can_show: BOOL = msg_send![response, canShowMIMEType];
                if can_show == YES { true } else { false }
            }
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
