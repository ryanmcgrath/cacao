//! A wrapper for NSButton. Currently the epitome of jank - if you're poking around here, expect
//! that this will change at some point.

use std::fmt;
use std::sync::Once;

use objc_id::ShareId;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::color::Color;
use crate::foundation::{id, nil, BOOL, YES, NO, NSString, NSUInteger};
use crate::invoker::TargetActionHandler;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::text::Font;
use crate::utils::load;

#[cfg(feature = "macos")]
use crate::macos::FocusRingType;

extern "C" {
    static NSForegroundColorAttributeName: id;
}

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
            let _: () = msg_send![button, setWantsLayer:YES];
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
    pub fn set_background_color(&self, color: Color) {
        let bg = color.into_platform_specific_color();
        
        #[cfg(feature = "macos")]
        unsafe {
            let cell: id = msg_send![&*self.objc, cell];
            let _: () = msg_send![cell, setBackgroundColor:bg];
            /*let cg: id = msg_send![bg, CGColor];
            let layer: id = msg_send![&*self.objc, layer];
            let _: () = msg_send![layer, setBackgroundColor:cg];
            */
        }
    }

    pub fn set_key_equivalent(&self, key: &str) {
        let key = NSString::new(key).into_inner();

        unsafe {
            let _: () = msg_send![&*self.objc, setKeyEquivalent:key];
        }
    }

    pub fn set_text_color(&self, color: Color) {
        let bg = color.into_platform_specific_color();
        
        // @TODO: Clean this up, and look at just using `CFMutableAttributedString` instead
        // to avoid ObjC overhead.
        unsafe {
            let alloc: id = msg_send![class!(NSMutableAttributedString), alloc];
            let s: id = msg_send![&*self.objc, attributedTitle];
            let attributed_string: id = msg_send![alloc, initWithAttributedString:s];
            let len: isize = msg_send![s, length];
            let range = core_foundation::base::CFRange::init(0, len);

            let _: () = msg_send![attributed_string, addAttribute:NSForegroundColorAttributeName value:bg range:range];
            let _: () = msg_send![&*self.objc, setAttributedTitle:attributed_string];
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
    pub fn set_font(&self, font: &Font) {
        unsafe {
            let _: () = msg_send![&*self.objc, setFont:&*font.objc];
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
            Tried to add a subview to a Button. This is not allowed in Cacao. If you think this should be supported, 
            open a discussion on the GitHub repo.
        "#);
    }
}

impl Layout for &Button {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, _view: &V) {
        panic!(r#"
            Tried to add a subview to a Button. This is not allowed in Cacao. If you think this should be supported, 
            open a discussion on the GitHub repo.
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
    Circular,
    Disclosure,
    HelpButton,
    Inline,
    Recessed,
    RegularSquare,
    RoundRect,
    Rounded,
    RoundedDisclosure,
    ShadowlessSquare,
    SmallSquare,
    TexturedRounded,
    TexturedSquare,
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
