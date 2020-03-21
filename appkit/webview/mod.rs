//! Implements a WebView, which wraps a number of different classes/delegates/controllers into one
//! useful interface. This encompasses...
//!
//! - `WKWebView`
//! - `WKUIDelegate`
//! - `WKScriptMessageHandler`

use std::rc::Rc;
use std::cell::RefCell;

use objc_id::ShareId;
use objc::runtime::{Class, Object};
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, YES, NO, CGRect, NSString};
use crate::geometry::Rect;
use crate::layout::{Layout, LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};

pub mod actions;
pub mod enums;

pub(crate) mod class;
use class::{register_webview_class, register_webview_class_with_delegate};
//pub(crate) mod process_pool;

pub mod traits;
pub use traits::WebViewDelegate;

pub mod config;
pub use config::WebViewConfig;

pub(crate) static WEBVIEW_DELEGATE_PTR: &str = "rstWebViewDelegatePtr";

fn allocate_webview(
    config: WebViewConfig,
    delegate: Option<&Object>
) -> id {
    unsafe {
        let configuration = config.into_inner();
        
        if let Some(delegate) = objc_delegate {
            // Technically private!
            #[cfg(feature = "webview-downloading")]
            let process_pool: id = msg_send![configuration, processPool]; 
            #[cfg(feature = "webview-downloading")]
            let _: () = msg_send![process_pool, _setDownloadDelegate:*delegate];

            // add handlers
            for
        }

        let zero: CGRect = Rect::zero().into();
        let webview_alloc: id = msg_send![register_webview_class(), alloc];
        let webview: id = msg_send![webview_alloc, initWithFrame:zero configuration:configuration];

        let _: () = msg_send![webview, setWantsLayer:YES];
        let _: () = msg_send![webview, setTranslatesAutoresizingMaskIntoConstraints:NO];
        let _: () = msg_send![webview, setNavigationDelegate:webview];
        let _: () = msg_send![webview, setUIDelegate:webview];

        webview
    }
}

pub struct WebView<T = ()> {
    /// A pointer to the Objective-C runtime view controller.
    pub objc: ShareId<Object>,

    /// We need to store the underlying delegate separately from the `WKWebView` - this is a where
    /// we do so.
    pub objc_delegate: Option<ShareId<Object>>,

    /// An internal callback pointer that we use in delegate loopbacks. Default implementations
    /// don't require this.
    pub(crate) internal_callback_ptr: Option<*const RefCell<T>>,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Rc<RefCell<T>>>,

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

impl Default for WebView {
    fn default() -> Self {
        WebView::new(WebViewConfig::default())
    }
}

impl WebView {
    pub fn new(config: WebViewConfig) -> Self {
        let view = allocate_webview(register_webview_class, config, None);

        WebView {
            internal_callback_ptr: None,
            delegate: None,
            objc_delegate: None,
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

impl<T> WebView<T> where T: WebViewDelegate + 'static {
    /// Initializes a new WebView with a given `WebViewDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(config: WebViewConfig, delegate: T) -> WebView<T> {
        let delegate = Rc::new(RefCell::new(delegate));
        
        let internal_callback_ptr = {
            let cloned = Rc::clone(&delegate);
            Rc::into_raw(cloned)
        };

        let objc_delegate = unsafe {
            let objc_delegate: id = msg_send![register_webview_delegate_class::<T>, new];
            (&mut *objc_delegate).set_ivar(WEBVIEW_DELEGATE_PTR, internal_callback_ptr as usize);
            ShareId::from_ptr(objc_delegate)
        };

        let view = allocate_webview(config, Some(&objc_delegate));

        let mut view = WebView {
            internal_callback_ptr: Some(internal_callback_ptr),
            delegate: None,
            objc_delegate: Some(objc_delegate),
            top: LayoutAnchorY::new(unsafe { msg_send![view, topAnchor] }),
            leading: LayoutAnchorX::new(unsafe { msg_send![view, leadingAnchor] }),
            trailing: LayoutAnchorX::new(unsafe { msg_send![view, trailingAnchor] }),
            bottom: LayoutAnchorY::new(unsafe { msg_send![view, bottomAnchor] }),
            width: LayoutAnchorDimension::new(unsafe { msg_send![view, widthAnchor] }),
            height: LayoutAnchorDimension::new(unsafe { msg_send![view, heightAnchor] }),
            center_x: LayoutAnchorX::new(unsafe { msg_send![view, centerXAnchor] }),
            center_y: LayoutAnchorY::new(unsafe { msg_send![view, centerYAnchor] }),
            objc: unsafe { ShareId::from_ptr(view) },
        };

        {
            let mut delegate = delegate.borrow_mut();
            (*delegate).did_load(view.clone_as_handle()); 
        }

        view.delegate = Some(delegate);
        view
    }
}

impl<T> WebView<T> {
    /// An internal method that returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `View` is the only true holder of those.
    pub(crate) fn clone_as_handle(&self) -> WebView {
        WebView {
            internal_callback_ptr: None,
            delegate: None,
            top: self.top.clone(),
            leading: self.leading.clone(),
            trailing: self.trailing.clone(),
            bottom: self.bottom.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            center_x: self.center_x.clone(),
            center_y: self.center_y.clone(),
            objc: self.objc.clone(),
            objc_delegate: None
        }
    }
}

impl<T> Layout for WebView<T> {
    /// Returns the Objective-C object used for handling the view heirarchy.
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }

    fn add_subview<V: Layout>(&self, subview: &V) {

    }
}

impl<T> std::fmt::Debug for WebView<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebView ({:p})", self)
    }
}

impl<T> Drop for WebView<T> {
    /// A bit of extra cleanup for delegate callback pointers.
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.internal_callback_ptr {
                let _ = Rc::from_raw(ptr);
            }
        }
    }
}
