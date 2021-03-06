//! Wraps `NSTextField` and `UILabel` across platforms, explicitly as a Label.
//! In AppKit, `NSTextField` does double duty, and for clarity we just double
//! the implementation.
//!
//! Labels implement Autolayout, which enable you to specify how things should appear on the screen.
//! 
//! ```rust,no_run
//! use cacao::color::rgb;
//! use cacao::layout::{Layout, LayoutConstraint};
//! use cacao::view::Label;
//! use cacao::window::{Window, WindowDelegate};
//!
//! #[derive(Default)]
//! struct AppWindow {
//!     content: Label,
//!     label: Label,
//!     window: Window
//! }
//! 
//! impl WindowDelegate for AppWindow {
//!     fn did_load(&mut self, window: Window) {
//!         window.set_minimum_content_size(300., 300.);
//!         self.window = window;
//!
//!         self.label.set_background_color(rgb(224, 82, 99));
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

use objc_id::ShareId;
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSArray, NSInteger, NSUInteger, NSString};
use crate::color::Color;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::text::{Font, TextAlign, LineBreakMode};
use crate::utils::properties::ObjcProperty;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::{register_view_class, register_view_class_with_delegate};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::{register_view_class, register_view_class_with_delegate};

mod traits;
pub use traits::LabelDelegate;

pub(crate) static LABEL_DELEGATE_PTR: &str = "rstLabelDelegatePtr";

/// A helper method for instantiating view classes and applying default settings to them.
fn allocate_view(registration_fn: fn() -> *const Class) -> id { 
    unsafe {
        #[cfg(target_os = "macos")]
        let view: id = {
            // This sucks, but for now, sure.
            let blank = NSString::no_copy("");
            msg_send![registration_fn(), labelWithString:&*blank]
        };

        #[cfg(target_os = "ios")]
        let view: id = msg_send![registration_fn(), new];

        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];

        #[cfg(target_os = "macos")]
        let _: () = msg_send![view, setWantsLayer:YES];

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
/// ```rust,no_run
/// use cacao::color::rgb;
/// use cacao::layout::{Layout, LayoutConstraint};
/// use cacao::view::Label;
/// use cacao::window::{Window, WindowDelegate};
///
/// #[derive(Default)]
/// struct AppWindow {
///     content: Label,
///     label: Label,
///     window: Window
/// }
/// 
/// impl WindowDelegate for AppWindow {
///     fn did_load(&mut self, window: Window) {
///         window.set_minimum_content_size(300., 300.);
///         self.window = window;
///
///         self.label.set_background_color(rgb(224, 82, 99));
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

impl Default for Label {
    fn default() -> Self {
        Label::new()
    }
}

impl Label {
    /// Returns a default `Label`, suitable for 
    pub fn new() -> Self {
        let view = allocate_view(register_view_class);

        Label {
            delegate: None,
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
}

impl<T> Label<T> where T: LabelDelegate + 'static {
    /// Initializes a new Label with a given `LabelDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> Label<T> {
        let delegate = Box::new(delegate);
        
        let label = allocate_view(register_view_class_with_delegate::<T>);
        unsafe {
            //let view: id = msg_send![register_view_class_with_delegate::<T>(), new];
            //let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            let ptr: *const T = &*delegate;
            (&mut *label).set_ivar(LABEL_DELEGATE_PTR, ptr as usize);
        };

        let mut label = Label {
            delegate: None,
            top: LayoutAnchorY::top(label),
            left: LayoutAnchorX::left(label),
            leading: LayoutAnchorX::leading(label),
            right: LayoutAnchorX::right(label),
            trailing: LayoutAnchorX::trailing(label),
            bottom: LayoutAnchorY::bottom(label),
            width: LayoutAnchorDimension::width(label),
            height: LayoutAnchorDimension::height(label),
            center_x: LayoutAnchorX::center(label),
            center_y: LayoutAnchorY::center(label),
            objc: ObjcProperty::retain(label),
        };

        //(&mut delegate).did_load(label.clone_as_handle()); 
        label.delegate = Some(delegate);
        label
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
            top: self.top.clone(),
            leading: self.leading.clone(),
            left: self.left.clone(),
            trailing: self.trailing.clone(),
            right: self.right.clone(),
            bottom: self.bottom.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            center_x: self.center_x.clone(),
            center_y: self.center_y.clone(),
            objc: self.objc.clone()
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        // @TODO: This is wrong.
        // Needs to set ivar and such, akin to View. 
        self.objc.with_mut(|obj| unsafe {
            let color = color.as_ref().cg_color();
            let layer: id = msg_send![obj, layer];
            let _: () = msg_send![layer, setBackgroundColor:color];
        });
    }

    /// Call this to set the color of the text.
    pub fn set_text_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTextColor:color];
        });
    }

    /// Call this to set the text for the label.
    pub fn set_text<S: AsRef<str>>(&self, text: S) {
        let text = text.as_ref();
        let s = NSString::new(text);

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setStringValue:&*s];
        });
    }

    /// Retrieve the text currently held in the label.
    pub fn get_text(&self) -> String {
        self.objc.get(|obj| unsafe {
            NSString::retain(msg_send![obj, stringValue]).to_string()
        })
    }

    /// Sets the text alignment for this label.
    pub fn set_text_alignment(&self, alignment: TextAlign) {
        self.objc.with_mut(|obj| unsafe {
            let alignment: NSInteger = alignment.into();
            let _: () = msg_send![obj, setAlignment:alignment];
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
            let _: () = msg_send![obj, setMaximumNumberOfLines:num];
        });
    }

    /// Set the line break mode for this label.
    pub fn set_line_break_mode(&self, mode: LineBreakMode) {
        #[cfg(target_os = "macos")]
        self.objc.with_mut(|obj| unsafe {
            let cell: id = msg_send![obj, cell];
            let mode = mode as NSUInteger;
            let _: () = msg_send![cell, setTruncatesLastVisibleLine:YES];
            let _: () = msg_send![cell, setLineBreakMode:mode];
        });
    }
}

impl<T> Layout for Label<T> {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_node<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

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
