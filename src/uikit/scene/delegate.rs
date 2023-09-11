use objc::runtime::{Class, Object, Protocol, Sel};
use objc::{class, msg_send, sel};

use crate::foundation::{id, load_or_register_class_with_optional_generated_suffix};
use crate::uikit::app::SCENE_DELEGATE_VENDOR;
use crate::uikit::scene::{Scene, SceneConnectionOptions, SceneSession, WindowSceneDelegate};
use crate::utils::load;

pub(crate) static WINDOW_SCENE_PTR: &str = "rstWindowSceneDelegatePtr";

///
extern "C" fn init<T: WindowSceneDelegate, F: Fn() -> Box<T>>(mut this: &mut Object, _: Sel) -> id {
    let x = unsafe {
        this = msg_send![super(this, class!(UIResponder)), init];

        let scene_delegate_vendor = SCENE_DELEGATE_VENDOR as *const F;
        let factory: &F = &*scene_delegate_vendor;
        let scene_delegate = factory();
        let scene_delegate_ptr = Box::into_raw(scene_delegate);
        //println!("scene ptr: {:p}", scene_delegate_ptr);
        this.set_ivar(WINDOW_SCENE_PTR, scene_delegate_ptr as usize);

        this
    };

    x
}

extern "C" fn scene_will_connect_to_session_with_options<T: WindowSceneDelegate>(
    this: &Object,
    _: Sel,
    scene: id,
    session: id,
    options: id
) {
    let delegate = load::<T>(this, WINDOW_SCENE_PTR);

    delegate.will_connect(
        Scene::with(scene),
        SceneSession::with(session),
        SceneConnectionOptions::with(options)
    );
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
pub(crate) fn register_window_scene_delegate_class<T: WindowSceneDelegate, F: Fn() -> Box<T>>() -> &'static Class {
    let should_generate_suffix = false;

    load_or_register_class_with_optional_generated_suffix("UIResponder", "RSTWindowSceneDelegate", false, |decl| unsafe {
        let p = Protocol::get("UIWindowSceneDelegate").unwrap();

        // A spot to hold a pointer to
        decl.add_ivar::<usize>(WINDOW_SCENE_PTR);
        decl.add_protocol(p);

        // Override the `init` call to handle creating and attaching a WindowSceneDelegate.
        decl.add_method(sel!(init), init::<T, F> as extern "C" fn(_, _) -> _);

        // UIWindowSceneDelegate API
        decl.add_method(
            sel!(scene:willConnectToSession:options:),
            scene_will_connect_to_session_with_options::<T> as extern "C" fn(_, _, _, _, _)
        );
    })
}
