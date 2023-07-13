//! Hoists some type definitions in a way that I personally find cleaner than what's in the Servo
//! code.

use crate::foundation::NSUInteger;

/// Flags that indicate a key is in the mix for an event.
#[derive(Clone, Copy, Debug)]
pub enum EventModifierFlag {
    /// CapsLock (or shift... oddly named...) is held.
    CapsLock,

    /// Control is held.
    Control,

    /// Option is held.
    Option,

    /// Command (CMD) is held.
    Command,

    /// Device independent flags mask.
    DeviceIndependentFlagsMask
}

impl From<EventModifierFlag> for NSUInteger {
    fn from(flag: EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000
        }
    }
}

impl From<&EventModifierFlag> for NSUInteger {
    fn from(flag: &EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000
        }
    }
}

/// Represents an event type that you can request to be notified about.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
pub enum EventType {
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    MouseMoved = 5,
    LeftMouseDragged = 6,
    RightMouseDragged = 7,
    MouseEntered = 8,
    MouseExited = 9,
    KeyDown = 10,
    KeyUp = 11,
    FlagsChanged = 12,
    AppKitDefined = 13,
    SystemDefined = 14,
    ApplicationDefined = 15,
    Periodic = 16,
    CursorUpdate = 17,

    ScrollWheel = 22,
    TabletPoint = 23,
    TabletProximity = 24,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    OtherMouseDragged = 27,

    Gesture = 29,
    Magnify = 30,
    Swipe = 31,
    Rotate = 18,
    BeginGesture = 19,
    EndGesture = 20,

    SmartMagnify = 32,
    QuickLook = 33,
    Pressure = 34,
    DirectTouch = 37,

    ChangeMode = 38
}
