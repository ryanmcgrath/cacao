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

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::{register_view_class, register_view_class_with_delegate};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::{register_view_class, register_view_class_with_delegate};

//mod controller;
//pub use controller::LabelController;

mod traits;
pub use traits::LabelDelegate;

pub(crate) static LABEL_DELEGATE_PTR: &str = "rstLabelDelegatePtr";

/// A helper method for instantiating view classes and applying default settings to them.
fn allocate_view(registration_fn: fn() -> *const Class) -> id { 
    unsafe {
        #[cfg(target_os = "macos")]
        let view: id = {
            // This sucks, but for now, sure.
            let blank = NSString::new("");
            msg_send![registration_fn(), labelWithString:blank.into_inner()]
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
#[derive(Debug)]
pub struct Label<T = ()> {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ShareId<Object>,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Box<T>>,

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
            top: LayoutAnchorY::new(unsafe { msg_send![label, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![label, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![label, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![label, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![label, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![label, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![label, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![label, centerYAnchor] }),
            objc: unsafe { ShareId::from_ptr(label) },
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
            trailing: self.trailing.clone(),
            bottom: self.bottom.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            center_x: self.center_x.clone(),
            center_y: self.center_y.clone(),
            objc: self.objc.clone()
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color(&self, color: Color) {
        let bg = color.into_platform_specific_color();
        
        unsafe {
            let cg: id = msg_send![bg, CGColor];
            let layer: id = msg_send![&*self.objc, layer];
            let _: () = msg_send![layer, setBackgroundColor:cg];
        }
    }

    /// Call this to set the color of the text.
    pub fn set_text_color(&self, color: Color) {
        let color = color.into_platform_specific_color();

        unsafe {
            let _: () = msg_send![&*self.objc, setTextColor:color];
        }
    }

    /// Call this to set the text for the label.
    pub fn set_text(&self, text: &str) {
        let s = NSString::new(text);

        unsafe {
            let _: () = msg_send![&*self.objc, setStringValue:s.into_inner()];
        }
    }

    /// Retrieve the text currently held in the label.
    pub fn text(&self) -> String {
        let s = NSString::wrap(unsafe {
            msg_send![&*self.objc, stringValue]
        });

        s.to_str().to_string()
    }

    pub fn set_text_alignment(&self, alignment: TextAlign) {
        unsafe {
            let alignment: NSInteger = alignment.into();
            let _: () = msg_send![&*self.objc, setAlignment:alignment];
        }
    }

    pub fn set_font(&self, font: &Font) {
        unsafe {
            let _: () = msg_send![&*self.objc, setFont:&*font.objc];
        }
    }

    pub fn set_line_break_mode(&self, mode: LineBreakMode) {
        #[cfg(target_os = "macos")]
        unsafe {
            let cell: id = msg_send![&*self.objc, cell];
            let mode = mode as NSUInteger;
            let _: () = msg_send![cell, setLineBreakMode:mode];
        }
    }
}

impl<T> Layout for Label<T> {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, view: &V) {
        let backing_node = view.get_backing_node();

        unsafe {
            let _: () = msg_send![&*self.objc, addSubview:backing_node];
        }
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
        if self.delegate.is_some() {
            unsafe {
                let superview: id = msg_send![&*self.objc, superview];
                if superview != nil {
                    let _: () = msg_send![&*self.objc, removeFromSuperview];
                }
            }
        }
    }
}
