use objc::{class, msg_send, sel};

use crate::foundation::id;

/// Represents a type of cursor that you can associate with mouse movement.
/// @TODO: Loading?
#[derive(Debug)]
pub enum CursorType {
    /// A standard arrow.
    Arrow,

    /// Current Cusrosr
    Current,

    /// Current System cursor
    CurrentSystem,

    /// A crosshair.
    Crosshair,

    /// A closed hand, typically for mousedown and drag.
    ClosedHand,

    /// An open hand, typically for indicating draggable.
    OpenHand,

    /// A pointing hand, like clicking a link.
    PointingHand,

    /// Indicator that something can be resized to the left.
    ResizeLeft,

    /// Indicator that something can be resized to the right.
    ResizeRight,

    /// Indicator  that something can be resized on the horizontal axis.
    ResizeLeftRight,

    /// Indicates that something can be resized up.
    ResizeUp,

    /// Indicates that something can be resized down.
    ResizeDown,

    /// Indicator  that something can be resized on the vertical axis.
    ResizeUpDown,

    /// Otherwise known as the "poof" or "cloud" cursor. Indicates something will vanish, like
    /// dragging into the Trash.
    DisappearingItem,

    /// Indicate an insertion point, like for text.
    IBeam,

    /// The vertical version of `CursorType::IBeam`.
    IBeamVertical,

    /// Indicates an operation is illegal.
    OperationNotAllowed,

    /// The drag link cursor.
    DragLink,

    /// Used for drag-and-drop usually, will displayu the standard "+" icon next to the cursor.
    DragCopy,

    /// Indicates a context menu will open.
    ContextMenu
}

/// A wrapper around NSCursor.
///
/// You use then when you need to control how the cursor (pointer) should appear. Like `NSCursor`,
/// this is stack based - you push, and you pop. You are responsible for ensuring that this is
/// correctly popped!
///
/// For a very abbreviated example:
///
/// ```rust,no_run
/// use cacao::appkit::Cursor;
/// use cacao::appkit::CursorType;
/// use cacao::dragdrop::DragInfo;
/// use cacao::dragdrop::DragOperation;
/// use cacao::view::ViewDelegate;
/// struct MyView;
/// impl ViewDelegate for MyView {
///     const NAME: &'static str = "RootView";
///     fn dragging_entered(&self, _info: DragInfo) -> DragOperation {
///         Cursor::push(CursorType::DragCopy);
///         DragOperation::Copy
///     }
///
///     fn dragging_exited(&self, _info: DragInfo) {
///         Cursor::pop();
///     }
/// }
/// ```
///
/// This will show the "add files +" indicator when the user has entered the dragging threshold
/// with some items that trigger it, and undo the cursor when the user leaves (regardless of drop
/// status).
#[derive(Debug)]
pub struct Cursor;

impl Cursor {
    /// Given a cursor type, will make it the system cursor.
    /// The inverse of this call, which you should call when ready, is `pop()`.
    pub fn push(cursor_type: CursorType) {
        unsafe {
            let cursor: id = match cursor_type {
                CursorType::Arrow => msg_send![class!(NSCursor), arrowCursor],
                CursorType::Current => msg_send![class!(NSCursor), currentCursor],
                CursorType::CurrentSystem => msg_send![class!(NSCursor), currentSystemCursor],
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

    /// Sets the cursor to hidden, but will reveal it if the user moves the mouse.
    ///
    /// Potentially useful for games and other immersive experiences.
    ///
    /// If you use this, do _not_ use `unhide` - just call this with the inverted boolean value.
    /// Trying to invert this with `unhide` will result in undefined system behavior.
    pub fn set_hidden_until_mouse_moves(status: bool) {
        unsafe {
            let _: () = msg_send![class!(NSCursor), setHiddenUntilMouseMoves: status];
        }
    }
}
