//! Wraps `NSView` and `UIView` across platforms.
//!
//! This implementation errs towards the `UIView` side of things, and mostly acts as a wrapper to
//! bring `NSView` to the modern era. It does this by flipping the coordinate system to be what
//! people expect in 2020, and layer-backing all views by default.
//!
//! Views implement Autolayout, which enable you to specify how things should appear on the screen.
//! 
//! ```rust,no_run
//! use cacao::color::rgb;
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
//!         self.red.set_background_color(rgb(224, 82, 99));
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

use std::rc::Rc;
use std::cell::RefCell;

use objc_id::{Id, ShareId};
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, NSArray, NSString};
use crate::color::Color;
use crate::layer::Layer;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};
use crate::pasteboard::PasteboardType;
use crate::view::ViewDelegate;
use crate::utils::properties::ObjcProperty;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::{register_listview_row_class, register_listview_row_class_with_delegate};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::{register_listview_row_view_class, register_listview_row_class_with_delegate};

pub(crate) static BACKGROUND_COLOR: &str = "alchemyBackgroundColor";
pub(crate) static LISTVIEW_ROW_DELEGATE_PTR: &str = "rstListViewRowDelegatePtr";

/// A helper method for instantiating view classes and applying default settings to them.
fn allocate_view(registration_fn: fn() -> *const Class) -> id { 
    unsafe {
        let view: id = msg_send![registration_fn(), new];
        let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];

        #[cfg(target_os = "macos")]
        let _: () = msg_send![view, setWantsLayer:YES];

        view 
    }
}

/// A clone-able handler to a `ViewController` reference in the Objective C runtime. We use this
/// instead of a stock `View` for easier recordkeeping, since it'll need to hold the `View` on that
/// side anyway.
#[derive(Debug)]
pub struct ListViewRow<T = ()> {
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

impl<T> ListViewRow<T> where T: ViewDelegate + 'static {
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
            //let view: id = msg_send![register_view_class_with_delegate::<T>(), new];
            //let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            let ptr: *const T = &*delegate;
            (&mut *view).set_ivar(LISTVIEW_ROW_DELEGATE_PTR, ptr as usize);
        };

        let mut view = ListViewRow {
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
            layer: Layer::new(),
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
            (&mut *obj).set_ivar(BACKGROUND_COLOR, color);
        });
    }

    /// Register this view for drag and drop operations.
    pub fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        let types: NSArray = types.into_iter().map(|t| {
            // This clone probably doesn't need to be here, but it should also be cheap as
            // this is just an enum... and this is not an oft called method.
            let x: NSString = (*t).into();
            x.into()
        }).collect::<Vec<id>>().into();

        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, registerForDraggedTypes:&*types];
        });
    }
}

impl<T> Layout for ListViewRow<T> {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }
}

impl<T> Drop for ListViewRow<T> {
    fn drop(&mut self) {
    }
}
