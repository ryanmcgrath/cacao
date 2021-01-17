//! A wrapper for NSButton. Currently the epitome of jank - if you're poking around here, expect
//! that this will change at some point.

use std::fmt;
use std::sync::Once;

use objc_id::ShareId;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, BOOL, YES, NO, NSString};
use crate::invoker::TargetActionHandler;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::utils::load;

/// A wrapper for `NSButton`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSButton` lives.
#[derive(Debug)]
pub struct Button {
    pub objc: ShareId<Object>,
    handler: Option<TargetActionHandler>,
    
    /// A pointer to the Objective-C runtime top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    pub center_y: LayoutAnchorY
}

impl Button {
    /// Creates a new `NSButton` instance, configures it appropriately,
    /// and retains the necessary Objective-C runtime pointer.
    pub fn new(text: &str) -> Self {
        let title = NSString::new(text);

        let view: id = unsafe {
            let button: id = msg_send![register_class(), buttonWithTitle:title target:nil action:nil];
            let _: () = msg_send![button, setTranslatesAutoresizingMaskIntoConstraints:NO];
            button
        };
        
        Button {
            handler: None,
            top: LayoutAnchorY::new(unsafe { msg_send![view, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![view, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![view, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![view, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![view, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![view, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![view, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![view, centerYAnchor] }),
            objc: unsafe { ShareId::from_ptr(view) },
        }
    }

    /// Sets the bezel style for this button.
    pub fn set_bezel_style(&self, bezel_style: i32) {
        unsafe {
            let _: () = msg_send![&*self.objc, setBezelStyle:bezel_style];
        }
    }

    /// Attaches a callback for button press events. Don't get too creative now...
    /// best just to message pass or something.
    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        let handler = TargetActionHandler::new(&*self.objc, action);
        self.handler = Some(handler);
    }
}

impl Layout for Button {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, view: &V) {
        /*let backing_node = view.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, addSubview:backing_node];
        }*/
    }
}


impl Drop for Button {
    // Just to be sure, let's... nil these out. They should be weak references,
    // but I'd rather be paranoid and remove them later.
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTarget:nil];
            let _: () = msg_send![&*self.objc, setAction:nil];
        }
    }
}

/// Registers an `NSButton` subclass, and configures it to hold some ivars 
/// for various things we need to store.
fn register_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSButton);
        let decl = ClassDecl::new("RSTButton", superclass).unwrap(); 
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
