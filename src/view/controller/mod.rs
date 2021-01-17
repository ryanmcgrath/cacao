use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::foundation::id;
use crate::layout::{Layout};
use crate::view::{VIEW_DELEGATE_PTR, View, ViewDelegate};
use crate::utils::Controller;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::register_view_controller_class;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::register_view_controller_class;

#[derive(Debug)]
pub struct ViewController<T> {
    pub objc: ShareId<Object>,
    pub view: View<T>
}

impl<T> ViewController<T> where T: ViewDelegate + 'static {
    pub fn new(delegate: T) -> Self {
        let mut view = View::with(delegate);

        let objc = unsafe {
            let vc: id = msg_send![register_view_controller_class::<T>(), new];
            
            if let Some(delegate)= &view.delegate {
                let ptr: *const T = &**delegate;
                (&mut *vc).set_ivar(VIEW_DELEGATE_PTR, ptr as usize);
            }

            let _: () = msg_send![vc, setView:&*view.get_backing_node()];

            ShareId::from_ptr(vc)
        };

        //let handle = view.clone_as_handle();
        //if let Some(view_delegate) = &mut view.delegate {
        //    view_delegate.did_load(handle);
        //}

        ViewController {
            objc: objc,
            view: view
        }
    }
}

impl<T> Controller for ViewController<T> {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }
}
