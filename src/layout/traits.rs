//! Various traits related to controllers opting in to autolayout routines and support for view
//! heirarchies.

use core_graphics::geometry::{CGRect, CGPoint, CGSize};

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, YES, NO};
use crate::geometry::Rect;

/// A trait that view wrappers must conform to. Enables managing the subview tree.
#[allow(unused_variables)]
pub trait Layout {
    /// Returns a reference to the backing Objective-C layer. This is optional, as we try to keep
    /// the general lazy-loading approach Cocoa has. This may change in the future, and in general
    /// this shouldn't affect your code too much (if at all).
    fn with_backing_node<F: Fn(id)>(&self, handler: F);

    /// Adds another Layout-backed control or view as a subview of this view.
    fn add_subview<V: Layout>(&self, view: &V) {
        self.with_backing_node(|backing_node| {
            view.with_backing_node(|subview_node| unsafe {
                let _: () = msg_send![backing_node, addSubview:subview_node];
            });
        });
    }

    /// Removes a control or view from the superview.
    fn remove_from_superview(&self) {
        self.with_backing_node(|backing_node| unsafe {
            let _: () = msg_send![backing_node, removeFromSuperview];
        });
    }

    /// Sets the `frame` for the view this trait is applied to.
    ///
    /// Note that Cacao, by default, opts into autolayout - you need to call
    /// `set_translates_autoresizing_mask_into_constraints` to enable frame-based layout calls (or
    /// use an appropriate initializer for a given view type).
    fn set_frame<R: Into<CGRect>>(&self, rect: R) {
        let frame: CGRect = rect.into();
        
        self.with_backing_node(move |backing_node| unsafe {
            let _: () = msg_send![backing_node, setFrame:frame];
        });
    }
    
    /// Sets whether the view for this trait should translate autoresizing masks into layout
    /// constraints.
    ///
    /// Cacao defaults this to `false`; if you need to set frame-based layout pieces,
    /// then you should set this to `true` (or use an appropriate initializer that does it for you).
    fn set_translates_autoresizing_mask_into_constraints(&self, translates: bool) {
        self.with_backing_node(|backing_node| unsafe {
            let _: () = msg_send![backing_node, setTranslatesAutoresizingMaskIntoConstraints:match translates {
                true => YES,
                false => NO
            }];
        });
    }
}
