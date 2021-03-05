//! Various traits used for Views.

use crate::macos::menu::MenuItem;
use crate::dragdrop::{DragInfo, DragOperation};
use crate::listview::{ListView, ListViewRow, RowAction, RowEdge};
use crate::layout::Layout;
use crate::view::View;

#[allow(unused_variables)]
pub trait ListViewDelegate {
    /// Used to cache subclass creations on the Objective-C side.
    /// You can just set this to be the name of your view type. This
    /// value *must* be unique per-type.
    const NAME: &'static str;

    /// You should rarely (read: probably never) need to implement this yourself.
    /// It simply acts as a getter for the associated `NAME` const on this trait.
    fn subclass_name(&self) -> &'static str {
        Self::NAME
    }

    /// Called when the View is ready to work with. You're passed a `View` - this is safe to
    /// store and use repeatedly, but it's not thread safe - any UI calls must be made from the
    /// main thread!
    fn did_load(&mut self, view: ListView);

    /// Returns the number of items in the list view.
    fn number_of_items(&self) -> usize;

    /// Called when an item will be displayed.
    fn will_display_item(&self, row: usize) {}

    /// This is temporary and you should not rely on this signature if you
    /// choose to try and work with this. NSTableView & such associated delegate patterns
    /// are tricky to support in Rust, and while I have a few ideas about them, I haven't
    /// had time to sit down and figure them out properly yet.
    fn item_for(&self, row: usize) -> ListViewRow;

    /// Called when an item has been selected (clicked/tapped on).
    fn item_selected(&self, row: usize) {}

    /// Called when the menu for the tableview is about to be shown. You can update the menu here
    /// depending on, say, what the user has context-clicked on. You should avoid any expensive
    /// work in here and return the menu as fast as possible.
    fn context_menu(&self) -> Vec<MenuItem> { vec![] }
    
    /// An optional delegate method; implement this if you'd like swipe-to-reveal to be
    /// supported for a given row by returning a vector of actions to show.
    fn actions_for(&self, row: usize, edge: RowEdge) -> Vec<RowAction> { Vec::new() }

    /// Called when this is about to be added to the view heirarchy.
    fn will_appear(&self, animated: bool) {}

    /// Called after this has been added to the view heirarchy.
    fn did_appear(&self, animated: bool) {}

    /// Called when this is about to be removed from the view heirarchy.
    fn will_disappear(&self, animated: bool) {}

    /// Called when this has been removed from the view heirarchy.
    fn did_disappear(&self, animated: bool) {}

    /// Invoked when the dragged image enters destination bounds or frame; returns dragging operation to perform.
    fn dragging_entered(&self, info: DragInfo) -> DragOperation { DragOperation::None }
    
    /// Invoked when the image is released, allowing the receiver to agree to or refuse drag operation.
    fn prepare_for_drag_operation(&self, info: DragInfo) -> bool { false }

    /// Invoked after the released image has been removed from the screen, signaling the receiver to import the pasteboard data.
    fn perform_drag_operation(&self, info: DragInfo) -> bool { false }

    /// Invoked when the dragging operation is complete, signaling the receiver to perform any necessary clean-up.
    fn conclude_drag_operation(&self, info: DragInfo) {}

    /// Invoked when the dragged image exits the destination’s bounds rectangle (in the case of a view) or its frame 
    /// rectangle (in the case of a window object).
    fn dragging_exited(&self, info: DragInfo) {}
}
