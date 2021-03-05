//! Hoists some type definitions in a way that I personally find cleaner than what's in the Servo
//! code.

use crate::foundation::NSUInteger;

#[derive(Clone, Copy, Debug)]
pub enum EventModifierFlag {
    CapsLock,
    Control,
    Option,
    Command,
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

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    KeyDown
}
