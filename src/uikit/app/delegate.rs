//! This module implements forwarding methods for standard `UIApplicationDelegate` calls. It also
//! creates a custom `UIApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

use objc::runtime::{Class, Object, Sel};
use objc::sel;

//use crate::error::Error;
use crate::foundation::{id, load_or_register_class_with_optional_generated_suffix, BOOL, YES};
use crate::uikit::app::{AppDelegate, APP_DELEGATE};
use crate::uikit::scene::{SceneConnectionOptions, SceneSession};

//use std::unreachable;

//use block::Block;

//use crate::user_activity::UserActivity;

/// A handy method for grabbing our `AppDelegate` from the pointer. This is different from our
/// standard `utils` version as this doesn't require `RefCell` backing.
fn app<T>(this: &Object) -> &T {
    unsafe {
        //let app_ptr: usize = *this.get_ivar(APP_DELEGATE);
        let app = APP_DELEGATE as *const T;
        &*app
    }
}

/// Fires when the Application Delegate receives a `applicationDidFinishLaunching` notification.
extern "C" fn did_finish_launching<T: AppDelegate>(this: &Object, _: Sel, _: id, _: id) -> BOOL {
    app::<T>(this).did_finish_launching();
    YES
}

extern "C" fn configuration_for_scene_session<T: AppDelegate>(this: &Object, _: Sel, _: id, session: id, opts: id) -> id {
    app::<T>(this)
        .config_for_scene_session(SceneSession::with(session), SceneConnectionOptions::with(opts))
        .into_inner()
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
pub(crate) fn register_app_delegate_class<T: AppDelegate>() -> &'static Class {
    let should_generate_suffix = false;

    load_or_register_class_with_optional_generated_suffix("NSObject", "RSTAppDelegate", should_generate_suffix, |decl| unsafe {
        // Launching Applications
        decl.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            did_finish_launching::<T> as extern "C" fn(_, _, _, _) -> _
        );

        // Scenes
        decl.add_method(
            sel!(application:configurationForConnectingSceneSession:options:),
            configuration_for_scene_session::<T> as extern "C" fn(_, _, _, _, _) -> _
        );
        /*decl.add_method(
            sel!(application:didDiscardSceneSessions:),
            did_discard_scene_sessions::<T> as extern "C" fn(_, _, _, _)
        );*/
    })
}
