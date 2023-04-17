//! Wraps `NSButton` on appkit, and `UIButton` on iOS and tvOS.
//!
//! You'd use this type to create a button that a user can interact with. Buttons can be configured
//! a number of ways, and support setting a callback to fire when they're clicked or tapped.
//!
//! Some properties are platform-specific; see the documentation for further information.
//!
//! ```rust,no_run
//! use cacao::button::Button;
//! use cacao::view::View;
//! use crate::cacao::layout::Layout;
//! let mut button = Button::new("My button title");
//! button.set_key_equivalent("c");
//!
//! button.set_action(|| {
//!     println!("My button was clicked.");
//! });
//! let my_view : View<()> = todo!();
//!
//! // Make sure you don't let your Button drop for as long as you need it.
//! my_view.add_subview(&button);
//! ```

use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};
use objc_id::ShareId;

pub use enums::*;

#[cfg(feature = "appkit")]
use crate::appkit::FocusRingType;
use crate::color::Color;
use crate::control::Control;
use crate::foundation::{id, load_or_register_class, nil, NSString, NSUInteger, NO, YES};
use crate::image::Image;
use crate::invoker::TargetActionHandler;
use crate::keys::Key;
use crate::layout::Layout;
#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};
use crate::objc_access::ObjcAccess;
use crate::text::{AttributedString, Font};
use crate::utils::properties::ObjcProperty;

mod enums;

/// Wraps `NSButton` on appkit, and `UIButton` on iOS and tvOS.
///
/// You'd use this type to create a button that a user can interact with. Buttons can be configured
/// a number of ways, and support setting a callback to fire when they're clicked or tapped.
///
/// Some properties are platform-specific; see the documentation for further information.
///
/// ```rust,no_run
/// use cacao::button::Button;
/// use cacao::view::View;
/// use crate::cacao::layout::Layout;
/// let mut button = Button::new("My button title");
/// button.set_key_equivalent("c");
///
/// button.set_action(|| {
///     println!("My button was clicked.");
/// });
/// let my_view : View<()> = todo!();
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
    #[cfg(feature = "autolayout")]
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    #[cfg(feature = "autolayout")]
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    #[cfg(feature = "autolayout")]
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    #[cfg(feature = "autolayout")]
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    #[cfg(feature = "autolayout")]
    pub right: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    #[cfg(feature = "autolayout")]
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    #[cfg(feature = "autolayout")]
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    #[cfg(feature = "autolayout")]
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    #[cfg(feature = "autolayout")]
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

            let _: () = msg_send![button, setWantsLayer: YES];

            #[cfg(feature = "autolayout")]
            let _: () = msg_send![button, setTranslatesAutoresizingMaskIntoConstraints: NO];

            button
        };

        Button {
            handler: None,
            image: None,

            #[cfg(feature = "autolayout")]
            top: LayoutAnchorY::top(view),

            #[cfg(feature = "autolayout")]
            left: LayoutAnchorX::left(view),

            #[cfg(feature = "autolayout")]
            leading: LayoutAnchorX::leading(view),

            #[cfg(feature = "autolayout")]
            right: LayoutAnchorX::right(view),

            #[cfg(feature = "autolayout")]
            trailing: LayoutAnchorX::trailing(view),

            #[cfg(feature = "autolayout")]
            bottom: LayoutAnchorY::bottom(view),

            #[cfg(feature = "autolayout")]
            width: LayoutAnchorDimension::width(view),

            #[cfg(feature = "autolayout")]
            height: LayoutAnchorDimension::height(view),

            #[cfg(feature = "autolayout")]
            center_x: LayoutAnchorX::center(view),

            #[cfg(feature = "autolayout")]
            center_y: LayoutAnchorY::center(view),

            objc: ObjcProperty::retain(view)
        }
    }

    /// Sets an image on the underlying button.
    pub fn set_image(&mut self, image: Image) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setImage:&*image.0];
        });

        self.image = Some(image);
    }

    /// Sets the bezel style for this button. Only supported on appkit.
    #[cfg(feature = "appkit")]
    pub fn set_bezel_style(&self, bezel_style: BezelStyle) {
        let style: NSUInteger = bezel_style.into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setBezelStyle: style];
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

        #[cfg(feature = "appkit")]
        self.objc.with_mut(|obj| unsafe {
            let cell: id = msg_send![obj, cell];
            let _: () = msg_send![cell, setBackgroundColor: color];
        });
    }

    /// Set a key to be bound to this button. When the key is pressed, the action coupled to this
    /// button will fire.
    pub fn set_key_equivalent<'a, K>(&self, key: K)
    where
        K: Into<Key<'a>>
    {
        let key: Key<'a> = key.into();

        self.objc.with_mut(|obj| {
            let keychar = match key {
                Key::Char(s) => NSString::new(s),
                Key::Delete => NSString::new("\u{08}")
            };

            unsafe {
                let _: () = msg_send![obj, setKeyEquivalent:&*keychar];
            }
        });
    }

    /// Sets the text color for this button.
    ///
    /// On appkit, this is done by way of an `AttributedString` under the hood.
    pub fn set_text_color<C: AsRef<Color>>(&self, color: C) {
        #[cfg(feature = "appkit")]
        self.objc.with_mut(move |obj| unsafe {
            let text: id = msg_send![obj, attributedTitle];
            let len: isize = msg_send![text, length];

            let mut attr_str = AttributedString::wrap(text);
            attr_str.set_text_color(color.as_ref(), 0..len);

            let _: () = msg_send![obj, setAttributedTitle:&*attr_str];
        });
    }

    // @TODO: Figure out how to handle oddities like this.
    /// For buttons on appkit, one might need to disable the border. This does that.
    #[cfg(feature = "appkit")]
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
    /// This is an appkit-only method.
    #[cfg(feature = "appkit")]
    pub fn set_focus_ring_type(&self, focus_ring_type: FocusRingType) {
        let ring_type: NSUInteger = focus_ring_type.into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setFocusRingType: ring_type];
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

impl ObjcAccess for Button {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for Button {}

impl Control for Button {}

impl ObjcAccess for &Button {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for &Button {}

impl Control for &Button {}

impl Drop for Button {
    /// Nils out references on the Objective-C side and removes this from the backing view.
    // Just to be sure, let's... nil these out. They should be weak references,
    // but I'd rather be paranoid and remove them later.
    fn drop(&mut self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTarget: nil];
            let _: () = msg_send![obj, setAction: nil];
        });
    }
}

/// Registers an `NSButton` subclass, and configures it to hold some ivars
/// for various things we need to store.
fn register_class() -> *const Class {
    load_or_register_class("NSButton", "RSTButton", |decl| unsafe {})
}
