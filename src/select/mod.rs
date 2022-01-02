//! Implements a Select-style dropdown. By default this uses NSPopupSelect on macOS.

use std::sync::Once;

use core_graphics::geometry::CGRect;

use objc_id::ShareId;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use crate::control::Control;
use crate::foundation::{id, nil, YES, NO, NSString, NSInteger};
use crate::invoker::TargetActionHandler;
use crate::geometry::Rect;
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;

#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorX, LayoutAnchorY, LayoutAnchorDimension};

/// Wraps `NSPopUpSelect` on AppKit. Not currently implemented for iOS.
///
/// Acts like a `<select>` dropdown, if you're familiar with HTML. Use for dropdown option
/// selecting.
///
/// Some properties are platform-specific; see the documentation for further information.
///
/// ```rust,no_run
/// let mut dropdown = Select::new();
///
/// // Make sure you don't let your Select drop for as long as you need it.
/// my_view.add_subview(&dropdown);
/// ```
#[derive(Debug)]
pub struct Select {
    /// A handle for the underlying Objective-C object.
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

impl Select {
    /// Creates a new `Select` instance, configures it appropriately,
    /// and retains the necessary Objective-C runtime pointer.
    pub fn new() -> Self {
        let zero: CGRect = Rect::zero().into();
        
        let view: id = unsafe {
            let alloc: id = msg_send![register_class(), alloc];
            let select: id = msg_send![alloc, initWithFrame:zero pullsDown:NO];

            #[cfg(feature = "autolayout")]
            let _: () = msg_send![select, setTranslatesAutoresizingMaskIntoConstraints:NO];
            
            select
        };
        
        Select {
            handler: None,

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
            
            objc: ObjcProperty::retain(view),
        }
    }

    /// Attaches a callback for selection events.
    /// Much like `Button`, this really needs to be revisited.
    ///
    /// Really, this is not ideal.
    ///
    /// I cannot stress this enough.
    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        // @TODO: This probably isn't ideal but gets the job done for now; needs revisiting.
        let this = self.objc.get(|obj| unsafe { ShareId::from_ptr(msg_send![obj, self]) });
        let handler = TargetActionHandler::new(&*this, action);
        self.handler = Some(handler);
    }

    /// Sets whether this pulls down (dropdown) or pops up.
    pub fn set_pulls_down(&self, pulls_down: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setPullsDown:match pulls_down {
                true => YES,
                false => NO
            }];
        });
    }

    /// Adds an item to the dropdown list.
    pub fn add_item(&self, title: &str) {
        self.objc.with_mut(|obj| unsafe {
            let s = NSString::new(title);
            let _: () = msg_send![obj, addItemWithTitle:s];
        });
    }

    /// Removes all items from the dropdown list.
    pub fn remove_all_items(&self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, removeAllItems];
        });
    }

    /// Remove the item at the specified index.
    pub fn remove_item_at_index(&self, index: usize) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, removeItemAtIndex:index];
        });
    }

    pub fn set_selected_index(&self, index: NSInteger) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, selectItemAtIndex:index];
        });
    }

    /// Gets the selected index.
    pub fn get_selected_index(&self) -> usize {
        self.objc.get(|obj| unsafe {
            let index: NSInteger = msg_send![obj, indexOfSelectedItem];
            index as usize
        })
    }

    /// Returns the number of items in the dropdown.
    pub fn len(&self) -> usize {
        self.objc.get(|obj| unsafe {
            let index: NSInteger = msg_send![obj, numberOfItems];
            index as usize
        })       
    }
}

impl ObjcAccess for Select {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for Select {
    fn add_subview<V: Layout>(&self, _view: &V) { 
        panic!(r#"
            Tried to add a subview to a Select. This is not allowed in Cacao. If you think this should be supported, 
            open a discussion on the GitHub repo.
        "#);
    }
}

impl Control for Select {}

impl ObjcAccess for &Select {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for &Select {
    fn add_subview<V: Layout>(&self, _view: &V) { 
        panic!(r#"
            Tried to add a subview to a Select. This is not allowed in Cacao. If you think this should be supported, 
            open a discussion on the GitHub repo.
        "#);
    }
}

impl Control for &Select {}

impl Drop for Select {
    /// Nils out references on the Objective-C side and removes this from the backing view.
    // Just to be sure, let's... nil these out. They should be weak references,
    // but I'd rather be paranoid and remove them later.
    fn drop(&mut self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTarget:nil];
            let _: () = msg_send![obj, setAction:nil];
        });
    }
}

/// Registers an `NSSelect` subclass, and configures it to hold some ivars 
/// for various things we need to store.
fn register_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSPopUpButton);
        let decl = ClassDecl::new("CacaoSelect", superclass).unwrap(); 
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
