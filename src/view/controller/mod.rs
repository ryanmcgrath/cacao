use crate::id_shim::ShareId;
use objc::runtime::Object;
use objc::{msg_send, sel};

use crate::foundation::id;
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::Controller;
use crate::view::{View, ViewDelegate, VIEW_DELEGATE_PTR};

#[cfg_attr(feature = "appkit", path = "appkit.rs")]
#[cfg_attr(feature = "uikit", path = "uikit.rs")]
mod native_interface;

/// A `ViewController` is a wrapper around `NSViewController` in AppKit, and `UIViewController` in
/// UIKit
///
/// This type is interchangeable with a standard `View<T>`, in that using this simply forwards
/// standard view controller lifecycle methods onto your `ViewDelegate`. You would use this if you
/// need to be notified of _when_ something is going to be used (e.g, for lifecycle event-based
/// cleanup routines, or something).
///
/// ## Example
/// ```rust,no_run
/// use cacao::view::ViewDelegate;
///
/// struct ContentViewDelegate;
///
/// impl ViewDelegate for ContentViewDelegate {
///     const NAME: &'static str = "ContentViewDelegate";
///
///     fn will_appear(&self, animated: bool) {
///         println!("This controller is about to appear!");
///     }
/// }
/// ```
#[derive(Debug)]
pub struct ViewController<T> {
    /// The underlying Objective-C pointer.
    pub objc: ShareId<Object>,

    /// The underlying View that we manage.
    pub view: View<T>
}

impl<T> ViewController<T>
where
    T: ViewDelegate + 'static
{
    /// Creates and returns a new `ViewController` with the provided `delegate`.
    pub fn new(delegate: T) -> Self {
        let class = native_interface::register_view_controller_class::<T>(&delegate);
        let view = View::with(delegate);

        let objc = unsafe {
            let vc: id = msg_send![class, new];

            if let Some(delegate) = &view.delegate {
                let ptr: *const T = &**delegate;
                (&mut *vc).set_ivar(VIEW_DELEGATE_PTR, ptr as usize);
            }

            view.with_backing_obj_mut(|backing_node| {
                let _: () = msg_send![vc, setView: backing_node];
            });

            ShareId::from_ptr(vc)
        };

        ViewController { objc, view }
    }
}

impl<T> Controller for ViewController<T> {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }
}
