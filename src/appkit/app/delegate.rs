//! This module implements forwarding methods for standard `NSApplicationDelegate` calls. It also
//! creates a custom `NSApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

use std::ffi::c_void;
use std::sync::Once;

use block::Block;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use url::Url;

use crate::appkit::app::{AppDelegate, APP_PTR};
use crate::appkit::printing::PrintSettings;
use crate::error::Error;
use crate::foundation::{id, nil, to_bool, NSArray, NSString, NSUInteger, BOOL, NO, YES};
use crate::user_activity::UserActivity;

#[cfg(feature = "cloudkit")]
use crate::cloudkit::share::CKShareMetaData;

/// A handy method for grabbing our `AppDelegate` from the pointer. This is different from our
/// standard `utils` version as this doesn't require `RefCell` backing.
fn app<T>(this: &Object) -> &T {
    unsafe {
        let app_ptr: usize = *this.get_ivar(APP_PTR);
        let app = app_ptr as *const T;
        &*app
    }
}

/// Fires when the Application Delegate receives a `applicationWillFinishLaunching` notification.
extern "C" fn will_finish_launching<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_finish_launching();
}

/// Fires when the Application Delegate receives a `applicationDidFinishLaunching` notification.
extern "C" fn did_finish_launching<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_finish_launching();
}

/// Fires when the Application Delegate receives a `applicationWillBecomeActive` notification.
extern "C" fn will_become_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_become_active();
}

/// Fires when the Application Delegate receives a `applicationDidBecomeActive` notification.
extern "C" fn did_become_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_become_active();
}

/// Fires when the Application Delegate receives a `applicationWillResignActive` notification.
extern "C" fn will_resign_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_resign_active();
}

/// Fires when the Application Delegate receives a `applicationDidResignActive` notification.
extern "C" fn did_resign_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_resign_active();
}

/// Fires when the Application Delegate receives a 'applicationShouldTerminate:` notification.
extern "C" fn should_terminate<T: AppDelegate>(this: &Object, _: Sel, _: id) -> NSUInteger {
    app::<T>(this).should_terminate().into()
}

/// Fires when the Application Delegate receives a `applicationWillTerminate:` notification.
extern "C" fn will_terminate<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_terminate();
}

/// Fires when the Application Delegate receives a `applicationWillHide:` notification.
extern "C" fn will_hide<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_hide();
}

/// Fires when the Application Delegate receives a `applicationDidHide:` notification.
extern "C" fn did_hide<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_hide();
}

/// Fires when the Application Delegate receives a `applicationWillUnhide:` notification.
extern "C" fn will_unhide<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_unhide();
}

/// Fires when the Application Delegate receives a `applicationDidUnhide:` notification.
extern "C" fn did_unhide<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_unhide();
}

/// Fires when the Application Delegate receives a `applicationWillUpdate:` notification.
extern "C" fn will_update<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).will_update();
}

/// Fires when the Application Delegate receives a `applicationDidUpdate:` notification.
extern "C" fn did_update<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_update();
}

/// Fires when the Application Delegate receives a
/// `applicationShouldHandleReopen:hasVisibleWindows:` notification.
extern "C" fn should_handle_reopen<T: AppDelegate>(this: &Object, _: Sel, _: id, has_visible_windows: BOOL) -> BOOL {
    match app::<T>(this).should_handle_reopen(to_bool(has_visible_windows)) {
        true => YES,
        false => NO,
    }
}

/// Fires when the application delegate receives a `applicationDockMenu:` request.
// @TODO: Make this return Vec<MenuItem>.
extern "C" fn dock_menu<T: AppDelegate>(this: &Object, _: Sel, _: id) -> id {
    match app::<T>(this).dock_menu() {
        Some(mut menu) => &mut *menu.0,
        None => nil,
    }
}

/// Fires when the application delegate receives a `application:willPresentError:` notification.
extern "C" fn will_present_error<T: AppDelegate>(this: &Object, _: Sel, _: id, error: id) -> id {
    let error = Error::new(error);
    app::<T>(this).will_present_error(error).into_nserror()
}

/// Fires when the application receives a `applicationDidChangeScreenParameters:` notification.
extern "C" fn did_change_screen_parameters<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).did_change_screen_parameters();
}

