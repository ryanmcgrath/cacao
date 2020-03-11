//! Various traits used for Views.

use objc::runtime::Object;

use objc_id::ShareId;

use crate::dragdrop::{DragInfo, DragOperation};

pub trait ViewWrapper {
    fn get_handle(&self) -> Option<ShareId<Object>>;
}

pub trait ViewController {
    fn did_load(&self);

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
