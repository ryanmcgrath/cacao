//! Wraps `NSView` and `UIView` across platforms.
//!
//! This implementation errs towards the `UIView` side of things, and mostly acts as a wrapper to
//! bring `NSView` to the modern era. It does this by flipping the coordinate system to be what
//! people expect in 2020, and layer-backing all views by default.
//!
//! Views implement Autolayout, which enable you to specify how things should appear on the screen.
//!
//! ```rust
//! use cacao::color::Color;
//! use cacao::layout::{Layout, LayoutConstraint};
//! use cacao::view::View;
//! use cacao::appkit::window::{Window, WindowDelegate};
//!
//! #[derive(Default)]
//! struct AppWindow {
//!     content: View,
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

use std::cell::Ref;

use objc::rc::{Id, Owned};
use objc::runtime::{Class, Object};
use objc::{msg_send, msg_send_id, sel};

use crate::color::Color;
use crate::foundation::{id, nil, NSArray, NSInteger, NSString, NO, YES};
use crate::layer::Layer;
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;

#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY, SafeAreaLayoutGuide};

#[cfg(feature = "appkit")]
use crate::pasteboard::PasteboardType;

#[cfg(all(feature = "appkit", target_os = "macos"))]
mod animator;

#[cfg(all(feature = "appkit", target_os = "macos"))]
pub use animator::ViewAnimatorProxy;

#[cfg_attr(feature = "appkit", path = "appkit.rs")]
#[cfg_attr(feature = "uikit", path = "uikit.rs")]
mod native_interface;

mod controller;
pub use controller::ViewController;

#[cfg(feature = "appkit")]
mod splitviewcontroller;

#[cfg(feature = "appkit")]
pub use splitviewcontroller::SplitViewController;

#[cfg(feature = "appkit")]
mod popover;
#[cfg(feature = "appkit")]
pub use popover::*;
mod traits;
pub use traits::ViewDelegate;

pub(crate) static BACKGROUND_COLOR: &str = "cacaoBackgroundColor";
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

    /// An object that supports limited animations. Can be cloned into animation closures.
    ///
    /// This is currently only supported on macOS with the `appkit` feature.
    #[cfg(all(feature = "appkit", target_os = "macos"))]
    pub animator: ViewAnimatorProxy,

    /// References the underlying layer. This is consistent across AppKit & UIKit - in AppKit
    /// we explicitly opt in to layer backed views.
    pub layer: Layer,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Box<T>>,

    /// A property containing safe layout guides.
    #[cfg(feature = "autolayout")]
    pub safe_layout_guide: SafeAreaLayoutGuide,

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
            #[cfg(feature = "autolayout")]
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints: NO];

            #[cfg(feature = "appkit")]
            let _: () = msg_send![view, setWantsLayer: YES];
        }

        View {
            is_handle: false,
            delegate: None,

            #[cfg(feature = "autolayout")]
            safe_layout_guide: SafeAreaLayoutGuide::new(view),

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

            layer: Layer::from_id(unsafe { msg_send_id![view, layer] }),

            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: ViewAnimatorProxy::new(view),
            objc: ObjcProperty::retain(view)
        }
    }

    /// Returns a default `View`, suitable for customizing and displaying.
    pub fn new() -> Self {
        View::init(unsafe { msg_send![native_interface::register_view_class(), new] })
    }
}

impl<T> View<T>
where
    T: ViewDelegate + 'static
{
    /// Initializes a new View with a given `ViewDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> View<T> {
        let class = native_interface::register_view_class_with_delegate(&delegate);
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
    /// Returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `View` is the only true holder of those.
    pub fn clone_as_handle(&self) -> View {
        View {
            delegate: None,
            is_handle: true,
            layer: self.layer.clone(),
            objc: self.objc.clone(),

            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: self.animator.clone(),

            #[cfg(feature = "autolayout")]
            safe_layout_guide: self.safe_layout_guide.clone(),

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

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();

        #[cfg(feature = "appkit")]
        self.objc.with_mut(|obj| unsafe {
            // TODO: Fix this unnecessary retain!
            (&mut *obj).set_ivar::<id>(BACKGROUND_COLOR, msg_send![color, retain]);
        });

        #[cfg(feature = "uikit")]
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*obj, setBackgroundColor: color];
        });
    }

    /// A setter for `[NSView layerContentsRedrawPolicy]`.
    ///
    /// For more information, consult:
    ///
    /// [https://developer.apple.com/documentation/appkit/nsview/1483514-layercontentsredrawpolicy?language=objc](https://developer.apple.com/documentation/appkit/nsview/1483514-layercontentsredrawpolicy?language=objc)
    #[cfg(feature = "appkit")]
    pub fn set_contents_redraw_policy(&self, policy: LayerContentsRedrawPolicy) {
        self.objc.with_mut(|obj| unsafe {
            let policy = policy.to_nsinteger();
            let _: () = msg_send![obj, setLayerContentsRedrawPolicy:policy];
        });
    }

    /// Mark all child layers as being able to be drawn into a single CALayer. This can be useful
    /// for moments when you need to lower your total layer count, which can impair composition
    /// time.
    #[cfg(feature = "appkit")]
    pub fn set_can_draw_subviews_into_layer(&self, can: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*obj, setCanDrawSubviewsIntoLayer:match can {
                true => YES,
                false => NO
            }];
        });
    }
}

impl<T> ObjcAccess for View<T> {
    fn with_backing_obj_mut(&self, handler: &dyn Fn(id)) {
        self.objc.with_mut(handler);
    }

    fn get_backing_obj(&self) -> Ref<'_, Id<Object, Owned>> {
        self.objc.get_ref()
    }
}

impl<T> Layout for View<T> {}

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

/// Variants describing what an underlying NSView layer redraw policy should be.
#[cfg(feature = "appkit")]
#[derive(Debug)]
pub enum LayerContentsRedrawPolicy {
    Never,
    OnSetNeedsDisplay,
    DuringViewResize,
    BeforeViewResize,
    Crossfade
}

#[cfg(feature = "appkit")]
impl LayerContentsRedrawPolicy {
    /// Mapping required for ObjC setters.
    pub fn to_nsinteger(&self) -> NSInteger {
        match self {
            Self::Never => 0,
            Self::OnSetNeedsDisplay => 1,
            Self::DuringViewResize => 2,
            Self::BeforeViewResize => 3,
            Self::Crossfade => 4
        }
    }
}

#[test]
fn test_view() {
    let view = View::new();
    let _clone = view.clone_as_handle();
    view.set_background_color(Color::SystemGreen);
}
