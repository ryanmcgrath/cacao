//! Wraps `NSButton` on macOS, and `UIButton` on iOS and tvOS.
//!
//! You'd use this type to create a button that a user can interact with. Buttons can be configured
//! a number of ways, and support setting a callback to fire when they're clicked or tapped.
//!
//! Some properties are platform-specific; see the documentation for further information.
//!
//! ```rust,no_run
//! let mut button = Button::new("My button title");
//! button.set_text_equivalent("c");
//!
//! button.set_action(|| {
//!     println!("My button was clicked.");
//! });
//!
//! // Make sure you don't let your Button drop for as long as you need it.
//! my_view.add_subview(&button);
//! ```

use std::fmt;
use std::sync::Once;

use std::cell::RefCell;
use std::rc::Rc;

use objc_id::ShareId;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::color::Color;
use crate::image::Image;
use crate::foundation::{id, nil, BOOL, YES, NO, NSString, NSUInteger};
use crate::invoker::TargetActionHandler;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::text::{AttributedString, Font};
use crate::utils::{load, properties::ObjcProperty};

#[cfg(feature = "macos")]
use crate::macos::FocusRingType;

mod enums;
pub use enums::*;

/// Wraps `NSButton` on macOS, and `UIButton` on iOS and tvOS.
///
/// You'd use this type to create a button that a user can interact with. Buttons can be configured
/// a number of ways, and support setting a callback to fire when they're clicked or tapped.
/// 
/// Some properties are platform-specific; see the documentation for further information.
///
/// ```rust,no_run
/// let mut button = Button::new("My button title");
/// button.set_text_equivalent("c");
///
/// button.set_action(|| {
///     println!("My button was clicked.");
/// });
///
/// // Make sure you don't let your Button drop for as long as you need it.
/// my_view.add_subview(&button);
/// ```
#[derive(Debug)]
pub struct Button {
    /// A handle for the underlying Objective-C object.
    pub objc: ObjcProperty,

    /// A reference to an image, if set. We keep a copy to avoid any ownership snafus.
    pub image: Option<Image>,

    handler: Option<TargetActionHandler>,
    
    /// A pointer to the Objective-C runtime top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    pub right: LayoutAnchorX,

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
            let button: id = msg_send![register_class(), buttonWithTitle:&*title
                target:nil
                action:nil
            ];

            let _: () = msg_send![button, setWantsLayer:YES];
            let _: () = msg_send![button, setTranslatesAutoresizingMaskIntoConstraints:NO];
            button
        };
        
        Button {
            handler: None,
            image: None,
            top: LayoutAnchorY::top(view),
            left: LayoutAnchorX::left(view),
            leading: LayoutAnchorX::leading(view),
            right: LayoutAnchorX::right(view),
            trailing: LayoutAnchorX::trailing(view),
            bottom: LayoutAnchorY::bottom(view),
            width: LayoutAnchorDimension::width(view),
            height: LayoutAnchorDimension::height(view),
            center_x: LayoutAnchorX::center(view),
            center_y: LayoutAnchorY::center(view),
            objc: ObjcProperty::retain(view),
        }
    }

    /// Sets an image on the underlying button.
    pub fn set_image(&mut self, image: Image) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setImage:&*image.0];
        });

        self.image = Some(image);
    }

    /// Sets the bezel style for this button. Only supported on macOS.
    #[cfg(feature = "macos")]
    pub fn set_bezel_style(&self, bezel_style: BezelStyle) {
        let style: NSUInteger = bezel_style.into();
        
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setBezelStyle:style];
        });
    }

    /// Attaches a callback for button press events. Don't get too creative now...
    /// best just to message pass or something.
    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        // @TODO: This probably isn't ideal but gets the job done for now; needs revisiting.
        let this = self.objc.get(|obj| unsafe { ShareId::from_ptr(msg_send![obj, self]) });
        let handler = TargetActionHandler::new(&*this, action);
        self.handler = Some(handler);
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();
        
        #[cfg(feature = "macos")]
        self.objc.with_mut(|obj| unsafe {
            let cell: id = msg_send![obj, cell];
            let _: () = msg_send![cell, setBackgroundColor:color];
        });
    }

    /// Set a key to be bound to this button. When the key is pressed, the action coupled to this
    /// button will fire.
    pub fn set_key_equivalent(&self, key: &str) {
        let key = NSString::new(key);

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setKeyEquivalent:&*key];
        });
    }

    /// Sets the text color for this button.
    ///
    /// On macOS, this is done by way of an `AttributedString` under the hood. 
    pub fn set_text_color<C: AsRef<Color>>(&self, color: C) {
        #[cfg(feature = "macos")]
        self.objc.with_mut(move |obj| unsafe {
            let text: id = msg_send![obj, attributedTitle];
            let len: isize = msg_send![text, length];
            
            let mut attr_str = AttributedString::wrap(text);
            attr_str.set_text_color(color.as_ref(), 0..len);
            
            let _: () = msg_send![obj, setAttributedTitle:&*attr_str];
        });
    }

    // @TODO: Figure out how to handle oddities like this.
    /// For buttons on macOS, one might need to disable the border. This does that.
    #[cfg(feature = "macos")]
    pub fn set_bordered(&self, is_bordered: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setBordered:match is_bordered {
                true => YES,
                false => NO
            }];
        });
    }

    /// Sets the font for this button.
    pub fn set_font<F: AsRef<Font>>(&self, font: F) {
        let font = font.as_ref().clone();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setFont:&*font];
        });
    }

    /// Sets how the control should draw a focus ring when a user is focused on it.
    ///
    /// This is a macOS-only method.
    #[cfg(feature = "macos")]
    pub fn set_focus_ring_type(&self, focus_ring_type: FocusRingType) {
        let ring_type: NSUInteger = focus_ring_type.into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setFocusRingType:ring_type];
        });
    }

    /// Toggles the highlighted status of the button.
    pub fn set_highlighted(&self, highlight: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, highlight:match highlight {
                true => YES,
                false => NO
            }];
        });
    }
}

impl Layout for Button {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }
}

impl Layout for &Button {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }
}

impl Drop for Button {
    /// Nils out references on the Objective-C side and removes this from the backing view.
    // Just to be sure, let's... nil these out. They should be weak references,
    // but I'd rather be paranoid and remove them later.
    fn drop(&mut self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTarget:nil];
            let _: () = msg_send![obj, setAction:nil];
        });
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
