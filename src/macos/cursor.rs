
use objc::{class, msg_send, sel, sel_impl};
use crate::foundation::id;

#[derive(Debug)]
pub enum CursorType {
    Arrow,
    Crosshair,
    ClosedHand,
    OpenHand,
    PointingHand,
    
    ResizeLeft,
    ResizeRight,
    ResizeLeftRight,

    ResizeUp,
    ResizeDown,
    ResizeUpDown,

    DisappearingItem,

    IBeam,
    IBeamVertical,

    OperationNotAllowed,
    DragLink,
    DragCopy,
    ContextMenu
}

#[derive(Debug)]
pub struct Cursor;

impl Cursor {
    /// Given a cursor type, will make it the system cursor.
    /// The inverse of this call, which you should call when ready, is `pop()`.
    pub fn push(cursor_type: CursorType) {
        unsafe {
            let cursor: id = match cursor_type {
                CursorType::Arrow => msg_send![class!(NSCursor), arrowCursor],
                CursorType::Crosshair => msg_send![class!(NSCursor), crosshairCursor],
                CursorType::ClosedHand => msg_send![class!(NSCursor), closedHandCursor],
                CursorType::OpenHand => msg_send![class!(NSCursor), openHandCursor],
                CursorType::PointingHand => msg_send![class!(NSCursor), pointingHandCursor],
                CursorType::ResizeLeft => msg_send![class!(NSCursor), resizeLeftCursor],
                CursorType::ResizeRight => msg_send![class!(NSCursor), resizeRightCursor],
                CursorType::ResizeLeftRight => msg_send![class!(NSCursor), resizeLeftRightCursor],
                CursorType::ResizeUp => msg_send![class!(NSCursor), resizeUpCursor],
                CursorType::ResizeDown => msg_send![class!(NSCursor), resizeDownCursor],
                CursorType::ResizeUpDown => msg_send![class!(NSCursor), resizeUpDownCursor],
                CursorType::DisappearingItem => msg_send![class!(NSCursor), disappearingItemCursor],
                CursorType::IBeam => msg_send![class!(NSCursor), IBeamCursor],
                CursorType::IBeamVertical => msg_send![class!(NSCursor), IBeamCursorForVerticalLayout],
                CursorType::OperationNotAllowed => msg_send![class!(NSCursor), operationNotAllowedCursor],
                CursorType::DragLink => msg_send![class!(NSCursor), dragLinkCursor],
                CursorType::DragCopy => msg_send![class!(NSCursor), dragCopyCursor],
                CursorType::ContextMenu => msg_send![class!(NSCursor), contextualMenuCursor]
            };
            
            let _: () = msg_send![cursor, push];
        }
    }

    /// Pops the current cursor off the cursor-stack. The inverse of push.
    pub fn pop() {
        unsafe {
            let _: () = msg_send![class!(NSCursor), pop];
        }
    }

    /// Hides the cursor. Part of a balanced call stack.
    pub fn hide() {
        unsafe {
            let _: () = msg_send![class!(NSCursor), hide];
        }
    }

    /// Un-hides the cursor. Part of a balanced call stack.
    pub fn unhide() {
        unsafe {
            let _: () = msg_send![class!(NSCursor), unhide];
        }
    }
}
