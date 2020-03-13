//! Implements an `NSWindow` wrapper for MacOS, backed by
//! Cocoa and associated widgets. This also handles looping back
//! lifecycle events, such as window resizing or close events.

use cocoa::base::{id, YES, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSUInteger};

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

#[allow(non_upper_case_globals, non_snake_case)]
pub mod WindowStyle {
    use cocoa::foundation::NSUInteger;

    pub const Borderless: NSUInteger = 0;
    pub const Titled: NSUInteger = 1 << 0;
    pub const Closable: NSUInteger = 1 << 1;
    pub const Miniaturizable: NSUInteger = 1 << 2;
    pub const Resizable: NSUInteger = 1 << 3;
    pub const UnifiedTitleAndToolbar: NSUInteger = 1 << 12;
    pub const FullScreen: NSUInteger = 1 << 14;
    pub const FullSizeContentView: NSUInteger = 1 << 15;
    pub const Utility: NSUInteger = 1 << 4;
    pub const DocModalWindow: NSUInteger = 1 << 6;
    pub const NonActivatingPanel: NSUInteger = 1 << 7;
    pub const HUDWindow: NSUInteger = 1 << 13;
}

pub struct WindowConfig(pub Id<Object>);

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig(unsafe {
            let dimensions = NSRect::new(NSPoint::new(0., 0.), NSSize::new(800., 600.));

            let style = WindowStyle::Resizable | WindowStyle::Miniaturizable | WindowStyle::UnifiedTitleAndToolbar |
                WindowStyle::Closable | WindowStyle::Titled;

            let alloc: id = msg_send![class!(NSWindow), alloc];
            let window: id = msg_send![alloc, initWithContentRect:dimensions styleMask:style backing:2 as NSUInteger defer:YES];
            let _: () = msg_send![window, autorelease];

            let _: () = msg_send![window, setTitlebarAppearsTransparent:NO];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync.
            let _: () = msg_send![window, setReleasedWhenClosed:NO];
        
            Id::from_ptr(window)    
        })
    }
}
