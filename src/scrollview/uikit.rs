//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use std::sync::Once;

use objc::declare::ClassBuilder;
use objc::runtime::{Bool, Class, Object, Sel};
use objc::{class, sel};

use crate::foundation::{id, NSUInteger};
use crate::scrollview::{ScrollViewDelegate, SCROLLVIEW_DELEGATE_PTR};
use crate::utils::load;

/// Enforces normalcy, or: a needlessly cruel method in terms of the name. You get the idea though.
extern "C" fn enforce_normalcy(_: &Object, _: Sel) -> Bool {
    return Bool::YES;
}

/*
use crate::dragdrop::DragInfo;
/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_entered<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);
    view.dragging_entered(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    })
    .into()
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn prepare_for_drag_operation<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) -> Bool {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);

    Bool::new(view.prepare_for_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }))
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn perform_drag_operation<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) -> Bool {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);

    Bool::new(view.perform_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }))
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn conclude_drag_operation<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);

    view.conclude_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    });
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_exited<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);

    view.dragging_exited(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    });
}
*/

/// Injects an `UIScrollView` subclass.
pub(crate) fn register_scrollview_class() -> &'static Class {
    static mut VIEW_CLASS: Option<&'static Class> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UIScrollView);
        let decl = ClassBuilder::new("RSTScrollView", superclass).unwrap();
        VIEW_CLASS = Some(decl.register());
    });

    unsafe { VIEW_CLASS.unwrap() }
}

/// Injects an `NSView` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_scrollview_class_with_delegate<T: ScrollViewDelegate>() -> &'static Class {
    static mut VIEW_CLASS: Option<&'static Class> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(UIScrollView);
        let mut decl = ClassBuilder::new("RSTScrollViewWithDelegate", superclass).unwrap();

        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(SCROLLVIEW_DELEGATE_PTR);

        decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(_, _) -> _);

        /*
        // Drag and drop operations (e.g, accepting files)
        decl.add_method(
            sel!(draggingEntered:),
            dragging_entered::<T> as extern "C" fn(_, _, _) -> _
        );
        decl.add_method(
            sel!(prepareForDragOperation:),
            prepare_for_drag_operation::<T> as extern "C" fn(_, _, _) -> _
        );
        decl.add_method(
            sel!(performDragOperation:),
            perform_drag_operation::<T> as extern "C" fn(_, _, _) -> _
        );
        decl.add_method(
            sel!(concludeDragOperation:),
            conclude_drag_operation::<T> as extern "C" fn(_, _, _)
        );
        decl.add_method(
            sel!(draggingExited:),
            dragging_exited::<T> as extern "C" fn(_, _, _)
        );
        */

        VIEW_CLASS = Some(decl.register());
    });

    unsafe { VIEW_CLASS.unwrap() }
}
