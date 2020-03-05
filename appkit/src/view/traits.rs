//! Various traits used for Views.

use objc::runtime::Object;

use objc_id::ShareId;

use crate::dragdrop::DragOperation;

pub trait ViewWrapper {
    fn get_handle(&self) -> Option<ShareId<Object>>;
}

pub trait ViewController {
    fn did_load(&self);

    fn dragging_entered(&self) -> DragOperation { DragOperation::None }
    fn prepare_for_drag_operation(&self) -> bool { false }
    fn perform_drag_operation(&self) -> bool { false }
    fn dragging_exited(&self) {}
}
