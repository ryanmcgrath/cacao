//! Wraps `NSTextField` and `UILabel` across platforms, explicitly as a Label.
//! In AppKit, `NSTextField` does double duty, and for clarity we just double
//! the implementation.
//!
//! Labels implement Autolayout, which enable you to specify how things should appear on the screen.
//!
//! ```rust
//! use cacao::color::Color;
//! use cacao::layout::{Layout, LayoutConstraint};
//! use cacao::text::Label;
//! use cacao::view::View;
//! use cacao::appkit::window::{Window, WindowDelegate};
//!
//! #[derive(Default)]
//! struct AppWindow {
//!     content: Label,
//!     label: Label,
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

use core_foundation::base::TCFType;

use crate::id_shim::ShareId;
use objc::runtime::{Class, Object};
use objc::{msg_send, sel};

use crate::color::Color;
use crate::foundation::{id, nil, NSArray, NSInteger, NSString, NSUInteger, NO, YES};
use crate::layer::Layer;
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::text::{AttributedString, Font, LineBreakMode, TextAlign};
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
pub use traits::LabelDelegate;

pub(crate) static LABEL_DELEGATE_PTR: &str = "rstLabelDelegatePtr";

/// A helper method for instantiating view classes and applying default settings to them.
fn allocate_view(registration_fn: fn() -> *const Class) -> id {
    unsafe {
        #[cfg(feature = "appkit")]
        let view: id = {
            // This sucks, but for now, sure.
            let blank = NSString::no_copy("");
            let label: id = msg_send![registration_fn(), wrappingLabelWithString:&*blank];

            // We sub this in to get the general expected behavior for 202*.
            let _: () = msg_send![label, setSelectable: NO];

            label
        };

        #[cfg(feature = "uikit")]
        let view: id = msg_send![registration_fn(), new];

        #[cfg(feature = "autolayout")]
        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints: NO];

        #[cfg(feature = "appkit")]
        let _: () = msg_send![view, setWantsLayer: YES];

        view
    }
}

/// A clone-able handler to an `NSTextField/UILabel` reference in the
/// Objective-C runtime.
/// Wraps `NSTextField` and `UILabel` across platforms, explicitly as a Label.
/// In AppKit, `NSTextField` does double duty, and for clarity we just double
/// the implementation.
///
/// Labels implement Autolayout, which enable you to specify how things should appear on the screen.
///
/// ```rust
/// use cacao::color::Color;
/// use cacao::layout::{Layout, LayoutConstraint};
/// use cacao::text::Label;
/// use cacao::appkit::window::{Window, WindowDelegate};
/// use cacao::view::View;
///
/// #[derive(Default)]
/// struct AppWindow {
///     content: Label,
///     label: Label,
///     red: View,
///     window: Window
/// }
///
/// impl WindowDelegate for AppWindow {
///     const NAME: &'static str = "RootView";
///     fn did_load(&mut self, window: Window) {
///         window.set_minimum_content_size(300., 300.);
///         self.window = window;
///
///         self.label.set_background_color(Color::rgb(224, 82, 99));
///         self.label.set_text("LOL");
///         self.content.add_subview(&self.red);
///
///         self.window.set_content_view(&self.content);
///
///         LayoutConstraint::activate(&[
///             self.red.top.constraint_equal_to(&self.content.top).offset(16.),
///             self.red.leading.constraint_equal_to(&self.content.leading).offset(16.),
///             self.red.trailing.constraint_equal_to(&self.content.trailing).offset(-16.),
///             self.red.bottom.constraint_equal_to(&self.content.bottom).offset(-16.),
///         ]);
///     }
/// }
/// ```
///
/// For more information on Autolayout, view the module or check out the examples folder.
#[derive(Debug)]
pub struct Label<T = ()> {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ObjcProperty,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Box<T>>,

    /// References the underlying layer. This is consistent across AppKit & UIKit - in AppKit
    /// we explicitly opt in to layer backed views.
    pub layer: Layer,

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

impl Default for Label {
    fn default() -> Self {
        Label::new()
    }
}

impl Label {
    /// Returns a default `Label`, suitable for
    pub fn new() -> Self {
        let view = allocate_view(register_view_class);
        Self::init(view, None)
    }

    pub(crate) fn init<T>(view: id, delegate: Option<Box<T>>) -> Label<T> {
        Label {
            delegate,

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

            layer: Layer::wrap(unsafe { msg_send![view, layer] }),

            objc: ObjcProperty::retain(view)
        }
    }
}

impl<T> Label<T>
where
    T: LabelDelegate + 'static
{
    /// Initializes a new Label with a given `LabelDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> Label<T> {
        let delegate = Box::new(delegate);

        let view = allocate_view(register_view_class_with_delegate::<T>);
        unsafe {
            let ptr: *const T = &*delegate;
            (&mut *view).set_ivar(LABEL_DELEGATE_PTR, ptr as usize);
        };
        Label::init(view, Some(delegate))
    }
}

