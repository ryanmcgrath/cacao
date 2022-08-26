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
    DeviceIndependentFlagsMask,
}

impl From<EventModifierFlag> for NSUInteger {
    fn from(flag: EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000,
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
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000,
        }
    }
}

/// Represents an event type that you can request to be notified about.
#[derive(Clone, Copy, Debug)]
pub enum EventType {
    /// A keydown event.
    KeyDown,
}
