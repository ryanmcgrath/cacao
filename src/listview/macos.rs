//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{class, sel, sel_impl, msg_send};
use objc_id::Id;

use crate::foundation::{id, YES, NO, NSInteger, NSUInteger};
use crate::dragdrop::DragInfo;
use crate::listview::{LISTVIEW_DELEGATE_PTR, ListViewDelegate};
use crate::utils::load;

/// Determines the number of items by way of the backing data source (the Rust struct).
extern fn number_of_items<T: ListViewDelegate>(
    this: &Object,
    _: Sel,
    _: id
) -> NSInteger {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    view.number_of_items() as NSInteger
}

extern fn view_for_column<T: ListViewDelegate>(
    this: &Object,
    _: Sel,
    table_view: id,
    _: id,
    item: NSInteger
) -> id {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    let item = view.item(item as usize);

    // A hacky method of returning the underlying pointer
    // without Rust annoying us.
    //
    // @TODO: probably find a better way to do this. It's theoretically fine,
    // as we *know* the underlying view will be retained by the NSTableView, so
    // passing over one more won't really screw up retain counts.
    unsafe {
        msg_send![&*item, self]
    }
}

/// Enforces normalcy, or: a needlessly cruel method in terms of the name. You get the idea though.
extern fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// Called when a drag/drop operation has entered this view.
extern fn dragging_entered<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    view.dragging_entered(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }).into()
}

/// Called when a drag/drop operation has entered this view.
extern fn prepare_for_drag_operation<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    
    match view.prepare_for_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }) {
        true => YES,
        false => NO
    }
}

/// Called when a drag/drop operation has entered this view.
extern fn perform_drag_operation<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
        
    match view.perform_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }) {
        true => YES,
        false => NO
    }
}

/// Called when a drag/drop operation has entered this view.
extern fn conclude_drag_operation<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    
    view.conclude_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    });           
}

/// Called when a drag/drop operation has entered this view.
extern fn dragging_exited<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
        
    view.dragging_exited(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    });
}

/// Injects an `NSTableView` subclass, with some callback and pointer ivars for what we
/// need to do. Note that we treat and constrain this as a one-column "list" view to match
/// `UITableView` semantics; if `NSTableView`'s multi column behavior is needed, then it can
/// be added in.
pub(crate) fn register_listview_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSTableView);
        let mut decl = ClassDecl::new("RSTListView", superclass).unwrap();
       
        VIEW_CLASS = decl.register();
    });

    unsafe {
        VIEW_CLASS
    }
}

/// Injects an `NSTableView` subclass, with some callback and pointer ivars for what we
/// need to do. Note that we treat and constrain this as a one-column "list" view to match
/// `UITableView` semantics; if `NSTableView`'s multi column behavior is needed, then it can
/// be added in.
pub(crate) fn register_listview_class_with_delegate<T: ListViewDelegate>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSTableView);
        let mut decl = ClassDecl::new("RSTListViewWithDelegate", superclass).unwrap();

        // A pointer to the "view controller" on the Rust side. It's expected that this doesn't
        // move.
        decl.add_ivar::<usize>(LISTVIEW_DELEGATE_PTR);
        
        decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Tableview-specific
        decl.add_method(sel!(numberOfRowsInTableView:), number_of_items::<T> as extern fn(&Object, _, id) -> NSInteger);
        decl.add_method(sel!(tableView:viewForTableColumn:row:), view_for_column::<T> as extern fn(&Object, _, id, id, NSInteger) -> id);

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
