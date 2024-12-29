use bitmask_enum::bitmask;
use block::ConcreteBlock;

use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::events::EventType;
use crate::foundation::{id, nil, NSInteger, NSPoint, NSString, NSUInt16};

mod test;

/// An EventMask describes the type of event.
#[bitmask(u64)]
pub enum EventMask {
    LeftMouseDown = 1 << 1,
    LeftMouseUp = 1 << 2,
    RightMouseDown = 1 << 3,
    RightMouseUp = 1 << 4,
    MouseMoved = 1 << 5,
    LeftMouseDragged = 1 << 6,
    RightMouseDragged = 1 << 7,
    MouseEntered = 1 << 8,
    MouseExited = 1 << 9,
    KeyDown = 1 << 10,
    KeyUp = 1 << 11,
    FlagsChanged = 1 << 12,
    AppKitDefined = 1 << 13,
    SystemDefined = 1 << 14,
    ApplicationDefined = 1 << 15,
    Periodic = 1 << 16,
    CursorUpdate = 1 << 17,

    ScrollWheel = 1 << 22,
    TabletPoint = 1 << 23,
    TabletProximity = 1 << 24,
    OtherMouseDown = 1 << 25,
    OtherMouseUp = 1 << 26,
    OtherMouseDragged = 1 << 27,

    Gesture = 1 << 29,
    Magnify = 1 << 30,
    Swipe = 1 << 31,
    Rotate = 1 << 18,
    BeginGesture = 1 << 19,
    EndGesture = 1 << 20,

    SmartMagnify = 1 << 32,
    QuickLook = 1 << 33,
    Pressure = 1 << 34,
    DirectTouch = 1 << 37,

    ChangeMode = 1 << 38
}

/// A wrapper over an `NSEvent`.
#[derive(Debug)]
pub struct EventMonitor(pub Id<Object, Owned>);

/// A wrapper over an `NSEvent`.
#[derive(Debug)]
pub struct Event(pub Id<Object, Owned>);

impl Event {
    pub(crate) fn new(objc: id) -> Self {
        Event(unsafe { Id::retain(objc).unwrap() })
    }

    /// The event's type.
    ///
    /// Corresponds to the `type` getter.
    pub fn kind(&self) -> EventType {
        let kind: NSUInteger = unsafe { msg_send![&*self.0, type] };

        unsafe { ::std::mem::transmute(kind) }
    }

    /// The characters associated with a key-up or key-down event.
    pub fn characters(&self) -> String {
        // @TODO: Check here if key event, invalid otherwise.
        // @TODO: Figure out if we can just return &str here, since the Objective-C side
        // should... make it work, I think.
        let characters = NSString::retain(unsafe { msg_send![&*self.0, characters] });

        characters.to_string()
    }

    /// An integer bit field that indicates the pressed modifier keys
    pub fn modifier_flags(&self) -> EventModifierBitFlag {
        let flags: NSUInteger = unsafe { msg_send![&*self.0, modifierFlags] };

        flags.into()
    }

    /// A raw integer bit field that indicates the pressed modifier keys
    pub fn modifier_flags_raw(&self) -> NSUInteger {
        unsafe { msg_send![&*self.0, modifierFlags] }
    }

    /// The virtual code for the key associated with the event
    pub fn key_code(&self) -> NSUInt16 {
        // @TODO: Check here if key event, invalid otherwise.
        // @TODO: Figure out if we can just return &str here, since the Objective-C side
        // should... make it work, I think.
        unsafe { msg_send![&*self.0, keyCode] }
    }

    /// The indices of the currently pressed mouse buttons.
    pub fn pressed_mouse_buttons() -> NSUInteger {
        unsafe { msg_send![class!(NSEvent), pressedMouseButtons] }
    }

    /// Reports the current mouse position in screen coordinates.
    pub fn mouse_location() -> NSPoint {
        unsafe { msg_send![class!(NSEvent), mouseLocation] }
    }

    /// The button number for a mouse event.
    pub fn button_number(&self) -> NSInteger {
        unsafe { msg_send![&*self.0, buttonNumber] }
    }

    /// The number of mouse clicks associated with a mouse-down or mouse-up event.
    pub fn click_count(&self) -> NSInteger {
        unsafe { msg_send![&*self.0, clickCount] }
    }

    /*pub fn contains_modifier_flags(&self, flags: &[EventModifierFlag]) -> bool {
        let modifier_flags: NSUInteger = unsafe {
            msg_send![&*self.0, modifierFlags]
        };

        for flag in flags {
            let f: NSUInteger = flag.into();

        }

        false
    }*/

    /// Register an event handler with the local system event stream. This method
    /// watches for events that occur _within the application_. Events outside
    /// of the application require installing a `global_monitor` handler.
    ///
    /// Note that in order to monitor all possible events, both local and global
    /// monitors are required - the streams don't mix.
    pub fn local_monitor<F>(mask: EventMask, handler: F) -> EventMonitor
    where
        F: Fn(Event) -> Option<Event> + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |event: id| {
            let evt = Event::new(event);

            match handler(evt) {
                Some(mut evt) => &mut *evt.0,
                None => nil
            }
        });
        let block = block.copy();

        EventMonitor(unsafe {
            msg_send_id![
                class!(NSEvent),
                addLocalMonitorForEventsMatchingMask: mask.bits,
                handler: &*block,
            ]
        })
    }

    /// Register an event handler with the global system event stream. This method
    /// watches for events that occur _outside the application_. Events within
    /// the application require installing a `local_monitor` handler.
    ///
    /// Note that in order to monitor all possible events, both local and global
    /// monitors are required - the streams don't mix.
    pub fn global_monitor<F>(mask: EventMask, handler: F) -> EventMonitor
    where
        F: Fn(Event) -> Option<Event> + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |event: id| {
            let evt = Event::new(event);

            match handler(evt) {
                Some(mut evt) => &mut *evt.0,
                None => nil
            }
        });
        let block = block.copy();

        EventMonitor(unsafe {
            msg_send_id![
                class!(NSEvent),
                addGlobalMonitorForEventsMatchingMask: mask.bits,
                handler: &*block,
            ]
        })
    }
}

