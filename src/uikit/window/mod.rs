use core_graphics::geometry::CGRect;

use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::id;
use crate::geometry::Rect;
use crate::uikit::Scene;
use crate::utils::Controller;

#[derive(Debug)]
pub struct Window(pub Id<Object, Owned>);

impl Window {
    pub fn new(frame: Rect) -> Self {
        Window(unsafe {
            let rect: CGRect = frame.into();
            let alloc = msg_send_id![class!(UIWindow), alloc];
            msg_send_id![alloc, initWithFrame: rect]
        })
    }

    pub fn set_window_scene(&mut self, scene: Scene) {
        unsafe {
            let _: () = msg_send![&*self.0, setWindowScene: &*scene.0];
        }
    }

    pub fn set_root_view_controller<VC: Controller + 'static>(&self, controller: &VC) {
        let backing_node = controller.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.0, setRootViewController:&*backing_node];
        }
    }

    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.0, makeKeyAndVisible];
        }
    }
}
