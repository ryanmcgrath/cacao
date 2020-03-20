use std::rc::Rc;
use std::cell::RefCell;

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, NO};
use crate::constants::VIEW_DELEGATE_PTR;
use crate::layout::{Layout};
use crate::view::traits::ViewDelegate;

mod class;
use class::register_view_controller_class;

//#[derive(Debug)]
pub struct ViewController {
    pub objc: ShareId<Object>,
    pub view: Box<dyn ViewDelegate>
}

impl ViewController {
    pub fn new<T: ViewDelegate + Layout + 'static>(view: T) -> Self {
        let view = Box::new(view);

        let objc = unsafe {
            let vc: id = msg_send![register_view_controller_class::<T>(), new];
            let _: () = msg_send![vc, setView:&*view.get_backing_node()];
            let ptr: *const T = &*view;
            (&mut *vc).set_ivar(VIEW_DELEGATE_PTR, ptr as usize);
            ShareId::from_ptr(vc)
        };

        ViewController {
            objc: objc,
            view: view
        }
    }
}
