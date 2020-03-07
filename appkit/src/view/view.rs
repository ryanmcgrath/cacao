//! A wrapper for `NSViewController`. Uses interior mutability to 

use std::rc::Rc;
use std::cell::RefCell;

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::NSArray;

use objc_id::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

use crate::color::Color;
use crate::constants::{BACKGROUND_COLOR, VIEW_CONTROLLER_PTR};
use crate::pasteboard::PasteboardType;
use crate::view::traits::ViewController;
use crate::view::controller::register_controller_class;

#[derive(Default)]
pub struct ViewInner {
    pub controller: Option<ShareId<Object>>
}

impl ViewInner {
    pub fn configure<T: ViewController + 'static>(&mut self, controller: &T) {
        self.controller = Some(unsafe {
            let view_controller: id = msg_send![register_controller_class::<T>(), new];
            (&mut *view_controller).set_ivar(VIEW_CONTROLLER_PTR, controller as *const T as usize);
            
            let view: id = msg_send![view_controller, view];
            (&mut *view).set_ivar(VIEW_CONTROLLER_PTR, controller as *const T as usize);
            
            ShareId::from_ptr(view_controller)
        });
    }

    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        if let Some(controller) = &self.controller {
            unsafe {
                let types = NSArray::arrayWithObjects(nil, &types.iter().map(|t| {
                    t.to_nsstring()
                }).collect::<Vec<id>>());

                let view: id = msg_send![*controller, view];
                let _: () = msg_send![view, registerForDraggedTypes:types];
            }
        }
    }

    pub fn set_background_color(&self, color: Color) {
        if let Some(controller) = &self.controller {
            unsafe {
                let view: id = msg_send![*controller, view];
                (*view).set_ivar(BACKGROUND_COLOR, color.into_platform_specific_color());
                let _: () = msg_send![view, setNeedsDisplay:YES];
            }
        }
    }
}

#[derive(Default)]
pub struct View(Rc<RefCell<ViewInner>>);

impl View {
    pub fn configure<T: ViewController + 'static>(&self, controller: &T) {
        {
            let mut view = self.0.borrow_mut();
            view.configure(controller);
        }

        controller.did_load();
    }

    pub fn get_handle(&self) -> Option<ShareId<Object>> {
        let view = self.0.borrow();
        view.controller.clone()
    }

    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        let view = self.0.borrow();
        view.register_for_dragged_types(types);
    }

    pub fn set_background_color(&self, color: Color) {
        let view = self.0.borrow();
        view.set_background_color(color);
    }
}
