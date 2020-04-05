//! Various traits used for Views.

use crate::dragdrop::{DragInfo, DragOperation};
use crate::view::View;

pub trait ViewDelegate {
    /// Called when the View is ready to work with. You're passed a `ViewHandle` - this is safe to
    /// store and use repeatedly, but it's not thread safe - any UI calls must be made from the
    /// main thread!
    fn did_load(&self, _view: View) {}

    /// Called when this is about to be added to the view heirarchy.
    fn will_appear(&self, animated: bool) {}

    /// Called after this has been added to the view heirarchy.
    fn did_appear(&self, animated: bool) {}

    /// Called when this is about to be removed from the view heirarchy.
    fn will_disappear(&self, animated: bool) {}

    /// Called when this has been removed from the view heirarchy.
    fn did_disappear(&self, animated: bool) {}

    /// Invoked when the dragged image enters destination bounds or frame; returns dragging operation to perform.
    fn dragging_entered(&self, _info: DragInfo) -> DragOperation { DragOperation::None }
    
    /// Invoked when the image is released, allowing the receiver to agree to or refuse drag operation.
    fn prepare_for_drag_operation(&self, _info: DragInfo) -> bool { false }

    /// Invoked after the released image has been removed from the screen, signaling the receiver to import the pasteboard data.
    fn perform_drag_operation(&self, _info: DragInfo) -> bool { false }

    /// Invoked when the dragging operation is complete, signaling the receiver to perform any necessary clean-up.
    fn conclude_drag_operation(&self, _info: DragInfo) {}

    /// Invoked when the dragged image exits the destination’s bounds rectangle (in the case of a view) or its frame 
    /// rectangle (in the case of a window object).
    fn dragging_exited(&self, _info: DragInfo) {}
}