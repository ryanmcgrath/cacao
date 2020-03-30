//! Traits that an implementing application can conform to. These aim to wrap the general
//! lifecycles across macOS/iOS/etc, while still conforming to a Rust-ish approach.

use url::Url;

use crate::error::AppKitError;
use crate::user_activity::UserActivity;

use crate::macos::app::enums::TerminateResponse;
use crate::macos::menu::Menu;
use crate::macos::printing::enums::PrintResponse;
use crate::macos::printing::settings::PrintSettings;

#[cfg(feature = "cloudkit")]
use crate::cloudkit::share::CKShareMetaData;

/// `AppDelegate` is more or less `NSApplicationDelegate` from the Objective-C/Swift side, just named
/// differently to fit in with the general naming scheme found within this framework. You can
/// implement methods from this trait in order to respond to lifecycle events that the system will
/// fire off.
pub trait AppDelegate {
    /// Called right before the application will finish launching. You really, probably, want to do
    /// your setup in `did_finish_launching` unless you're sure of what you're doing.
    fn will_finish_launching(&self) {}

    /// Fired when the application has finished launching. Unlike most other "load" lifecycle
    /// events in this framework, you don't get a reference to an app here - if you need to call
    /// through to your shared application, then used the `App::shared()` call.
    fn did_finish_launching(&self) {}

    /// Fired when the application is about to become active.
    fn did_become_active(&self) {}

    /// Fired when the application is about to resign active state.
    fn will_resign_active(&self) {}
    
    /// Fired when the user is going to continue an activity.
    fn will_continue_user_activity(&self, _activity_type: &str) -> bool { false }

    /// Fired when data for continuing an activity is available. Currently, the
    /// `restoration_handler` is not used, but there to communicate intent with what this API will
    /// eventually be doing.
    fn continue_user_activity<F: Fn()>(&self, _activity: UserActivity, _restoration_handler: F) -> bool { false }

    /// Fired when the activity could not be continued.
    fn failed_to_continue_user_activity(&self, _activity_type: &str, _error: AppKitError) {}

    /// Fired after the user activity object has been updated.
    fn updated_user_activity(&self, _activity: UserActivity) {}

    /// Fired when you've successfully registered for remote notifications with APNS.
    fn registered_for_remote_notifications(&self, _token: &str) {}

    /// Fired after you've received a push notification from APNS.
    //fn did_receive_remote_notification(&self, notification: PushNotification) {}

    /// Fired if there was a failure to register for remote notifications with APNS - e.g,
    /// connection issues or something.
    fn failed_to_register_for_remote_notifications(&self, _error: AppKitError) {}

    /// Fires after the user accepted a CloudKit sharing invitation associated with your
    /// application. 
    #[cfg(feature = "cloudkit")]
    fn user_accepted_cloudkit_share(&self, _share_metadata: CKShareMetaData) {}
    
    /// Fired before the application terminates. You can use this to do any required cleanup.
    fn will_terminate(&self) {}

    /// Fired immediately before the application is about to become active.
    fn will_become_active(&self) {}

    /// Fired when the application has resigned active state.
    fn did_resign_active(&self) {}

    /// Fired when the application is about to hide.
    fn will_hide(&self) {}

    /// Fired after the application has hidden.
    fn did_hide(&self) {}

    /// Fired when the application is about to unhide itself.
    fn will_unhide(&self) {}

    /// Fired after the application has unhidden itself.
    fn did_unhide(&self) {}

    /// Fired immediately before the application object updates its windows.
    fn will_update(&self) {}

    /// Fired immediately after the application object updates its windows.
    fn did_update(&self) {}

    /// This is fired after the `Quit` menu item has been selected, or after you've called `App::terminate()`.
    ///
    /// In most cases you just want `TerminateResponse::Now` here, which enables business as usual. If you need, 
    /// though, you can cancel the termination via `TerminateResponse::Cancel` to continue something essential. If
    /// you do this, you'll need to be sure to call `App::reply_to_termination_request()` to circle
    /// back.
    fn should_terminate(&self) -> TerminateResponse { TerminateResponse::Now }

