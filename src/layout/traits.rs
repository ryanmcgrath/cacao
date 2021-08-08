//! Various traits related to controllers opting in to autolayout routines and support for view
//! heirarchies.

use core_graphics::geometry::{CGRect, CGPoint, CGSize};

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, nil, to_bool, YES, NO, NSArray, NSString};
use crate::geometry::Rect;

#[cfg(feature = "appkit")]
use crate::pasteboard::PasteboardType;

/// A trait that view wrappers must conform to. Enables managing the subview tree.
#[allow(unused_variables)]
pub trait Layout {
    /// Used for mutably interacting with the underlying Objective-C instance.
    fn with_backing_node<F: Fn(id)>(&self, handler: F);

    /// Used for checking backing properties of the underlying Objective-C instance, without
    /// needing a mutable borrow.
    fn get_from_backing_node<F: Fn(&Object) -> R, R>(&self, handler: F) -> R;

    /// Sets whether this needs to be redrawn before being displayed.
    ///
    /// If you're updating data that dynamically impacts this view, mark this as true - the next
    /// pass from the system will redraw it accordingly, and set the underlying value back to
    /// `false`.
    fn set_needs_display(&self, needs_display: bool) {
        self.with_backing_node(|obj| unsafe {
            let _: () = msg_send![obj, setNeedsDisplay:match needs_display {
                true => YES,
                false => NO
            }];
        });
    }

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

    /// Sets whether the view for this is hidden or not.
    ///
    /// When hidden, widgets don't receive events and is not visible. 
    fn set_hidden(&self, hide: bool) {
        self.with_backing_node(|obj| unsafe {
            let _: () = msg_send![obj, setHidden:match hide {
                true => YES,
                false => NO
            }];
        });
    }

    /// Returns whether this is hidden or not.
    /// 
    /// Note that this can report `false` if an ancestor widget is hidden, thus hiding this - to check in 
    /// that case, you may want `is_hidden_or_ancestor_is_hidden()`.
    fn is_hidden(&self) -> bool {
        self.get_from_backing_node(|obj| {
            to_bool(unsafe {
                msg_send![obj, isHidden]
            })
        })
    }
    
    /// Returns whether this is hidden, *or* whether an ancestor view is hidden.
    #[cfg(feature = "appkit")]
    fn is_hidden_or_ancestor_is_hidden(&self) -> bool {
        self.get_from_backing_node(|obj| {
            to_bool(unsafe {
                msg_send![obj, isHiddenOrHasHiddenAncestor]
            })
        })
    }

    /// Register this view for drag and drop operations.
    ///
    /// This should be supported under UIKit as well, but is featured gated under AppKit
    /// currently to avoid compile issues.
    #[cfg(feature = "appkit")]
    fn register_for_dragged_types(&self, types: &[PasteboardType]) {
        let types: NSArray = types.into_iter().map(|t| {
            let x: NSString = (*t).into();
            x.into()
        }).collect::<Vec<id>>().into();

        self.with_backing_node(|obj| unsafe {
            let _: () = msg_send![obj, registerForDraggedTypes:&*types];
        });
    }

    /// Unregisters this as a target for drag and drop operations.
    ///
    /// This should be supported under UIKit as well, but is featured gated under AppKit
    /// currently to avoid compile issues.
    #[cfg(feature = "appkit")]
    fn unregister_dragged_types(&self) { 
        self.with_backing_node(|obj| unsafe {
            let _: () = msg_send![obj, unregisterDraggedTypes];
        });
    }

    /// Sets whether this posts notifications when the frame rectangle changes.
    ///
    /// If you have a high performance tableview or collectionview that has issues, disabling these
    /// can be helpful - but always test!
    #[cfg(feature = "appkit")]
    fn set_posts_frame_change_notifications(&self, posts: bool) {
        self.with_backing_node(|obj| unsafe {
            let _: () = msg_send![obj, setPostsFrameChangedNotifications:match posts {
                true => YES,
                false => NO
            }];
        });
    }

    /// Sets whether this posts notifications when the bounds rectangle changes.
    ///
    /// If you have a high performance tableview or collectionview that has issues, disabling these
    /// can be helpful - but always test!
    #[cfg(feature = "appkit")]
    fn set_posts_bounds_change_notifications(&self, posts: bool) {
        self.with_backing_node(|obj| unsafe {
            let _: () = msg_send![obj, setPostsBoundsChangedNotifications:match posts {
                true => YES,
                false => NO
            }];
        });
    }
}