/// Fires when the application receives a `application:willContinueUserActivityWithType:`
/// notification.
extern "C" fn will_continue_user_activity_with_type<T: AppDelegate>(this: &Object, _: Sel, _: id, activity_type: id) -> BOOL {
    let activity = NSString::retain(activity_type);

    match app::<T>(this).will_continue_user_activity(activity.to_str()) {
        true => YES,
        false => NO,
    }
}

/// Fires when the application receives a `application:continueUserActivity:restorationHandler:` notification.
extern "C" fn continue_user_activity<T: AppDelegate>(this: &Object, _: Sel, _: id, activity: id, handler: id) -> BOOL {
    // @TODO: This needs to support restorable objects, but it involves a larger question about how
    // much `NSObject` retainping we want to do here. For now, pass the handler for whenever it's
    // useful.
    let activity = UserActivity::with_inner(activity);

    match app::<T>(this).continue_user_activity(activity, || unsafe {
        let handler = handler as *const Block<(id,), c_void>;
        (*handler).call((nil,));
    }) {
        true => YES,
        false => NO,
    }
}

/// Fires when the application receives a
/// `application:didFailToContinueUserActivityWithType:error:` message.
extern "C" fn failed_to_continue_user_activity<T: AppDelegate>(this: &Object, _: Sel, _: id, activity_type: id, error: id) {
    app::<T>(this).failed_to_continue_user_activity(NSString::retain(activity_type).to_str(), Error::new(error));
}

/// Fires when the application receives a `application:didUpdateUserActivity:` message.
extern "C" fn did_update_user_activity<T: AppDelegate>(this: &Object, _: Sel, _: id, activity: id) {
    let activity = UserActivity::with_inner(activity);
    app::<T>(this).updated_user_activity(activity);
}

/// Fires when the application receives a `application:didRegisterForRemoteNotificationsWithDeviceToken:` message.
extern "C" fn registered_for_remote_notifications<T: AppDelegate>(_this: &Object, _: Sel, _: id, _: id) {}

/// Fires when the application receives a `application:didFailToRegisterForRemoteNotificationsWithError:` message.
extern "C" fn failed_to_register_for_remote_notifications<T: AppDelegate>(this: &Object, _: Sel, _: id, error: id) {
    app::<T>(this).failed_to_register_for_remote_notifications(Error::new(error));
}

/// Fires when the application receives a `application:didReceiveRemoteNotification:` message.
extern "C" fn did_receive_remote_notification<T: AppDelegate>(_this: &Object, _: Sel, _: id, _: id) {}

/// Fires when the application receives a `application:userDidAcceptCloudKitShareWithMetadata:`
/// message.
#[cfg(feature = "cloudkit")]
extern "C" fn accepted_cloudkit_share<T: AppDelegate>(this: &Object, _: Sel, _: id, metadata: id) {
    let share = CKShareMetaData::with_inner(metadata);
    app::<T>(this).user_accepted_cloudkit_share(share);
}

/// Fires when the application receives an `application:openURLs` message.
extern "C" fn open_urls<T: AppDelegate>(this: &Object, _: Sel, _: id, file_urls: id) {
    let urls = NSArray::retain(file_urls)
        .map(|url| {
            let uri = NSString::retain(unsafe { msg_send![url, absoluteString] });

            Url::parse(uri.to_str())
        })
        .into_iter()
        .filter_map(|url| url.ok())
        .collect();

    app::<T>(this).open_urls(urls);
}

/// Fires when the application receives an `application:openFileWithoutUI:` message.
extern "C" fn open_file_without_ui<T: AppDelegate>(this: &Object, _: Sel, _: id, file: id) -> BOOL {
    let filename = NSString::retain(file);

    match app::<T>(this).open_file_without_ui(filename.to_str()) {
        true => YES,
        false => NO,
    }
}

/// Fired when the application receives an `applicationShouldOpenUntitledFile:` message.
extern "C" fn should_open_untitled_file<T: AppDelegate>(this: &Object, _: Sel, _: id) -> BOOL {
    match app::<T>(this).should_open_untitled_file() {
        true => YES,
        false => NO,
    }
}

