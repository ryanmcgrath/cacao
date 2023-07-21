use std::f32::consts::E;

use bitmask_enum::bitmask;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{id, NSArray, NSInteger, NSString, NSUInteger, Retainable, NSURL},
    image::Image
};

#[derive(Debug)]
pub struct RunningApplication(Id<Object>);

impl RunningApplication {
    /// Returns the running application with the given process identifier, or `None` if no application has that pid.
    pub fn from_process_identifier(pid: usize) -> Option<Self> {
        let id: id = unsafe { msg_send![class!(NSRunningApplication), runningApplicationWithProcessIdentifier:pid] };

        if !id.is_null() {
            Some(RunningApplication::retain(id))
        } else {
            None
        }
    }

    /// Returns an array of currently running applications with the specified bundle identifier.
    pub fn from_bundle_identifier(identifier: &str) -> Vec<RunningApplication> {
        let identifier = NSString::new(identifier);
        let id: id = unsafe { msg_send![class!(NSRunningApplication), runningApplicationsWithBundleIdentifier:identifier] };

        NSArray::retain(id).iter().map(|a| RunningApplication::retain(a)).collect()
    }

    /// Returns an NSRunningApplication representing this application.
    pub fn current() -> Self {
        let id: id = unsafe { msg_send![class!(NSRunningApplication), currentApplication] };

        Self::retain(id)
    }

    // Activating applications

    /// Attempts to activate the application using the specified options.
    pub fn activate_with_options(&self, options: ApplicationActivationOptions) -> bool {
        let options = options.bits;
        unsafe { msg_send![self.0, activateWithOptions:options] }
    }

    /// Indicates whether the application is currently frontmost.
    pub fn active(&self) -> bool {
        unsafe { msg_send![self.0, isActive] }
    }

    pub fn activation_policy(&self) -> ApplicationActivationPolicy {
        let policy: NSUInteger = unsafe { msg_send![self.0, activationPolicy] };

        match policy {
            0 => ApplicationActivationPolicy::Regular,
            1 => ApplicationActivationPolicy::Accessory,
            2 => ApplicationActivationPolicy::Prohibited,

            _ => ApplicationActivationPolicy::Regular
        }
    }

    // Hiding and unhiding applications

    /// Attempts to hide or the application.
    pub fn hide(&self) {
        unsafe { msg_send![self.0, hide] }
    }

    /// Attempts to unhide or the application.
    pub fn unhide(&self) {
        unsafe { msg_send![self.0, unhide] }
    }

    /// Indicates whether the application is currently hidden.
    pub fn hidden(&self) -> bool {
        unsafe { msg_send![self.0, isHidden] }
    }

    // Application information

    /// Indicates the localized name of the application.
    pub fn localized_name(&self) -> Option<String> {
        NSString::retain_nullable(unsafe { msg_send![self.0, localizedName] }).and_then(|s| Some(s.to_string()))
    }

    /// Returns the icon for the receiver’s application.
    pub fn icon(&self) -> Option<Image> {
        let id: id = unsafe { msg_send![self.0, icon] };

        if !id.is_null() {
            Some(Image::with(id))
        } else {
            None
        }
    }

    /// Indicates the `CFBundleIdentifier` of the application.
    pub fn bundle_identifier(&self) -> Option<String> {
        NSString::retain_nullable(unsafe { msg_send![self.0, bundleIdentifier] }).and_then(|s| Some(s.to_string()))
    }

    /// Indicates the URL to the application's bundle.
    pub fn bundle_url(&self) -> Option<NSURL> {
        NSURL::retain_nullable(unsafe { msg_send![self.0, bundleURL] })
    }

    /// Indicates the executing processor architecture for the application.
    ///
    /// The returned value will be one of the constants in Mach-O Architecture in `NSBundle`.
    pub fn executable_architecture(&self) -> usize {
        let arch: NSInteger = unsafe { msg_send![self.0, executableArchitecture] };

        arch as usize
    }

    /// Indicates the URL to the application's executable.
    pub fn executable_url(&self) -> Option<NSURL> {
        NSURL::retain_nullable(unsafe { msg_send![self.0, executableURL] })
    }

    /// Indicates the date when the application was launched.
    pub fn launch_date(&self) -> Option<usize> {
        unimplemented!("Missing NSDate implementation")
    }

    /// Indicates whether the receiver’s process has finished launching.
    pub fn finished_launching(&self) -> bool {
        unsafe { msg_send![self.0, isFinishedLaunching] }
    }

    /// Indicates the process identifier (pid) of the application.
    pub fn process_identifier(&self) -> usize {
        unsafe { msg_send![self.0, processIdentifier] }
    }

    /// Returns whether the application owns the current menu bar.
    pub fn owns_menu_bar(&self) -> bool {
        unsafe { msg_send![self.0, ownsMenuBar] }
    }

    // Terminating applications

    /// Attempts to force the receiver to quit.
    pub fn force_terminate(&self) {
        unsafe { msg_send![self.0, forceTerminate] }
    }

    /// Attempts to quit the receiver normally.
    pub fn terminate(&self) {
        unsafe { msg_send![self.0, terminate] }
    }

    /// Indicates that the receiver’s application has terminated.
    pub fn terminated(&self) -> bool {
        unsafe { msg_send![self.0, isTerminated] }
    }

    /// Terminates invisibly running applications as if triggered by system memory pressure.
    pub fn terminate_automatically_terminable_applications() {
        unsafe { msg_send![class!(NSRunningApplication), terminateAutomaticallyTerminableApplications] }
    }
}

impl Retainable for RunningApplication {
    fn retain(app: id) -> Self {
        RunningApplication(unsafe { Id::from_ptr(app) })
    }

    fn from_retained(app: id) -> Self {
        RunningApplication(unsafe { Id::from_retained_ptr(app) })
    }
}

#[bitmask(u32)]
/// Flags are for `activate_with_options`
pub enum ApplicationActivationOptions {
    /// This enum option doesn't actually exist, but it provides a convenient way for Rust to set no values
    None = 0,

    /// By default, activation brings only the main and key windows forward.
    /// If you specify `ActivateAllWindows`, all of the application's windows are brought forward.
    ActivateAllWindows = 1 << 0,

    /// The application is activated regardless of the currently active app.
    ActivateIgnoringOtherApps = 1 << 1
}

/// Activation policies (used by `activationPolicy`) that control whether and how an app may be activated.
#[derive(Debug)]
pub enum ApplicationActivationPolicy {
    /// The application is an ordinary app that appears in the Dock and may have a user interface.
    Regular = 0,

    /// The application doesn’t appear in the Dock and doesn’t have a menu bar, but it may be activated
    /// programmatically or by clicking on one of its windows.
    Accessory = 1,

    /// The application doesn’t appear in the Dock and may not create windows or be activated.
    Prohibited = 2
}
