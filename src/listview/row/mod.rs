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
//!         self.red.set_background_color(Color::rgb(224, 82, 99));
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

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use objc::rc::{Id, Owned, Shared};
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel};

use crate::color::Color;
use crate::foundation::{id, nil, NSArray, NSString, NO, YES};
use crate::layer::Layer;
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;
#[cfg(all(feature = "appkit", target_os = "macos"))]
use crate::view::{ViewAnimatorProxy, ViewDelegate};

#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY, SafeAreaLayoutGuide};

#[cfg(feature = "appkit")]
mod appkit;

#[cfg(feature = "appkit")]
use appkit::{register_listview_row_class, register_listview_row_class_with_delegate};

//#[cfg(feature = "uikit")]
//mod ios;

//#[cfg(feature = "uikit")]
//use ios::{register_listview_row_view_class, register_listview_row_class_with_delegate};

pub(crate) static BACKGROUND_COLOR: &str = "cacaoBackgroundColor";
pub(crate) static LISTVIEW_ROW_DELEGATE_PTR: &str = "cacaoListViewRowDelegatePtr";

/// A helper method for instantiating view classes and applying default settings to them.
fn allocate_view(registration_fn: fn() -> &'static Class) -> id {
    unsafe {
        let view: id = msg_send![registration_fn(), new];

        #[cfg(feature = "autolayout")]
        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints: NO];

        #[cfg(feature = "appkit")]
        let _: () = msg_send![view, setWantsLayer: YES];

        view
    }
}

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Debug)]
pub struct ListViewRow<T = ()> {
    /// An object that supports limited animations. Can be cloned into animation closures.
    #[cfg(all(feature = "appkit", target_os = "macos"))]
    pub animator: ViewAnimatorProxy,

    /// A pointer to the Objective-C runtime view controller.
    pub objc: ObjcProperty,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Box<T>>,

    /// A safe layout guide property.
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

impl Default for ListViewRow {
    fn default() -> Self {
        ListViewRow::new()
    }
}

impl ListViewRow {
    /// Returns a default `View`, suitable for
    pub fn new() -> Self {
        let view = allocate_view(register_listview_row_class);

        ListViewRow {
            delegate: None,
            objc: ObjcProperty::retain(view),
            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: ViewAnimatorProxy::new(view),

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
            center_y: LayoutAnchorY::center(view)
        }
    }
}

impl<T> ListViewRow<T>
where
    T: ViewDelegate + 'static
{
    /// When we're able to retrieve a reusable view cell from the backing table view, we can check
    /// for the pointer and attempt to reconstruct the ListViewRow<T> that corresponds to this.
    ///
    /// We can be reasonably sure that the pointer for the delegate is accurate, as:
    ///
    /// - A `ListViewRow` is explicitly not clone-able
    /// - It owns the Delegate on creation
    /// - It takes ownership of the returned row in row_for_item
    /// - When it takes ownership, it "forgets" the pointer - and the `dealloc` method on the
    /// backing view cell will clean it up whenever it's dropped.
    pub(crate) fn from_cached(view: id) -> ListViewRow<T> {
        // @TODO: Make this better.
        let delegate = unsafe {
            let ptr: usize = *(&*view).get_ivar(LISTVIEW_ROW_DELEGATE_PTR);
            let obj = ptr as *mut T;
            Box::from_raw(obj)
            //&*obj
        };

        let view = ListViewRow {
            delegate: Some(delegate),
            objc: ObjcProperty::retain(view),
            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: ViewAnimatorProxy::new(view),

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
            center_y: LayoutAnchorY::center(view)
        };

        view
    }

    pub fn with(delegate: T) -> ListViewRow<T> {
        let delegate = Box::new(delegate);
        Self::with_boxed(delegate)
    }

    /// Initializes a new View with a given `ViewDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with_boxed(mut delegate: Box<T>) -> ListViewRow<T> {
        let view = allocate_view(register_listview_row_class_with_delegate::<T>);
        unsafe {
            let ptr: *const T = &*delegate;
            (&mut *view).set_ivar(LISTVIEW_ROW_DELEGATE_PTR, ptr as usize);
        };

        let mut view = ListViewRow {
            delegate: None,
            objc: ObjcProperty::retain(view),
            #[cfg(all(feature = "appkit", target_os = "macos"))]
            animator: ViewAnimatorProxy::new(view),

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
            center_y: LayoutAnchorY::center(view)
        };

        (&mut delegate).did_load(view.clone_as_handle());
        view.delegate = Some(delegate);
        view
    }

    pub fn into_row(mut self) -> ListViewRow {
        // "forget" delegate, then move into standard ListViewRow
        // to ease return type
        let delegate = self.delegate.take();
        if let Some(d) = delegate {
            let _ = Box::into_raw(d);
        }

        ListViewRow {
            delegate: None,
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
}

impl<T> ListViewRow<T> {
    /// An internal method that returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `View` is the only true holder of those.
    pub(crate) fn clone_as_handle(&self) -> crate::view::View {
        crate::view::View {
            delegate: None,
            is_handle: true,
            layer: Layer::new(), // @TODO: Fix & return cloned true layer for this row.
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

    /// Sets the identifier, which enables cells to be reused and dequeued properly.
    pub fn set_identifier(&self, identifier: &'static str) {
        let identifier = NSString::new(identifier);

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setIdentifier:&*identifier];
        });
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();

        self.objc.with_mut(|obj| unsafe {
            // TODO: Fix this unnecessary retain!
            (&mut *obj).set_ivar::<id>(BACKGROUND_COLOR, msg_send![color, retain]);
        });
    }
}

impl<T> ObjcAccess for ListViewRow<T> {
    fn with_backing_obj_mut(&self, handler: &dyn Fn(id)) {
        self.objc.with_mut(handler);
    }

    fn get_backing_obj(&self) -> Ref<'_, Id<Object, Owned>> {
        self.objc.get_ref()
    }
}

impl<T> Layout for ListViewRow<T> {}

impl<T> Drop for ListViewRow<T> {
    fn drop(&mut self) {}
}
