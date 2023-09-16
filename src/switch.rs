//! A wrapper for NSSwitch. Currently the epitome of jank - if you're poking around here, expect
//! that this will change at some point.

use objc::rc::{Id, Shared};
use objc::runtime::{Class, Object};
use objc::{msg_send, msg_send_id, sel};

use crate::foundation::{id, load_or_register_class, nil, NSString, NO};
use crate::invoker::TargetActionHandler;
use crate::layout::Layout;
#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;

/// A wrapper for `NSSwitch`. Holds (retains) pointers for the Objective-C runtime
/// where our `NSSwitch` lives.
#[derive(Debug)]
pub struct Switch {
    /// A pointer to the underlying Objective-C Object.
    pub objc: ObjcProperty,
    handler: Option<TargetActionHandler>,

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

impl Switch {
    /// Creates a new `NSSwitch` instance, configures it appropriately,
    /// and retains the necessary Objective-C runtime pointer.
    pub fn new(text: &str) -> Self {
        let title = NSString::new(text);

        let view: id = unsafe {
            let button: id = msg_send![register_class(), buttonWithTitle: &*title, target: nil, action: nil];

            #[cfg(feature = "autolayout")]
            let _: () = msg_send![button, setTranslatesAutoresizingMaskIntoConstraints: NO];

            #[cfg(feature = "appkit")]
            let _: () = msg_send![button, setButtonType:3];

            button
        };

        Switch {
            handler: None,
            objc: ObjcProperty::retain(view),

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

    /// Sets whether this is checked on or off.
    pub fn set_checked(&mut self, checked: bool) {
        self.objc.with_mut(|obj| unsafe {
            // @TODO: The constants to use here changed back in 10.13ish, so... do we support that,
            // or just hide it?
            let _: () = msg_send![obj, setState:match checked {
                true => 1,
                false => 0
            }];
        });
    }

    /// Attaches a callback for button press events. Don't get too creative now...
    /// best just to message pass or something.
    pub fn set_action<F: Fn(*const Object) + Send + Sync + 'static>(&mut self, action: F) {
        // @TODO: This probably isn't ideal but gets the job done for now; needs revisiting.
        let this: Id<Object, Shared> = self.objc.get(|obj| unsafe { msg_send_id![obj, self] });
        let handler = TargetActionHandler::new(&*this, action);
        self.handler = Some(handler);
    }
}

impl ObjcAccess for Switch {
    fn with_backing_obj_mut(&self, handler: &dyn Fn(id)) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for Switch {
    fn add_subview<V: Layout>(&self, _view: &V) {
        panic!(
            r#"
            Tried to add a subview to a Switch. This is not allowed in Cacao. If you think this should be supported,
            open a discussion on the GitHub repo.
        "#
        );
    }
}

impl Drop for Switch {
    // Just to be sure, let's... nil these out. They should be weak references,
    // but I'd rather be paranoid and remove them later.
    fn drop(&mut self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTarget: nil];
            let _: () = msg_send![obj, setAction: nil];
        });
    }
}

/// Registers an `NSButton` subclass, and configures it to hold some ivars
/// for various things we need to store.
fn register_class() -> &'static Class {
    load_or_register_class("NSButton", "RSTSwitch", |decl| unsafe {})
}