impl<T> Label<T> {
    /// An internal method that returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `Label` is the only true holder of those.
    pub(crate) fn clone_as_handle(&self) -> Label {
        Label {
            delegate: None,

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
            center_y: self.center_y.clone(),

            layer: self.layer.clone(),

            objc: self.objc.clone()
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        // @TODO: This is wrong.
        // Needs to set ivar and such, akin to View.
        self.objc.with_mut(|obj| unsafe {
            let color = color.as_ref().cg_color().as_concrete_TypeRef();
            let layer: id = msg_send![obj, layer];
            let _: () = msg_send![layer, setBackgroundColor: color];
        });
    }

    /// Call this to set the color of the text.
    pub fn set_text_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTextColor: color];
        });
    }

    /// Call this to set the text for the label.
    pub fn set_text<S: AsRef<str>>(&self, text: S) {
        let text = text.as_ref();
        let s = NSString::new(text);

        self.objc.with_mut(|obj| unsafe {
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setStringValue:&*s];
            #[cfg(all(feature = "uikit", not(feature = "appkit")))]
            let _: () = msg_send![obj, setText:&*s];
        });
    }

    /// Sets the attributed string to be the attributed string value on this label.
    pub fn set_attributed_text(&self, text: AttributedString) {
        self.objc.with_mut(|obj| unsafe {
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setAttributedStringValue:&*text];
            #[cfg(all(feature = "uikit", not(feature = "appkit")))]
            let _: () = msg_send![obj, setAttributedText:&*text];
        });
    }

    /// Retrieve the text currently held in the label.
    #[cfg(feature = "appkit")]
    pub fn get_text(&self) -> String {
        self.objc
            .get(|obj| unsafe { NSString::retain(msg_send![obj, stringValue]).to_string() })
    }
    #[cfg(all(feature = "uikit", not(feature = "appkit")))]
    pub fn get_text(&self) -> String {
        self.objc.get(|obj| {
            let val: id = unsafe { msg_send![obj, text] };
            // Through trial and error, this seems to return a null pointer when there's no
            // text.
            if val.is_null() {
                String::new()
            } else {
                NSString::retain(val).to_string()
            }
        })
    }

    /// Sets the text alignment for this label.
    pub fn set_text_alignment(&self, alignment: TextAlign) {
        self.objc.with_mut(|obj| unsafe {
            let alignment: NSInteger = alignment.into();
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setAlignment: alignment];
            #[cfg(all(feature = "uikit", not(feature = "appkit")))]
            let _: () = msg_send![obj, setTextAlignment: alignment];
        });
    }

    /// Sets the font for this label.
    pub fn set_font<F: AsRef<Font>>(&self, font: F) {
        // This clone is here to ensure there's no oddities with retain counts on the underlying
        // font object - it seems like it can be optimized away otherwise.
        let font = font.as_ref().clone();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setFont:&*font];
        });
    }

    /// Set whether this is hidden or not.
    pub fn set_hidden(&self, hidden: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setHidden:match hidden {
                true => YES,
                false => NO
            }];
        });
    }

    /// Sets the maximum number of lines.
    pub fn set_max_number_of_lines(&self, num: NSInteger) {
        self.objc.with_mut(|obj| unsafe {
            #[cfg(feature = "appkit")]
            let _: () = msg_send![obj, setMaximumNumberOfLines: num];
            #[cfg(feature = "uikit")]
            let _: () = msg_send![obj, setNumberOfLines: num];
        });
    }

    /// Set the line break mode for this label.
    pub fn set_line_break_mode(&self, mode: LineBreakMode) {
        #[cfg(feature = "appkit")]
        self.objc.with_mut(|obj| unsafe {
            let cell: id = msg_send![obj, cell];
            let mode = mode as NSUInteger;
            let _: () = msg_send![cell, setTruncatesLastVisibleLine: YES];
            let _: () = msg_send![cell, setLineBreakMode: mode];
        });
    }
}

impl<T> ObjcAccess for Label<T> {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl<T> Layout for Label<T> {}

impl<T> Drop for Label<T> {
    /// A bit of extra cleanup for delegate callback pointers. If the originating `Label` is being
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
fn test_label() {
    let label = Label::new();
    let text = label.get_text();
    assert!(text.is_empty());
    label.set_background_color(Color::SystemOrange);
    label.set_text_color(Color::SystemRed);
    label.set_text_alignment(TextAlign::Right);
    label.set_text("foobar");
    let text = label.get_text();
    assert_eq!(text, "foobar".to_string());
    label.set_font(Font::system(10.0));
    label.set_attributed_text(AttributedString::new("foobar"));
}
