
use crate::webview::action::{NavigationAction, NavigationPolicy, NavigationResponse, NavigationResponsePolicy, OpenPanelParameters};

pub trait WebViewController {
    fn on_message(&self, name: &str, body: &str) {}

    fn policy_for_navigation_action<F: Fn(NavigationPolicy)>(&self, action: NavigationAction, handler: F) {
        handler(NavigationPolicy::Allow);
    }

    fn policy_for_navigation_response<F: Fn(NavigationResponsePolicy)>(&self, response: NavigationResponse, handler: F) {
        handler(NavigationResponsePolicy::Allow);
    }

    fn run_open_panel<F: Fn(Option<Vec<String>>) + 'static>(&self, parameters: OpenPanelParameters, handler: F) {
        handler(None);
    }

    fn run_save_panel<F: Fn(bool, Option<String>) + 'static>(&self, suggested_filename: &str, handler: F) {
        handler(false, None);
    }
}
