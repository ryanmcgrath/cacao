//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{sel, sel_impl};
use objc_id::Id;

use crate::dragdrop::DragInfo;
use crate::foundation::{id, load_or_register_class, NSUInteger, NO, YES};
use crate::scrollview::{ScrollViewDelegate, SCROLLVIEW_DELEGATE_PTR};
use crate::utils::load;

/// Enforces normalcy, or: a needlessly cruel method in terms of the name. You get the idea though.
extern "C" fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_entered<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);
    view.dragging_entered(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    })
    .into()
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn prepare_for_drag_operation<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);

    match view.prepare_for_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }) {
        true => YES,
        false => NO
    }
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn perform_drag_operation<T: ScrollViewDelegate>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, SCROLLVIEW_DELEGATE_PTR);

    match view.perform_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }) {
        true => YES,
        false => NO
    }
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

/// Injects an `NSScrollView` subclass.
pub(crate) fn register_scrollview_class() -> *const Class {
    load_or_register_class("NSScrollView", "RSTScrollView", |decl| unsafe {})
}

/// Injects an `NSView` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_scrollview_class_with_delegate<T: ScrollViewDelegate>() -> *const Class {
    load_or_register_class("NSScrollView", "RSTScrollViewWithDelegate", |decl| unsafe {
        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(SCROLLVIEW_DELEGATE_PTR);

        decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(&Object, _) -> BOOL);

        // Drag and drop operations (e.g, accepting files)
        decl.add_method(
            sel!(draggingEntered:),
            dragging_entered::<T> as extern "C" fn(&mut Object, _, _) -> NSUInteger
        );
        decl.add_method(
            sel!(prepareForDragOperation:),
            prepare_for_drag_operation::<T> as extern "C" fn(&mut Object, _, _) -> BOOL
        );
        decl.add_method(
            sel!(performDragOperation:),
            perform_drag_operation::<T> as extern "C" fn(&mut Object, _, _) -> BOOL
        );
        decl.add_method(
            sel!(concludeDragOperation:),
            conclude_drag_operation::<T> as extern "C" fn(&mut Object, _, _)
        );
        decl.add_method(
            sel!(draggingExited:),
            dragging_exited::<T> as extern "C" fn(&mut Object, _, _)
        );
    })
}
