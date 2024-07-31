//! Various traits related to controllers opting in to autolayout routines and support for view
//! heirarchies.

use objc::foundation::{CGFloat, NSRect};
use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{msg_send, sel};

use crate::foundation::{id, nil, NSArray, NSString};
use crate::geometry::Rect;
use crate::objc_access::ObjcAccess;

#[cfg(feature = "appkit")]
use crate::pasteboard::PasteboardType;

/// A trait that view wrappers must conform to. Enables managing the subview tree.
#[allow(unused_variables)]
pub trait Layout: ObjcAccess {
    /// Sets whether this needs to be redrawn before being displayed.
    ///
    /// If you're updating data that dynamically impacts this view, mark this as true - the next
    /// pass from the system will redraw it accordingly, and set the underlying value back to
    /// `false`.
    fn set_needs_display(&self, needs_display: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setNeedsDisplay: needs_display];
        });
    }

    /// Adds another Layout-backed control or view as a subview of this view.
    fn add_subview<V: Layout>(&self, view: &V) {
        self.with_backing_obj_mut(|backing_node| {
            view.with_backing_obj_mut(|subview_node| unsafe {
                let _: () = msg_send![backing_node, addSubview: subview_node];
            });
        });
    }

    /// Removes a control or view from the superview.
    fn remove_from_superview(&self) {
        self.with_backing_obj_mut(|backing_node| unsafe {
            let _: () = msg_send![backing_node, removeFromSuperview];
        });
    }

    /// Sets the `frame` for the view this trait is applied to.
    ///
    /// Note that Cacao, by default, opts into autolayout - you need to call
    /// `set_translates_autoresizing_mask_into_constraints` to enable frame-based layout calls (or
    /// use an appropriate initializer for a given view type).
    fn set_frame<R: Into<NSRect>>(&self, rect: R) {
        let frame: NSRect = rect.into();

        self.with_backing_obj_mut(move |backing_node| unsafe {
            let _: () = msg_send![backing_node, setFrame: frame];
        });
    }

    /// Sets whether the view for this trait should translate autoresizing masks into layout
    /// constraints.
    ///
    /// Cacao defaults this to `false`; if you need to set frame-based layout pieces,
    /// then you should set this to `true` (or use an appropriate initializer that does it for you).
    #[cfg(feature = "autolayout")]
    fn set_translates_autoresizing_mask_into_constraints(&self, translates: bool) {
        self.with_backing_obj_mut(|backing_node| unsafe {
            let _: () = msg_send![backing_node, setTranslatesAutoresizingMaskIntoConstraints: translates];
        });
    }

    /// Sets whether the view for this is hidden or not.
    ///
    /// When hidden, widgets don't receive events and is not visible.
    fn set_hidden(&self, hide: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setHidden: hide];
        });
    }

    /// Returns whether this is hidden or not.
    ///
    /// Note that this can report `false` if an ancestor widget is hidden, thus hiding this - to check in
    /// that case, you may want `is_hidden_or_ancestor_is_hidden()`.
    fn is_hidden(&self) -> bool {
        self.get_from_backing_obj(|obj| unsafe { msg_send![obj, isHidden] })
    }

    /// Returns whether this is hidden, *or* whether an ancestor view is hidden.
    #[cfg(feature = "appkit")]
    fn is_hidden_or_ancestor_is_hidden(&self) -> bool {
        self.get_from_backing_obj(|obj| unsafe { msg_send![obj, isHiddenOrHasHiddenAncestor] })
    }

    /// Register this view for drag and drop operations.
    ///
    /// This should be supported under UIKit as well, but is featured gated under AppKit
    /// currently to avoid compile issues.
    #[cfg(feature = "appkit")]
    fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        let types: NSArray = types
            .into_iter()
            .map(|t| {
                let x: NSString = (*t).into();
                // FIXME: We shouldn't use autorelease here
                Id::autorelease_return(x.objc)
            })
            .collect::<Vec<id>>()
            .into();

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, registerForDraggedTypes:&*types];
        });
    }

    /// Unregisters this as a target for drag and drop operations.
    ///
    /// This should be supported under UIKit as well, but is featured gated under AppKit
    /// currently to avoid compile issues.
    #[cfg(feature = "appkit")]
    fn unregister_dragged_types(&self) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, unregisterDraggedTypes];
        });
    }

    /// Sets whether this posts notifications when the frame rectangle changes.
    ///
    /// If you have a high performance tableview or collectionview that has issues, disabling these
    /// can be helpful - but always test!
    #[cfg(feature = "appkit")]
    fn set_posts_frame_change_notifications(&self, posts: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setPostsFrameChangedNotifications: posts];
        });
    }

    /// Sets whether this posts notifications when the bounds rectangle changes.
    ///
    /// If you have a high performance tableview or collectionview that has issues, disabling these
    /// can be helpful - but always test!
    #[cfg(feature = "appkit")]
    fn set_posts_bounds_change_notifications(&self, posts: bool) {
        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setPostsBoundsChangedNotifications: posts];
        });
    }

    /// Theoretically this belongs elsewhere, but we want to enable this on all view layers, since
    /// it's common enough anyway.
    #[cfg(feature = "appkit")]
    fn set_alpha(&self, value: f64) {
        let value: CGFloat = value.into();

        self.with_backing_obj_mut(|obj| unsafe {
            let _: () = msg_send![obj, setAlphaValue: value];
        });
    }
}
