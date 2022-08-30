//! Wraps `NSTextField` and `UITextField` across platforms, explicitly as a TextField.
//! In AppKit, `NSTextField` does double duty, and for clarity we just double
//! the implementation.
//!
//! TextFields implement Autolayout, which enable you to specify how things should appear on the screen.
//!
//! ```rust,no_run
//! use cacao::color::Color;
//! use cacao::layout::{Layout, LayoutConstraint};
//! use cacao::input::TextField;
//! use cacao::view::View;
//! use cacao::appkit::window::{Window, WindowDelegate};
//!
//! #[derive(Default)]
//! struct AppWindow {
//!     content: TextField,
//!     label: TextField,
//!     red: View,
//!     window: Window
//! }
//!
//! impl WindowDelegate for AppWindow {
//!     const NAME: &'static str = "RootView";
//!     fn did_load(&mut self, window: Window) {
//!         window.set_minimum_content_size(300., 300.);
//!         self.window = window;
//!
//!         self.label.set_background_color(Color::rgb(224, 82, 99));
//!         self.label.set_text("LOL");
//!         self.content.add_subview(&self.red);
//!
//!         self.window.set_content_view(&self.content);
//!
//!         LayoutConstraint::activate(&[
//!             self.red.top.constraint_equal_to(&self.content.top).offset(16.),
//!             self.red.leading.constraint_equal_to(&self.content.leading).offset(16.),
//!             self.red.trailing.constraint_equal_to(&self.content.trailing).offset(-16.),
//!             self.red.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
//!         ]);
//!     }
//! }
//! ```
//!
//! For more information on Autolayout, view the module or check out the examples folder.

use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl, class};
use objc_id::ShareId;

use crate::color::Color;
use crate::control::Control;
use crate::foundation::{id, nil, NSArray, NSInteger, NSString, NO, YES};
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::text::{Font, TextAlign};
use crate::utils::properties::ObjcProperty;

#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};

#[cfg(feature = "appkit")]
mod appkit;

#[cfg(feature = "appkit")]
use appkit::{register_view_class, register_view_class_with_delegate};

#[cfg(feature = "uikit")]
mod uikit;

#[cfg(all(feature = "uikit", not(feature = "appkit")))]
use uikit::{register_view_class, register_view_class_with_delegate};

mod traits;
pub use traits::TextFieldDelegate;

pub(crate) static TEXTFIELD_DELEGATE_PTR: &str = "rstTextFieldDelegatePtr";

/// A helper method for instantiating view classes and applying default settings to them.
fn common_init(class: *const Class) -> id {
    unsafe {
        let view: id = msg_send![class, new];

        #[cfg(feature = "autolayout")]
        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints: NO];

        #[cfg(feature = "appkit")]
        let _: () = msg_send![view, setWantsLayer: YES];

        view
    }
}

/// A clone-able handler to an `NSTextField/UITextField` reference in the
/// Objective-C runtime.
#[derive(Debug)]
pub struct TextField<T = ()> {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ObjcProperty,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Box<T>>,

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

impl Default for TextField {
    fn default() -> Self {
        TextField::new()
    }
}

impl TextField {
    /// Returns a default `TextField`, suitable for
    pub fn new() -> Self {
        let class = register_view_class();
        let view = common_init(class);

        TextField {
            delegate: None,
            objc: ObjcProperty::retain(view),

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
            center_y: LayoutAnchorY::center(view)
        }
    }
}

impl<T> TextField<T>
where
    T: TextFieldDelegate + 'static
{
    /// Initializes a new TextField with a given `TextFieldDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> TextField<T> {
        let class = register_view_class_with_delegate(&delegate);
        let mut delegate = Box::new(delegate);

        let input = common_init(class);
        unsafe {
            let ptr: *const T = &*delegate;
            (&mut *input).set_ivar(TEXTFIELD_DELEGATE_PTR, ptr as usize);
        };
        #[cfg(feature = "uikit")]
        let _: () = unsafe {msg_send![input, setDelegate: input] };

        let mut input = TextField {
            delegate: None,
            objc: ObjcProperty::retain(input),

            #[cfg(feature = "autolayout")]
            top: LayoutAnchorY::top(input),

            #[cfg(feature = "autolayout")]
            left: LayoutAnchorX::left(input),

            #[cfg(feature = "autolayout")]
            leading: LayoutAnchorX::leading(input),

            #[cfg(feature = "autolayout")]
            right: LayoutAnchorX::right(input),

            #[cfg(feature = "autolayout")]
            trailing: LayoutAnchorX::trailing(input),

            #[cfg(feature = "autolayout")]
            bottom: LayoutAnchorY::bottom(input),

            #[cfg(feature = "autolayout")]
            width: LayoutAnchorDimension::width(input),

            #[cfg(feature = "autolayout")]
            height: LayoutAnchorDimension::height(input),

            #[cfg(feature = "autolayout")]
            center_x: LayoutAnchorX::center(input),

            #[cfg(feature = "autolayout")]
            center_y: LayoutAnchorY::center(input)
        };

        (&mut delegate).did_load(input.clone_as_handle());
        input.delegate = Some(delegate);
        input
    }
}

