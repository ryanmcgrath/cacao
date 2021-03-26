//! Wraps `NSView` and `UIView` across platforms.
//!
//! This implementation errs towards the `UIView` side of things, and mostly acts as a wrapper to
//! bring `NSView` to the modern era. It does this by flipping the coordinate system to be what
//! people expect in 2020, and layer-backing all views by default.
//!
//! Views implement Autolayout, which enable you to specify how things should appear on the screen.
//! 
//! ```rust,no_run
//! use cacao::color::Color;
//! use cacao::layout::{Layout, LayoutConstraint};
//! use cacao::view::View;
//! use cacao::window::{Window, WindowDelegate};
//!
//! #[derive(Default)]
//! struct AppWindow {
//!     content: View,
//!     red: View,
//!     window: Window
//! }
//! 
//! impl WindowDelegate for AppWindow {
//!     fn did_load(&mut self, window: Window) {
//!         window.set_minimum_content_size(300., 300.);
//!         self.window = window;
//!
//!         self.red.set_background_color(Color::SystemRed);
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
use objc::{msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSArray, NSString};
use crate::color::Color;
use crate::layer::Layer;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::pasteboard::PasteboardType;
use crate::utils::properties::ObjcProperty;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::{register_view_class, register_view_class_with_delegate};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::{register_view_class, register_view_class_with_delegate};

mod controller;
pub use controller::ViewController;

mod splitviewcontroller;
pub use splitviewcontroller::SplitViewController;

mod traits;
pub use traits::ViewDelegate;

pub(crate) static BACKGROUND_COLOR: &str = "alchemyBackgroundColor";
pub(crate) static VIEW_DELEGATE_PTR: &str = "rstViewDelegatePtr";

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Debug)]
pub struct View<T = ()> {
    /// An internal flag for whether an instance of a View<T> is a handle. Typically, there's only
    /// one instance that should have this set to `false` - if that one drops, we need to know to
    /// do some extra cleanup.
    pub is_handle: bool,

    /// A pointer to the Objective-C runtime view controller.
    pub objc: ObjcProperty,

    /// References the underlying layer. This is consistent across macOS, iOS and tvOS - on macOS
    /// we explicitly opt in to layer backed views.
    pub layer: Layer,

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

impl Default for View {
    /// Returns a stock view, for... well, whatever you want.
    fn default() -> Self {
        View::new()
    }
}

impl View {
    /// An internal initializer method for very common things that we need to do, regardless of
    /// what type the end user is creating.
    ///
    /// This handles grabbing autolayout anchor pointers, as well as things related to layering and
    /// so on. It returns a generic `View<T>`, which the caller can then customize as needed.
    pub(crate) fn init<T>(view: id) -> View<T> {
        unsafe {
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];

            #[cfg(target_os = "macos")]
            let _: () = msg_send![view, setWantsLayer:YES];
        }

        View {
            is_handle: false,
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
            
            layer: Layer::wrap(unsafe {
                msg_send![view, layer]
            }),

            objc: ObjcProperty::retain(view),
        }
    }

    /// Returns a default `View`, suitable for customizing and displaying.
    pub fn new() -> Self {
        View::init(unsafe {
            msg_send![register_view_class(), new]
        })
    }
}

impl<T> View<T> where T: ViewDelegate + 'static {
    /// Initializes a new View with a given `ViewDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> View<T> {
        let class = register_view_class_with_delegate(&delegate);
        let mut delegate = Box::new(delegate);
        
        let view = unsafe {
            let view: id = msg_send![class, new];
            let ptr = Box::into_raw(delegate);
            (&mut *view).set_ivar(VIEW_DELEGATE_PTR, ptr as usize);
            delegate = Box::from_raw(ptr);
            view
        };

        let mut view = View::init(view);
        (&mut delegate).did_load(view.clone_as_handle()); 
        view.delegate = Some(delegate);
        view
    }
}

impl<T> View<T> {
    /// An internal method that returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `View` is the only true holder of those.
    pub(crate) fn clone_as_handle(&self) -> View {
        View {
            delegate: None,
            is_handle: true,
            layer: self.layer.clone(),
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
        let color: id = color.as_ref().into();

        self.objc.with_mut(|obj| unsafe {
            (&mut *obj).set_ivar(BACKGROUND_COLOR, color);
        });
    }

    /// Register this view for drag and drop operations.
    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        let types: NSArray = types.into_iter().map(|t| {
            let x: NSString = (*t).into();
            x.into()
        }).collect::<Vec<id>>().into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, registerForDraggedTypes:&*types];
        });
    }
}

impl<T> Layout for View<T> {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }
}

impl<T> Drop for View<T> {
    /// If the instance being dropped is _not_ a handle, then we want to go ahead and explicitly
    /// remove it from any super views.
    ///
    /// Why do we do this? It's to try and match Rust's ownership model/semantics. If a Rust value
    /// drops, it (theoretically) makes sense that the View would drop... and not be visible, etc.
    ///
    /// If you're venturing into unsafe code for the sake of custom behavior via the Objective-C
    /// runtime, you can consider flagging your instance as a handle - it will avoid the drop logic here.
    fn drop(&mut self) {
        if !self.is_handle {
            self.remove_from_superview();
        }
    }
}