use crate::foundation::NSUInteger;

#[derive(Clone, Copy, Debug)]
pub enum EventModifierFlag {
    CapsLock,
    Shift,
    Control,
    Option,
    Command,
    Numpad,
    Help,
    Function,
    DeviceIndependentFlagsMask
}

impl From<EventModifierFlag> for NSUInteger {
    fn from(flag: EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Shift => 1 << 17,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::Numpad => 1 << 21,
            EventModifierFlag::Help => 1 << 22,
            EventModifierFlag::Function => 1 << 23,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000
        }
    }
}

impl From<&EventModifierFlag> for NSUInteger {
    fn from(flag: &EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Shift => 1 << 17,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::Numpad => 1 << 21,
            EventModifierFlag::Help => 1 << 22,
            EventModifierFlag::Function => 1 << 23,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000
        }
    }
}

// #[cfg(target_pointer_width = "32")] //@TODO contitional compilation fails, so always use 64
// #[bitmask(u32)]
// #[cfg(target_pointer_width = "64")]
/// Flags that indicate which modifier keys are in the mix for an event.
#[bitmask(u64)]
#[bitmask_config(inverted_flags)]
pub enum EventModifierBitFlag {
    CapsLock     = 1 << 16,
    LeftShift    =(1 << 17) + (1 <<  1),
    RightShift   =(1 << 17) + (1 <<  2),
    LeftControl  =(1 << 18) + (1 <<  0),
    RightControl =(1 << 18) + (1 << 13),
    LeftOption   =(1 << 19) + (1 <<  5),
    RightOption  =(1 << 19) + (1 <<  6),
    LeftCommand  =(1 << 20) + (1 <<  3),
    RightCommand =(1 << 20) + (1 <<  4),
    Shift        = Self::LeftShift  .bits | Self::RightShift  .bits,
    Control      = Self::LeftControl.bits | Self::RightControl.bits,
    Option       = Self::LeftOption .bits | Self::RightOption .bits,
    Command      = Self::LeftCommand.bits | Self::RightCommand.bits,
    LeftModi     = Self::LeftShift  .bits | Self::LeftControl .bits | Self::LeftOption .bits | Self::LeftCommand .bits,
    RightModi    = Self::RightShift .bits | Self::RightControl.bits | Self::RightOption.bits | Self::RightCommand.bits,
    LRModi       = Self::LeftModi   .bits | Self::RightModi   .bits,
    Numpad       = 1 << 21,
    Help         = 1 << 22,
    Function     = 1 << 23,
    DeviceIndependentFlagsMask = 0xffff0000,
}

use std::fmt;
impl fmt::Display for EventModifierBitFlag {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    // key combos with flagmasks don't make sense? so just print the mask and return, ignoring other bits
    if self.contains(EventModifierBitFlag::DeviceIndependentFlagsMask)    {write!(f,"!🖮")?;return fmt::Result::Ok(())};

    if self.contains(EventModifierBitFlag::CapsLock         ) {write!(f,"⇪")?;}
    if self.contains(EventModifierBitFlag::Shift            ) {write!(f,"‹⇧›")?;
    } else                                                  {
        if self.contains(EventModifierBitFlag::LeftShift    ) {write!(f,"‹⇧")?;}
        if self.contains(EventModifierBitFlag::RightShift   ) {write!(f,"⇧›")?;}
    }                                                       ;
    if self.contains(EventModifierBitFlag::Control          ) {write!(f,"‹⌃›")?;
    } else                                                  {
        if self.contains(EventModifierBitFlag::LeftControl  ) {write!(f,"‹⌃")?;}
        if self.contains(EventModifierBitFlag::RightControl ) {write!(f,"⌃›")?;}
    }                                                       ;
    if self.contains(EventModifierBitFlag::Option           ) {write!(f,"‹⌥›")?;
    } else                                                  {
        if self.contains(EventModifierBitFlag::LeftOption   ) {write!(f,"‹⌥")?;}
        if self.contains(EventModifierBitFlag::RightOption  ) {write!(f,"⌥›")?;}
    }                                                       ;
    if self.contains(EventModifierBitFlag::Command          ) {write!(f,"‹⌘›")?;
    } else                                                  {
        if self.contains(EventModifierBitFlag::LeftCommand  ) {write!(f,"‹⌘")?;}
        if self.contains(EventModifierBitFlag::RightCommand ) {write!(f,"⌘›")?;}
    }                                                       ;
    if self.contains(EventModifierBitFlag::Function         ) {
        if f.alternate()                                      {write!(f,"🌐")?; // when it's a modifier key
        } else                                                {write!(f,"ƒ")?;}}
    if self.contains(EventModifierBitFlag::Numpad           ) {
        if f.alternate()                                      {write!(f,"⇭")?; // when it's a modifier key
        } else                                                {write!(f,"🔢")?;}}
    if self.contains(EventModifierBitFlag::Help         ) {write!(f,"ℹ")?;}
    fmt::Result::Ok(())
  }
}