/// Fired when the application receives an `applicationShouldTerminateAfterLastWindowClosed:` message.
extern "C" fn should_terminate_after_last_window_closed<T: AppDelegate>(this: &Object, _: Sel, _: id) -> BOOL {
    match app::<T>(this).should_terminate_after_last_window_closed() {
        true => YES,
        false => NO,
    }
}

/// Fired when the application receives an `applicationOpenUntitledFile:` message.
extern "C" fn open_untitled_file<T: AppDelegate>(this: &Object, _: Sel, _: id) -> BOOL {
    match app::<T>(this).open_untitled_file() {
        true => YES,
        false => NO,
    }
}

/// Fired when the application receives an `application:openTempFile:` message.
extern "C" fn open_temp_file<T: AppDelegate>(this: &Object, _: Sel, _: id, filename: id) -> BOOL {
    let filename = NSString::retain(filename);

    match app::<T>(this).open_temp_file(filename.to_str()) {
        true => YES,
        false => NO,
    }
}

/// Fired when the application receives an `application:printFile:` message.
extern "C" fn print_file<T: AppDelegate>(this: &Object, _: Sel, _: id, file: id) -> BOOL {
    let filename = NSString::retain(file);

    match app::<T>(this).print_file(filename.to_str()) {
        true => YES,
        false => NO,
    }
}

/// Fired when the application receives an `application:printFiles:withSettings:showPrintPanels:`
/// message.
extern "C" fn print_files<T: AppDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    files: id,
    settings: id,
    show_print_panels: BOOL,
) -> NSUInteger {
    let files = NSArray::retain(files).map(|file| NSString::retain(file).to_str().to_string());

    let settings = PrintSettings::with_inner(settings);

    app::<T>(this).print_files(files, settings, to_bool(show_print_panels)).into()
}

/// Called when the application's occlusion state has changed.
extern "C" fn did_change_occlusion_state<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    app::<T>(this).occlusion_state_changed();
}

