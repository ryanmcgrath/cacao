//! Various traits used for Views.

use crate::dragdrop::{DragInfo, DragOperation};
use crate::view::View;

/// This trait can be used for implementing custom View behavior. You implement this trait on your
/// struct, and wrap your struct in a `View` or `ViewController`. The view or controller then
/// handles interfacing between your struct and system events.
///
/// It winds up feeling to subclassing, without the ability to subclass multiple levels deep and
/// get ultra confusing.
#[allow(unused_variables)]
pub trait ViewDelegate {
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
    fn did_load(&mut self, view: View) {}

    /// Called when this is about to be added to the view heirarchy.
    fn will_appear(&self, animated: bool) {}

    /// Called after this has been added to the view heirarchy.
    fn did_appear(&self, animated: bool) {}

    /// Called when this is about to be removed from the view heirarchy.
    fn will_disappear(&self, animated: bool) {}

    /// Called when this has been removed from the view heirarchy.
    fn did_disappear(&self, animated: bool) {}

    /// Invoked when the dragged image enters destination bounds or frame; returns dragging 
    /// operation to perform.
    fn dragging_entered(&self, info: DragInfo) -> DragOperation { DragOperation::None }
    
    /// Invoked when the image is released, allowing the receiver to agree to or refuse 
    /// drag operation.
    fn prepare_for_drag_operation(&self, info: DragInfo) -> bool { false }

    /// Invoked after the released image has been removed from the screen, signaling the 
    /// receiver to import the pasteboard data.
    fn perform_drag_operation(&self, info: DragInfo) -> bool { false }

    /// Invoked when the dragging operation is complete, signaling the receiver to perform
    /// any necessary clean-up.
    fn conclude_drag_operation(&self, info: DragInfo) {}

    /// Invoked when the dragged image exits the destination’s bounds rectangle (in the case 
    /// of a view) or its frame rectangle (in the case of a window object).
    fn dragging_exited(&self, info: DragInfo) {}

    //fn perform_key_equivalent(&self, event: Event) -> bool { false }
}
