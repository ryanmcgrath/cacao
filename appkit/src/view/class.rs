//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use std::rc::Rc;
use std::sync::Once;

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSUInteger};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{msg_send, sel, sel_impl};
use objc_id::Id;

use crate::constants::{BACKGROUND_COLOR, VIEW_CONTROLLER_PTR};
use crate::dragdrop::DragInfo;
use crate::view::traits::ViewController;
use crate::utils::load;


/// Enforces normalcy, or: a needlessly cruel method in terms of the name. You get the idea though.
extern fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// Used for handling background colors in layer backed views (which is the default here).
extern fn update_layer(this: &Object, _: Sel) {
    unsafe {
        let background_color: id = *this.get_ivar(BACKGROUND_COLOR);
        if background_color != nil {
            let layer: id = msg_send![this, layer];
            let cg: id = msg_send![background_color, CGColor];
            let _: () = msg_send![layer, setBackgroundColor:cg];
        }
    }
}

/// Called when a drag/drop operation has entered this view.
extern fn dragging_entered<T: ViewController>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = load::<T>(this, VIEW_CONTROLLER_PTR);

    let response = {
        let v = view.borrow();

        (*v).dragging_entered(DragInfo {
            info: unsafe { Id::from_ptr(info) }
        }).into()
    };

    Rc::into_raw(view);
    response
}

/// Called when a drag/drop operation has entered this view.
extern fn prepare_for_drag_operation<T: ViewController>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, VIEW_CONTROLLER_PTR);
    
    let response = {
        let v = view.borrow();
        
        match (*v).prepare_for_drag_operation(DragInfo {
            info: unsafe { Id::from_ptr(info) }
        }) {
            true => YES,
            false => NO
        }
    };

    Rc::into_raw(view); 
    response
}

/// Called when a drag/drop operation has entered this view.
extern fn perform_drag_operation<T: ViewController>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, VIEW_CONTROLLER_PTR);
        
    let response = {
        let v = view.borrow();
        
        match (*v).perform_drag_operation(DragInfo {
            info: unsafe { Id::from_ptr(info) }
        }) {
            true => YES,
            false => NO
        }
    };

    Rc::into_raw(view); 
    response
}

/// Called when a drag/drop operation has entered this view.
extern fn conclude_drag_operation<T: ViewController>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, VIEW_CONTROLLER_PTR);

    {
        let v = view.borrow();
        (*v).conclude_drag_operation(DragInfo {
            info: unsafe { Id::from_ptr(info) }
        });           
    }

    Rc::into_raw(view); 
}

/// Called when a drag/drop operation has entered this view.
extern fn dragging_exited<T: ViewController>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, VIEW_CONTROLLER_PTR);
        
    {
        let v = view.borrow();
        (*v).dragging_exited(DragInfo {
            info: unsafe { Id::from_ptr(info) }
        });
    }

    Rc::into_raw(view); 
}

/// Injects an `NSView` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class<T: ViewController>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSView").unwrap();
        let mut decl = ClassDecl::new("RSTView", superclass).unwrap();

        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(VIEW_CONTROLLER_PTR);
        decl.add_ivar::<id>(BACKGROUND_COLOR);
        
        decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);
        decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern fn(&Object, _) -> BOOL);
        decl.add_method(sel!(updateLayer), update_layer as extern fn(&Object, _));

        // Drag and drop operations (e.g, accepting files)
        decl.add_method(sel!(draggingEntered:), dragging_entered::<T> as extern fn (&mut Object, _, _) -> NSUInteger);
        decl.add_method(sel!(prepareForDragOperation:), prepare_for_drag_operation::<T> as extern fn (&mut Object, _, _) -> BOOL);
        decl.add_method(sel!(performDragOperation:), perform_drag_operation::<T> as extern fn (&mut Object, _, _) -> BOOL);
        decl.add_method(sel!(concludeDragOperation:), conclude_drag_operation::<T> as extern fn (&mut Object, _, _));
        decl.add_method(sel!(draggingExited:), dragging_exited::<T> as extern fn (&mut Object, _, _));
        
        VIEW_CLASS = decl.register();
    });

    unsafe {
        VIEW_CLASS
    }
}
