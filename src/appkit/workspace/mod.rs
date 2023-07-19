use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, NSArray, Retainable};

use self::running_application::RunningApplication;

pub mod running_application;

#[derive(Debug)]
pub struct Workspace(id);

impl Workspace {
    pub fn shared_workspace() -> Self {
        let workspace: id = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };

        Workspace(workspace)
    }

    pub fn running_applications(&self) -> Vec<RunningApplication> {
        let apps: id = unsafe { msg_send![self.0, runningApplications] };

        NSArray::retain(apps).iter().map(|a| RunningApplication::new(a)).collect()
    }
}