impl<T> TextField<T> {
    /// An internal method that returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `TextField` is the only true holder of those.
    pub(crate) fn clone_as_handle(&self) -> TextField {
        TextField {
            delegate: None,
            objc: self.objc.clone(),

            #[cfg(feature = "autolayout")]
            top: self.top.clone(),

            #[cfg(feature = "autolayout")]
            leading: self.leading.clone(),

            #[cfg(feature = "autolayout")]
            left: self.left.clone(),

            #[cfg(feature = "autolayout")]
            trailing: self.trailing.clone(),

            #[cfg(feature = "autolayout")]
            right: self.right.clone(),

            #[cfg(feature = "autolayout")]
            bottom: self.bottom.clone(),

            #[cfg(feature = "autolayout")]
            width: self.width.clone(),

            #[cfg(feature = "autolayout")]
            height: self.height.clone(),

            #[cfg(feature = "autolayout")]
            center_x: self.center_x.clone(),

            #[cfg(feature = "autolayout")]
            center_y: self.center_y.clone()
        }
    }

    /// Grabs the value from the textfield and returns it as an owned String.
    #[cfg(feature = "appkit")]
    pub fn get_value(&self) -> String {
        self.objc
            .get(|obj| unsafe { NSString::retain(msg_send![obj, stringValue]).to_string() })
    }
    #[cfg(all(feature = "uikit", not(feature = "appkit")))]
    pub fn get_value(&self) -> String {
        self.objc
            .get(|obj| unsafe { NSString::retain(msg_send![obj, text]).to_string() })
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        self.objc.with_mut(|obj| unsafe {
            let cg = color.as_ref().cg_color();
            let layer: id = msg_send![obj, layer];
            let _: () = msg_send![layer, setBackgroundColor: cg];
        });
    }

    /// Call this to set the text for the label.
    pub fn set_text(&self, text: &str) {
        let s = NSString::new(text);

        self.objc.with_mut(|obj| unsafe {
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setStringValue:&*s];
            #[cfg(all(feature = "uikit", not(feature = "appkit")))]
            let _: () = msg_send![obj, setText:&*s];
        });
    }

    /// Call this to set the text for the label.
    pub fn set_placeholder_text(&self, text: &str) {
        let s = NSString::new(text);

        self.objc.with_mut(|obj| unsafe {
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setPlaceholderString:&*s];
            #[cfg(all(feature = "uikit", not(feature = "appkit")))]
            let _: () = msg_send![obj, setPlaceholder:&*s];
        });
    }

    /// The the text alignment style for this control.
    pub fn set_text_alignment(&self, alignment: TextAlign) {
        self.objc.with_mut(|obj| unsafe {
            let alignment: NSInteger = alignment.into();
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setAlignment: alignment];
            #[cfg(all(feature = "uikit", not(feature = "appkit")))]
            let _: () = msg_send![obj, setTextAlignment: alignment];
        });
    }

    /// Set whether this field operates in single-line mode.
    pub fn set_uses_single_line(&self, uses_single_line: bool) {
        self.objc.with_mut(|obj| unsafe {
            let cell: id = msg_send![obj, cell];
            let _: () = msg_send![cell, setUsesSingleLineMode:match uses_single_line {
                true => YES,
                false => NO
            }];
        });
    }

    /// Set whether this field operates in single-line mode.
    pub fn set_wraps(&self, uses_single_line: bool) {
        self.objc.with_mut(|obj| unsafe {
            let cell: id = msg_send![obj, cell];
            let _: () = msg_send![cell, setWraps:match uses_single_line {
                true => YES,
                false => NO
            }];
        });
    }

    /// Sets the maximum number of lines.
    pub fn set_max_number_of_lines(&self, num: NSInteger) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setMaximumNumberOfLines: num];
        });
    }

    /// Sets the font for this input.
    pub fn set_font<F: AsRef<Font>>(&self, font: F) {
        let font = font.as_ref().clone();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setFont:&*font];
        });
    }
}

impl<T> ObjcAccess for TextField<T> {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl<T> Layout for TextField<T> {}

impl<T> Control for TextField<T> {}

impl<T> Drop for TextField<T> {
    /// A bit of extra cleanup for delegate callback pointers. If the originating `TextField` is being
    /// dropped, we do some logic to clean it all up (e.g, we go ahead and check to see if
    /// this has a superview (i.e, it's in the heirarchy) on the AppKit side. If it does, we go
    /// ahead and remove it - this is intended to match the semantics of how Rust handles things).
    ///
    /// There are, thankfully, no delegates we need to break here.
    fn drop(&mut self) {
        /*if self.delegate.is_some() {
            unsafe {
                let superview: id = msg_send![&*self.objc, superview];
                if superview != nil {
                    let _: () = msg_send![&*self.objc, removeFromSuperview];
                }
            }
        }*/
    }
}

#[test]
fn test_text_view() {
    let text_field = TextField::new();
    let value = text_field.get_value();
    assert!(value.is_empty());
    text_field.set_background_color(Color::SystemBlue);
    text_field.set_text("foobar");
    let value = text_field.get_value();
    assert_eq!(value, "foobar".to_string());
    text_field.set_text_alignment(TextAlign::Left);
    text_field.set_font(Font::default());
}
