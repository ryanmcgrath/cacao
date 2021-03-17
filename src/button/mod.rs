//! A wrapper for NSButton. Currently the epitome of jank - if you're poking around here, expect
//! that this will change at some point.

use std::fmt;
use std::sync::Once;

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
use crate::utils::load;

#[cfg(feature = "macos")]
use crate::macos::FocusRingType;

/// A wrapper for `NSButton`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSButton` lives.
#[derive(Debug)]
pub struct Button {
    /// A handle for the underlying Objective-C object.
    pub objc: ShareId<Object>,

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
            objc: unsafe { ShareId::from_ptr(view) },
        }
    }

    /// Sets an image on the underlying button.
    pub fn set_image(&mut self, image: Image) {
        unsafe {
            let _: () = msg_send![&*self.objc, setImage:&*image.0];
        }

        self.image = Some(image);
    }

    /// Sets the bezel style for this button.
    #[cfg(feature = "macos")]
    pub fn set_bezel_style(&self, bezel_style: BezelStyle) {
        let style: NSUInteger = bezel_style.into();

        unsafe {
            let _: () = msg_send![&*self.objc, setBezelStyle:style];
        }
    }

    /// Attaches a callback for button press events. Don't get too creative now...
    /// best just to message pass or something.
    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        let handler = TargetActionHandler::new(&*self.objc, action);
        self.handler = Some(handler);
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();
        
        #[cfg(feature = "macos")]
        unsafe {
            let cell: id = msg_send![&*self.objc, cell];
            let _: () = msg_send![cell, setBackgroundColor:color];
        }
    }

    /// Set a key to be bound to this button. When the key is pressed, the action coupled to this
    /// button will fire.
    pub fn set_key_equivalent(&self, key: &str) {
        let key = NSString::new(key);

        unsafe {
            let _: () = msg_send![&*self.objc, setKeyEquivalent:&*key];
        }
    }

    /// Sets the text color for this button.
    ///
    /// On macOS, this is done by way of an `AttributedString` under the hood. 
    pub fn set_text_color<C: AsRef<Color>>(&self, color: C) {
        #[cfg(feature = "macos")]
        unsafe {
            let text: id = msg_send![&*self.objc, attributedTitle];
            let len: isize = msg_send![text, length];
            
            let mut attr_str = AttributedString::wrap(text);
            attr_str.set_text_color(color, 0..len);
            
            let _: () = msg_send![&*self.objc, setAttributedTitle:&*attr_str];
        }
    }

    // @TODO: Figure out how to handle oddities like this.
    /// For buttons on macOS, one might need to disable the border. This does that.
    #[cfg(feature = "macos")]
    pub fn set_bordered(&self, is_bordered: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setBordered:match is_bordered {
                true => YES,
                false => NO
            }];
        }
    }

    /// Sets the font for this button.
    pub fn set_font<F: AsRef<Font>>(&self, font: F) {
        let font = font.as_ref().clone();

        unsafe {
            let _: () = msg_send![&*self.objc, setFont:&*font];
        }
    }

    /// Sets how the control should draw a focus ring when a user is focused on it.
    #[cfg(feature = "macos")]
    pub fn set_focus_ring_type(&self, focus_ring_type: FocusRingType) {
        let ring_type: NSUInteger = focus_ring_type.into();

        unsafe {
            let _: () = msg_send![&*self.objc, setFocusRingType:ring_type];
        }
    }

    /// Toggles the highlighted status of the button.
    pub fn set_highlighted(&self, highlight: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, highlight:match highlight {
                true => YES,
                false => NO
            }];
        }
    }
}

impl Layout for Button {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, _view: &V) {
        panic!(r#"
            Tried to add a subview to a Button. This is not allowed in Cacao. 
            If you think this should be supported, open a discussion on the GitHub repo.
        "#);
    }
}

impl Layout for &Button {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, _view: &V) {
        panic!(r#"
            Tried to add a subview to a Button. This is not allowed in Cacao.
            If you think this should be supported, open a discussion on the GitHub repo.
        "#);
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

/// Represents a bezel style for a button. This is a macOS-specific control, and has no effect
/// under iOS or tvOS.
#[cfg(feature = "macos")]
#[derive(Debug)]
pub enum BezelStyle {
    /// A standard circular button.
    Circular,

    /// A standard disclosure style button.
    Disclosure,

    /// The standard looking "Help" (?) button.
    HelpButton,

    /// An inline style, varies across OS's.
    Inline,

    /// A recessed style, varies slightly across OS's.
    Recessed,

    /// A regular square style, with no special styling.
    RegularSquare,

    /// A standard rounded rectangle.
    RoundRect,

    /// A standard rounded button.
    Rounded,

    /// A standard rounded disclosure button.
    RoundedDisclosure,

    /// A shadowless square styl.e
    ShadowlessSquare,

    /// A small square style.
    SmallSquare,

    /// A textured rounded style.
    TexturedRounded,

    /// A textured square style.
    TexturedSquare,

    /// Any style that's not known by this framework (e.g, if Apple 
    /// introduces something new).
    Unknown(NSUInteger)
}

#[cfg(feature = "macos")]
impl From<BezelStyle> for NSUInteger {
    fn from(style: BezelStyle) -> Self {
        match style {
            BezelStyle::Circular => 7,
            BezelStyle::Disclosure => 5,
            BezelStyle::HelpButton => 9,
            BezelStyle::Inline => 15,
            BezelStyle::Recessed => 13,
            BezelStyle::RegularSquare => 2,
            BezelStyle::RoundRect => 12,
            BezelStyle::Rounded => 1,
            BezelStyle::RoundedDisclosure => 14,
            BezelStyle::ShadowlessSquare => 6,
            BezelStyle::SmallSquare => 10,
            BezelStyle::TexturedRounded => 11,
            BezelStyle::TexturedSquare => 8,
            BezelStyle::Unknown(i) => i
        }
    }
}

#[cfg(feature = "macos")]
impl From<NSUInteger> for BezelStyle {
    fn from(i: NSUInteger) -> Self {
        match i {
            7 => Self::Circular,
            5 => Self::Disclosure,
            9 => Self::HelpButton,
            15 => Self::Inline,
            13 => Self::Recessed,
            2 => Self::RegularSquare,
            12 => Self::RoundRect,
            1 => Self::Rounded,
            14 => Self::RoundedDisclosure,
            6 => Self::ShadowlessSquare,
            10 => Self::SmallSquare,
            11 => Self::TexturedRounded,
            8 => Self::TexturedSquare,
            i => Self::Unknown(i)
        }
    }
}
