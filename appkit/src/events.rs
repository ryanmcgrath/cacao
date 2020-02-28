//! Hoists some type definitions in a way that I personally find cleaner than what's in the Servo
//! code.

#[allow(non_upper_case_globals, non_snake_case)]
pub mod NSEventModifierFlag {
    use cocoa::foundation::NSUInteger;

    /// Indicates the Caps Lock key has been pressed.
    pub const CapsLock: NSUInteger = 1 << 16;

    /// Indicates the Control key has been pressed.
    pub const Control: NSUInteger = 1 << 18;

    /// Indicates the Option key has been pressed.
    pub const Option: NSUInteger = 1 << 19;

    /// Indicates the Command key has been pressed.
    pub const Command: NSUInteger = 1 << 20;

    /// Indicates device-independent modifier flags are in play.
    pub const DeviceIndependentFlagsMask: NSUInteger = 0xffff0000;
}

#[allow(non_upper_case_globals, non_snake_case)]
mod NSEventType {
    pub const KeyDown: usize = 10;
}
