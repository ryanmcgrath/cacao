//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use crate::id_shim::Id;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{msg_send, sel, sel_impl};

use crate::appkit::menu::Menu;
use crate::dragdrop::DragInfo;
use crate::foundation::{id, load_or_register_class, NSArray, NSInteger, NSUInteger, NO, YES};
use crate::listview::{ListViewDelegate, RowEdge, LISTVIEW_DELEGATE_PTR};
use crate::utils::load;

/// Determines the number of items by way of the backing data source (the Rust struct).
extern "C" fn number_of_items<T: ListViewDelegate>(this: &Object, _: Sel, _: id) -> NSInteger {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    view.number_of_items() as NSInteger
}

extern "C" fn view_for_column<T: ListViewDelegate>(
    this: &Object,
    _: Sel,
    _table_view: id,
    _table_column: id,
    item: NSInteger
) -> id {
    /*use core_graphics::geometry::CGRect;
    unsafe {
        //let superview: id = msg_send![table_view, superview];
        let frame: CGRect = msg_send![table_view, frame];
        let _: () = msg_send![table_column, setWidth:frame.size.width];
    }*/

    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    let item = view.item_for(item as usize);

    // A hacky method of returning the underlying pointer
    // without Rust annoying us.
    //
    // @TODO: probably find a better way to do this. It's theoretically fine,
    // as we *know* the underlying view will be retained by the NSTableView, so
    // passing over one more won't really screw up retain counts.
    //
    // @TODO: Finish investing the `Rc` approach, might be able to just take
    // ownership and rely on Rust being correct.
    item.objc.get(|obj| unsafe { msg_send![obj, self] })
}

extern "C" fn will_display_cell<T: ListViewDelegate>(
    this: &Object,
    _: Sel,
    _table_view: id,
    _cell: id,
    _column: id,
    item: NSInteger
) {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    view.will_display_item(item as usize);
}

extern "C" fn menu_needs_update<T: ListViewDelegate>(this: &Object, _: Sel, menu: id) {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    let items = view.context_menu();
    let _ = Menu::append(menu, items);
}

/*/// NSTableView requires listening to an observer to detect row selection changes, but that is...
/// even clunkier than what we do in this framework.
///
/// The other less obvious way is to subclass and override the `shouldSelectRow:` method; here, we
/// simply assume things are selectable and call our delegate as if things were selected. This may
/// need to change in the future, but it works well enough for now.
extern "C" fn select_row<T: ListViewDelegate>(
    this: &Object,
    _: Sel,
    _table_view: id,
    item: NSInteger
) -> BOOL {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    view.item_selected(item as usize);
    YES
}*/

extern "C" fn selection_did_change<T: ListViewDelegate>(this: &Object, _: Sel, notification: id) {
    let selected_row: NSInteger = unsafe {
        let tableview: id = msg_send![notification, object];
        msg_send![tableview, selectedRow]
    };

    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    if selected_row == -1 {
        view.item_selected(None);
    } else {
        view.item_selected(Some(selected_row as usize));
    }
}

extern "C" fn row_actions_for_row<T: ListViewDelegate>(
    this: &Object,
    _: Sel,
    _table_view: id,
    row: NSInteger,
    edge: NSInteger
) -> id {
    let edge: RowEdge = edge.into();
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);

    let mut ids: NSArray = view
        .actions_for(row as usize, edge)
        .iter_mut()
        .map(|action| &*action.0)
        .collect::<Vec<&Object>>()
        .into();

    &mut *ids
}

/// Enforces normalcy, or: a needlessly cruel method in terms of the name. You get the idea though.
extern "C" fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_entered<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);
    view.dragging_entered(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    })
    .into()
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn prepare_for_drag_operation<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);

    match view.prepare_for_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }) {
        true => YES,
        false => NO
    }
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn perform_drag_operation<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) -> BOOL {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);

    match view.perform_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }) {
        true => YES,
        false => NO
    }
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn conclude_drag_operation<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, LISTVIEW_DELEGATE_PTR);

    view.conclude_drag_operation(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    });
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_exited<T: ListViewDelegate>(this: &mut Object, _: Sel, info: id) {
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
    load_or_register_class("NSTableView", "RSTListView", |decl| unsafe {})
}

/// Injects an `NSTableView` subclass, with some callback and pointer ivars for what we
/// need to do. Note that we treat and constrain this as a one-column "list" view to match
/// `UITableView` semantics; if `NSTableView`'s multi column behavior is needed, then it can
/// be added in.
pub(crate) fn register_listview_class_with_delegate<T: ListViewDelegate>(instance: &T) -> *const Class {
    load_or_register_class("NSTableView", instance.subclass_name(), |decl| unsafe {
        decl.add_ivar::<usize>(LISTVIEW_DELEGATE_PTR);

        decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(&Object, _) -> BOOL);

        // Tableview-specific
        decl.add_method(
            sel!(numberOfRowsInTableView:),
            number_of_items::<T> as extern "C" fn(&Object, _, id) -> NSInteger
        );
        decl.add_method(
            sel!(tableView:willDisplayCell:forTableColumn:row:),
            will_display_cell::<T> as extern "C" fn(&Object, _, id, id, id, NSInteger)
        );
        decl.add_method(
            sel!(tableView:viewForTableColumn:row:),
            view_for_column::<T> as extern "C" fn(&Object, _, id, id, NSInteger) -> id
        );
        decl.add_method(
            sel!(tableViewSelectionDidChange:),
            selection_did_change::<T> as extern "C" fn(&Object, _, id)
        );
        decl.add_method(
            sel!(tableView:rowActionsForRow:edge:),
            row_actions_for_row::<T> as extern "C" fn(&Object, _, id, NSInteger, NSInteger) -> id
        );

        // A slot for some menu handling; we just let it be done here for now rather than do the
        // whole delegate run, since things are fast enough nowadays to just replace the entire
        // menu.
        decl.add_method(
            sel!(menuNeedsUpdate:),
            menu_needs_update::<T> as extern "C" fn(&Object, _, id)
        );

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
