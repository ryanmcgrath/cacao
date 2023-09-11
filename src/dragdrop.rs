//! This module contains various bits and pieces for drag and drop operations. They're shared
//! across the codebase, hence why they're here - they're not currently exhaustive, so feel free to
//! tinker and pull request.

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{msg_send, sel};

use crate::foundation::NSUInteger;
use crate::pasteboard::Pasteboard;

/// Represents operations that can happen for a given drag/drop scenario.
#[derive(Copy, Clone, Debug)]
pub enum DragOperation {
    /// No drag operations are allowed.
    None,

    /// The data represented by the image can be copied.
    Copy,

    /// The data can be shared.
    Link,

    /// The operation can be defined by the destination.
    Generic,

    /// The operation is negotiated privately between the source and the destination.
    Private,

    /// The data can be moved.
    Move,

    /// The data can be deleted.
    Delete // All of the above.
           // @TODO: NSUIntegerMax, a tricky beast
           // Every
}

impl From<DragOperation> for NSUInteger {
    fn from(op: DragOperation) -> Self {
        match op {
            DragOperation::None => 0,
            DragOperation::Copy => 1,
            DragOperation::Link => 2,
            DragOperation::Generic => 4,
            DragOperation::Private => 8,
            DragOperation::Move => 16,
            DragOperation::Delete => 32
        }
    }
}

/// A wrapper for `NSDraggingInfo`. As this is a protocol/type you should never create yourself,
/// this only provides getters - merely a Rust-y way to grab what you need.
#[derive(Clone, Debug)]
pub struct DragInfo {
    pub info: Id<Object, Shared>
}

impl DragInfo {
    /// Returns a wrapped Pasteboard instance, enabling you to get the contents of whatever is
    /// being pasted/dragged/dropped/etc.
    ///
    /// Note: in general, you should not store pasteboards.
    pub fn get_pasteboard(&self) -> Pasteboard {
        unsafe { Pasteboard::with(msg_send![&*self.info, draggingPasteboard]) }
    }
}