    /// Sent by the application to the delegate prior to default behavior to reopen AppleEvents.
    ///
    /// `has_visible_windows` indicates whether the Application object found any visible windows in your application.
    /// You can use this value as an indication of whether the application would do anything if you return `true`.
    ///
    /// Return `true` if you want the application to perform its normal tasks, or `false` if you want the 
    /// application to do nothing. The default implementation of this method returns `true`.
    ///
    /// Some finer points of discussion, from Apple documentation:
    ///
    /// These events are sent whenever the Finder reactivates an already running application because someone 
    /// double-clicked it again or used the dock to activate it.
    ///
    /// For most document-based applications, an untitled document will be created.
    ///
    /// [Read more
    /// here](https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428638-applicationshouldhandlereopen?language=objc)
    fn should_handle_reopen(&self, _has_visible_windows: bool) -> bool { true }

    /// Supply a dock menu for the application dynamically. The default implementation for this
    /// method returns `None`, for no menu.
    fn dock_menu(&self) -> Option<Menu> { None }

    /// Fired before the application presents an error message to the user. If you find the error
    /// to be... not what you want, you can take it, alter it, and return it anew. The default
    /// implementation of this method simply returns the error as-is.
    fn will_present_error(&self, error: AppKitError) -> AppKitError { error }

    /// Fired when the screen parameters for the application have changed (e.g, the user changed
    /// something in their settings).
    fn did_change_screen_parameters(&self) {}
  
    /// Fired when you have a list of `Url`'s to open. This is best explained by quoting the Apple
    /// documentation verbatim:
    ///
    /// _"AppKit calls this method when your app is asked to open one or more URL-based resources. 
    /// You must declare the URL types that your app supports in your `Info.plist` file using the `CFBundleURLTypes` key. 
    /// The list can also include URLs for documents for which your app does not have an associated `NSDocument` class. 
    /// You configure document types by adding the `CFBundleDocumentTypes` key to your Info.plist
    /// file."
    ///
    /// Note that since we have this as the de-facto method of handling resource opens, the system
    /// will _not_ call `application:openFile:` or `application:openFiles`.
    fn open_urls(&self, _urls: Vec<Url>) { }

    /// Fired when the file is requested to be opened programmatically. This is not a commonly used
    /// or implemented method.
    ///
    /// According to Apple: 
    ///
    /// _"The method should open the file without bringing up its application’s user interface—that is, 
    /// work with the file is under programmatic control of sender, rather than under keyboard control of the user."_
    ///
    /// It's unclear how supported this is in sandbox environments, so use at your own risk.
    fn open_file_without_ui(&self, _filename: &str) -> bool { false }

    /// Fired when the application is ready and able to open a temporary file.
    /// Return `true` or `false` here depending on whether the operation was successful.
    ///
    /// It's your responsibility to remove the temp file.
    fn open_temp_file(&self, _filename: &str) -> bool { false }

    /// Fired before attempting to open an untitled file. Return `true` here if you want
    /// `open_untitled_file` to be called by the system.
    fn should_open_untitled_file(&self) -> bool { false }

    /// Called when the application has asked you to open a new, untitled file.
    /// Returns a `bool` indicating whether the file was successfully opened or not.
    fn open_untitled_file(&self) -> bool { true }

    /// Sent when the user starts up the application on the command line with the -NSPrint option.
    /// The application terminates immediately after this method returns. For more information,
    /// cosnult the official Apple documentation.
    ///
    /// (You probably never need to implement this, but we support it anyway)
    fn print_file(&self, _filename: &str) -> bool { false }

    /// Called when the user has requested to print some files.
    ///
    /// Returns a `PrintResponse`, indicating status of the print job. You can return
    /// `PrintResponse::ReplyLater` if you need to do something like confirming via a popover. If
    /// you do this, though, you must call `App::reply_to_open_or_print()` when the operation has
    /// been completed.
    ///
    /// Note that macOS has a long-deprecated `printFiles:` method, which your searching may bring
    /// up. This method really maps to `application:printFiles:withSettings:showPrintPanels:`, so
    /// be sure to just... look there.
    fn print_files(&self, _filenames: Vec<String>, _settings: PrintSettings, _show_panels: bool) -> PrintResponse {
        PrintResponse::Failure
    }

    /// Fired when the occlusion state for the app has changed.
    ///
    /// From Apple's docs, as there's no other way to describe this better: _upon receiving this method, you can query the 
    /// application for its occlusion state. Note that this only notifies about changes in the state of the occlusion, not 
    /// when the occlusion region changes. You can use this method to increase responsiveness and save power by halting any 
    /// expensive calculations that the user can not see._
    fn occlusion_state_changed(&self) {}

    /// Fired when the system wants to know whether your application, via scripting, can handle the
    /// key specifying operations.
    fn delegate_handles_key(&self, _key: &str) -> bool { false }
}
