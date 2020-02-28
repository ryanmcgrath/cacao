//! Hoists a basic NSView. In our current particular use case,
//! this is primarily used as the ContentView for a window. From there,
//! we configure an NSToolbar and WKWebview on top of them.

use std::sync::Once;

use cocoa::base::{id, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use objc::{msg_send, sel, sel_impl};

/// A trait for handling the view lifecycle.
pub trait View {
    fn did_load(&mut self) {}
}

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
pub struct View {
    pub inner: Id<Object>
}

impl View {
    /// Creates a new `NSWindow` instance, configures it appropriately (e.g, titlebar appearance),
    /// injects an `NSObject` delegate wrapper, and retains the necessary Objective-C runtime
    /// pointers.
    pub fn new() -> Self {
        let inner = unsafe {
            let rect_zero = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
            let alloc: id = msg_send![register_class(), alloc];
            let view: id = msg_send![alloc, initWithFrame:rect_zero];
            let _: () = msg_send![view, setWantsLayer:YES];
            let _: () = msg_send![view, setLayerContentsRedrawPolicy:1];
            Id::from_ptr(view)
        };

        View {
            inner: inner
        }
    }
}

/// This is used for some specific calls, where macOS NSView needs to be
/// forcefully dragged into the modern age (e.g, position coordinates from top left...).
extern fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// Registers an `NSView` subclass, and configures it to hold some ivars for various things we need
/// to store.
fn register_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSView").unwrap();
        let mut decl = ClassDecl::new("SBAView", superclass).unwrap();
        
        // Force NSView to render from the top-left, not bottom-left
        decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Request optimized backing layers
        decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