/// Called when the application receives an `application:delegateHandlesKey:` message.
/// Note: this may not fire in sandboxed applications. Apple's documentation is unclear on the
/// matter.
extern "C" fn delegate_handles_key<T: AppDelegate>(this: &Object, _: Sel, _: id, key: id) -> BOOL {
    let key = NSString::retain(key);

    match app::<T>(this).delegate_handles_key(key.to_str()) {
        true => YES,
        false => NO,
    }
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
pub(crate) fn register_app_delegate_class<T: AppDelegate + AppDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTAppDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(APP_PTR);

        // Launching Applications
        decl.add_method(
            sel!(applicationWillFinishLaunching:),
            will_finish_launching::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching::<T> as extern "C" fn(&Object, _, _),
        );

        // Managing Active Status
        decl.add_method(
            sel!(applicationWillBecomeActive:),
            will_become_active::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidBecomeActive:),
            did_become_active::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(applicationWillResignActive:),
            will_resign_active::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidResignActive:),
            did_resign_active::<T> as extern "C" fn(&Object, _, _),
        );

        // Terminating Applications
        decl.add_method(
            sel!(applicationShouldTerminate:),
            should_terminate::<T> as extern "C" fn(&Object, _, _) -> NSUInteger,
        );
        decl.add_method(
            sel!(applicationWillTerminate:),
            will_terminate::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            should_terminate_after_last_window_closed::<T> as extern "C" fn(&Object, _, _) -> BOOL,
        );

        // Hiding Applications
        decl.add_method(sel!(applicationWillHide:), will_hide::<T> as extern "C" fn(&Object, _, _));
        decl.add_method(sel!(applicationDidHide:), did_hide::<T> as extern "C" fn(&Object, _, _));
        decl.add_method(sel!(applicationWillUnhide:), will_unhide::<T> as extern "C" fn(&Object, _, _));
        decl.add_method(sel!(applicationDidUnhide:), did_unhide::<T> as extern "C" fn(&Object, _, _));

        // Managing Windows
        decl.add_method(sel!(applicationWillUpdate:), will_update::<T> as extern "C" fn(&Object, _, _));
        decl.add_method(sel!(applicationDidUpdate:), did_update::<T> as extern "C" fn(&Object, _, _));
        decl.add_method(
            sel!(applicationShouldHandleReopen:hasVisibleWindows:),
            should_handle_reopen::<T> as extern "C" fn(&Object, _, _, BOOL) -> BOOL,
        );

        // Dock Menu
        decl.add_method(
            sel!(applicationDockMenu:),
            dock_menu::<T> as extern "C" fn(&Object, _, _) -> id,
        );

        // Displaying Errors
        decl.add_method(
            sel!(application:willPresentError:),
            will_present_error::<T> as extern "C" fn(&Object, _, _, id) -> id,
        );

        // Managing the Screen
        decl.add_method(
            sel!(applicationDidChangeScreenParameters:),
            did_change_screen_parameters::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidChangeOcclusionState:),
            did_change_occlusion_state::<T> as extern "C" fn(&Object, _, _),
        );

        // User Activities
        decl.add_method(
            sel!(application:willContinueUserActivityWithType:),
            will_continue_user_activity_with_type::<T> as extern "C" fn(&Object, _, _, id) -> BOOL,
        );
        decl.add_method(
            sel!(application:continueUserActivity:restorationHandler:),
            continue_user_activity::<T> as extern "C" fn(&Object, _, _, id, id) -> BOOL,
        );
        decl.add_method(
            sel!(application:didFailToContinueUserActivityWithType:error:),
            failed_to_continue_user_activity::<T> as extern "C" fn(&Object, _, _, id, id),
        );
        decl.add_method(
            sel!(application:didUpdateUserActivity:),
            did_update_user_activity::<T> as extern "C" fn(&Object, _, _, id),
        );

        // Handling push notifications
        decl.add_method(
            sel!(application:didRegisterForRemoteNotificationsWithDeviceToken:),
            registered_for_remote_notifications::<T> as extern "C" fn(&Object, _, _, id),
        );
        decl.add_method(
            sel!(application:didFailToRegisterForRemoteNotificationsWithError:),
            failed_to_register_for_remote_notifications::<T> as extern "C" fn(&Object, _, _, id),
        );
        decl.add_method(
            sel!(application:didReceiveRemoteNotification:),
            did_receive_remote_notification::<T> as extern "C" fn(&Object, _, _, id),
        );

        // CloudKit
        #[cfg(feature = "cloudkit")]
        decl.add_method(
            sel!(application:userDidAcceptCloudKitShareWithMetadata:),
            accepted_cloudkit_share::<T> as extern "C" fn(&Object, _, _, id),
        );

        // Opening Files
        decl.add_method(
            sel!(application:openURLs:),
            open_urls::<T> as extern "C" fn(&Object, _, _, id),
        );
        decl.add_method(
            sel!(application:openFileWithoutUI:),
            open_file_without_ui::<T> as extern "C" fn(&Object, _, _, id) -> BOOL,
        );
        decl.add_method(
            sel!(applicationShouldOpenUntitledFile:),
            should_open_untitled_file::<T> as extern "C" fn(&Object, _, _) -> BOOL,
        );
        decl.add_method(
            sel!(applicationOpenUntitledFile:),
            open_untitled_file::<T> as extern "C" fn(&Object, _, _) -> BOOL,
        );
        decl.add_method(
            sel!(application:openTempFile:),
            open_temp_file::<T> as extern "C" fn(&Object, _, _, id) -> BOOL,
        );

        // Printing
        decl.add_method(
            sel!(application:printFile:),
            print_file::<T> as extern "C" fn(&Object, _, _, id) -> BOOL,
        );
        decl.add_method(
            sel!(application:printFiles:withSettings:showPrintPanels:),
            print_files::<T> as extern "C" fn(&Object, _, id, id, id, BOOL) -> NSUInteger,
        );

        // @TODO: Restoring Application State
        // Depends on NSCoder support, which is... welp.

        // Scripting
        decl.add_method(
            sel!(application:delegateHandlesKey:),
            delegate_handles_key::<T> as extern "C" fn(&Object, _, _, id) -> BOOL,
        );

        DELEGATE_CLASS = decl.register();
    });

    unsafe { DELEGATE_CLASS }
}
