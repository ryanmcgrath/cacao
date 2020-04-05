
use core_graphics::geometry::CGRect;

use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

use crate::foundation::id;
use crate::geometry::Rect;
use crate::ios::Scene;
use crate::utils::Controller;

#[derive(Debug)]
pub struct Window(pub Id<Object>);

impl Window {
    pub fn new(frame: Rect) -> Self {
        Window(unsafe {
            let rect: CGRect = frame.into();
            let alloc: id = msg_send![class!(UIWindow), alloc];
            Id::from_ptr(msg_send![alloc, initWithFrame:rect])
        })
    }

    pub fn set_window_scene(&mut self, scene: Scene) {
        unsafe {
            let _: () = msg_send![&*self.0, setWindowScene:scene.into_inner()];
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
