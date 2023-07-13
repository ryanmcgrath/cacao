use bitmask_enum::bitmask;
use block::ConcreteBlock;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::events::EventType;
use crate::foundation::{id, nil, NSString};

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

    ChangeMode = 1 << 38,
}

/// A wrapper over an `NSEvent`.
#[derive(Debug)]
pub struct EventMonitor(pub Id<Object>);

/// A wrapper over an `NSEvent`.
#[derive(Debug)]
pub struct Event(pub Id<Object>);

impl Event {
    pub(crate) fn new(objc: id) -> Self {
        Event(unsafe { Id::from_ptr(objc) })
    }

    /// Corresponds to the `type` getter
    pub fn event_type(&self) -> EventType {
        let kind: NSUInteger = unsafe { msg_send![&*self.0, type] };

        unsafe { ::std::mem::transmute(kind) }
    }

    pub fn characters(&self) -> String {
        // @TODO: Check here if key event, invalid otherwise.
        // @TODO: Figure out if we can just return &str here, since the Objective-C side
        // should... make it work, I think.
        let characters = NSString::retain(unsafe { msg_send![&*self.0, characters] });

        characters.to_string()
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
        F: Fn(Event) -> Option<Event> + Send + Sync + 'static,
    {
        let block = ConcreteBlock::new(move |event: id| {
            let evt = Event::new(event);

            match handler(evt) {
                Some(mut evt) => &mut *evt.0,
                None => nil,
            }
        });
        let block = block.copy();

        EventMonitor(unsafe {
            msg_send![class!(NSEvent), addLocalMonitorForEventsMatchingMask:mask.bits
                handler:block]
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
        F: Fn(Event) -> Option<Event> + Send + Sync + 'static,
    {
        let block = ConcreteBlock::new(move |event: id| {
            let evt = Event::new(event);

            match handler(evt) {
                Some(mut evt) => &mut *evt.0,
                None => nil,
            }
        });
        let block = block.copy();

        EventMonitor(unsafe {
            msg_send![class!(NSEvent), addGlobalMonitorForEventsMatchingMask:mask.bits
                handler:block]
        })
    }
}

use crate::foundation::NSUInteger;

#[derive(Clone, Copy, Debug)]
pub enum EventModifierFlag {
    CapsLock,
    Control,
    Option,
    Command,
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
