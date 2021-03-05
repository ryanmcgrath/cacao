//! A wrapper for NSMenuItem. Currently only supports menus going
//! one level deep; this could change in the future but is fine for
//! now.

use std::sync::Once;

use block::ConcreteBlock;
use objc::{class, msg_send, sel, sel_impl};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc_id::Id;

use crate::foundation::{id, nil, NSString, NSUInteger};
use crate::events::EventModifierFlag;

static BLOCK_PTR: &'static str = "cacaoMenuItemBlockPtr";

/// An Action is just an indirection layer to get around Rust and optimizing
/// zero-sum types; without this, pointers to callbacks will end up being 
/// 0x1, and all point to whatever is there first (unsure if this is due to 
/// Rust or Cocoa or what).
///
/// Point is, Button aren't created that much in the grand scheme of things,
/// and the heap isn't our enemy in a GUI framework anyway. If someone knows 
/// a better way to do this that doesn't require double-boxing, I'm all ears.
pub struct Action(Box<dyn Fn() + 'static>);

/// Internal method (shorthand) for generating `NSMenuItem` holders.
fn make_menu_item<S: AsRef<str>>(
    title: S,
    key: Option<&str>,
    action: Option<Sel>,
    modifiers: Option<&[EventModifierFlag]>
) -> Id<Object> {
    unsafe {
        let title = NSString::new(title.as_ref());

        // Note that AppKit requires a blank string if nil, not nil.
        let key = NSString::new(match key {
            Some(s) => s,
            None => ""
        });

        // Stock menu items that use selectors targeted at system pieces are just standard
        // `NSMenuItem`s. If there's no custom ones, we use our subclass that has a slot to store a
        // handler pointer.
        let alloc: id = msg_send![register_menu_item_class(), alloc];
        let item = Id::from_retained_ptr(match action {
            Some(a) => msg_send![alloc, initWithTitle:&*title action:a keyEquivalent:&*key],
            
            None => msg_send![alloc, initWithTitle:&*title 
                action:sel!(fireBlockAction:) 
                keyEquivalent:&*key]
        });

        if let Some(modifiers) = modifiers {
            let mut key_mask: NSUInteger = 0;

            for modifier in modifiers {
                let y: NSUInteger = modifier.into();
                key_mask = key_mask | y;
            }

            let _: () = msg_send![&*item, setKeyEquivalentModifierMask:key_mask];
        }

        item
    }
}

/// Represents varying `NSMenuItem` types - e.g, a separator vs an action. If you need something
/// outside of the stock item types, you can create a `Custom` variant that supports dispatching a
/// callback on the Rust side of things.
#[derive(Debug)]
pub enum MenuItem {
    /// A custom MenuItem. This type functions as a builder, so you can customize it easier.
    /// You can (and should) create this variant via the `new(title)` method, but if you need to do
    /// something crazier, then wrap it in this and you can hook into the Cacao menu system
    /// accordingly.
    Custom(Id<Object>),

    /// Shows a standard "About" item,  which will bring up the necessary window when clicked
    /// (include a `credits.html` in your App to make use of here). The argument baked in here
    /// should be your app name.
    About(String),

    /// A standard "hide the app" menu item.
    Hide,

    /// A standard "Services" menu item.
    Services,

    /// A "hide all other windows" menu item.
    HideOthers,

    /// A menu item to show all the windows for this app.
    ShowAll,

    /// Close the current window.
    CloseWindow,

    /// A "quit this app" menu icon.
    Quit,

    /// A menu item for enabling copying (often text) from responders.
    Copy,
    
    /// A menu item for enabling cutting (often text) from responders.
    Cut,

    /// An "undo" menu item; particularly useful for supporting the cut/copy/paste/undo lifecycle
    /// of events.
    Undo,

    /// An "redo" menu item; particularly useful for supporting the cut/copy/paste/undo lifecycle
    /// of events.
    Redo,
    
    /// A menu item for selecting all (often text) from responders.
    SelectAll,
    
    /// A menu item for pasting (often text) into responders.
    Paste,

    /// A standard "enter full screen" item.
    EnterFullScreen,

    /// An item for minimizing the window with the standard system controls.
    Minimize,

    /// An item for instructing the app to zoom. Your app must react to this with necessary window
    /// lifecycle events.
    Zoom,

    /// An item for automatically telling a SplitViewController to hide or show the sidebar. This
    /// only works on macOS 11.0+.
    ToggleSidebar,

    /// Represents a Separator. It's useful nonetheless for
    /// separating out pieces of the `NSMenu` structure.
    Separator
}

impl MenuItem {
    /// Consumes and returns a handle for the underlying MenuItem. This is internal as we make a few assumptions
    /// for how it interacts with our `Menu` setup, but this could be made public in the future.
    pub(crate) unsafe fn to_objc(self) -> Id<Object> {
        match self {
            Self::Custom(objc) => objc,
            
            Self::About(app_name) => {
                let title = format!("About {}", app_name);
                make_menu_item(&title, None, Some(sel!(orderFrontStandardAboutPanel:)), None)
            },

            Self::Hide => make_menu_item("Hide", Some("h"), Some(sel!(hide:)), None),

            // This one is a bit tricky to do right, as we need to expose a submenu, which isn't
            // supported by MenuItem yet.
            Self::Services => {
                let item = make_menu_item("Services", None, None, None);
                let app: id = msg_send![class!(RSTApplication), sharedApplication];
                let services: id = msg_send![app, servicesMenu];
                let _: () = msg_send![&*item, setSubmenu:services];
                item
            },

            Self::HideOthers => make_menu_item(
                "Hide Others",
                Some("h"),
                Some(sel!(hide:)),
                Some(&[EventModifierFlag::Command, EventModifierFlag::Option])
            ),

            Self::ShowAll => make_menu_item("Show All", None, Some(sel!(unhideAllApplications:)), None),
            Self::CloseWindow => make_menu_item("Close Window", Some("w"), Some(sel!(performClose:)), None),
            Self::Quit => make_menu_item("Quit", Some("q"), Some(sel!(terminate:)), None),
            Self::Copy => make_menu_item("Copy", Some("c"), Some(sel!(copy:)), None),
            Self::Cut => make_menu_item("Cut", Some("x"), Some(sel!(cut:)), None),
            Self::Undo => make_menu_item("Undo", Some("z"), Some(sel!(undo:)), None),
            Self::Redo => make_menu_item("Redo", Some("Z"), Some(sel!(redo:)), None),
            Self::SelectAll => make_menu_item("Select All", Some("a"), Some(sel!(selectAll:)), None),
            Self::Paste => make_menu_item("Paste", Some("v"), Some(sel!(paste:)), None),
            
            Self::EnterFullScreen => make_menu_item(
                "Enter Full Screen",
                Some("f"),
                Some(sel!(toggleFullScreen:)),
                Some(&[EventModifierFlag::Command, EventModifierFlag::Control])
            ),

            Self::Minimize => make_menu_item("Minimize", Some("m"), Some(sel!(performMiniaturize:)), None),
            Self::Zoom => make_menu_item("Zoom", None, Some(sel!(performZoom:)), None),

            Self::ToggleSidebar => make_menu_item(
                "Toggle Sidebar",
                Some("s"),
                Some(sel!(toggleSidebar:)),
                Some(&[EventModifierFlag::Command, EventModifierFlag::Option])
            ),

            Self::Separator => {
                let cls = class!(NSMenuItem);
                let separator: id = msg_send![cls, separatorItem];
                Id::from_ptr(separator)
            }
        }
    }

    /// Returns a `Custom` menu item, with the given title. You can configure this further with the
    /// builder methods on this object.
    pub fn new<S: AsRef<str>>(title: S) -> Self {
        MenuItem::Custom(make_menu_item(title, None, None, None))
    }

    /// Configures the a custom item to have specified key equivalent. This does nothing if called
    /// on a `MenuItem` type that is not `Custom`, 
    pub fn key(self, key: &str) -> Self {
        if let MenuItem::Custom(objc) = self {
            unsafe {
                let key = NSString::new(key);
                let _: () = msg_send![&*objc, setKeyEquivalent:key];
            }

            return MenuItem::Custom(objc);
        }

        self
    }

    /// Sets the modifier key flags for this menu item. This does nothing if called on a `MenuItem`
    /// that is not `Custom`.
    pub fn modifiers(self, modifiers: &[EventModifierFlag]) -> Self {
        if let MenuItem::Custom(objc) = self {
            let mut key_mask: NSUInteger = 0;

            for modifier in modifiers {
                let y: NSUInteger = modifier.into();
                key_mask = key_mask | y;
            }

            unsafe {
                let _: () = msg_send![&*objc, setKeyEquivalentModifierMask:key_mask];
            }

            return MenuItem::Custom(objc);
        }

        self
    }

    /// Attaches a target/action handler to dispatch events. This does nothing if called on a
    /// `MenuItem` that is not `Custom`.
    ///
    /// Note that we use an extra bit of unsafety here to pass over a heap'd block. We need to do
    /// this as some menu items live in odd places (the system menu bar), and we need the handlers
    /// to persist. We inject a custom dealloc method to pull the pointer back and drop the handler
    /// whenever the menu item goes kaput.
    pub fn action<F: Fn() + 'static>(self, action: F) -> Self {
        if let MenuItem::Custom(mut objc) = self {
            let handler = Box::new(Action(Box::new(action)));
            let ptr = Box::into_raw(handler);
            
            unsafe {
                (&mut *objc).set_ivar(BLOCK_PTR, ptr as usize);
                let _: () = msg_send![&*objc, setTarget:&*objc];
            }

            return MenuItem::Custom(objc);
        }

        self
    }
}

/// On the Objective-C side, we need to ensure our handler is dropped when this subclass
/// is deallocated. Note that NSMenuItem is seemingly odd outside of ARC contexts, and we
/// need to do some extra logic to ensure release calls are properly sent.
extern fn dealloc_cacao_menuitem(this: &Object, _: Sel) {
    unsafe {
        let ptr: usize = *this.get_ivar(BLOCK_PTR);
        let obj = ptr as *mut Action;
        
        if !obj.is_null() {
            let _handler = Box::from_raw(obj);
        }

        // This should be fine to _not_ do, but considering we go out of our way to loop it back on
        // itself, it's worth clearing out the slot.
        //let _: () = msg_send![this, setTarget:nil];

        let _: () = msg_send![super(this, class!(NSMenuItem)), dealloc];
    }
}

/// Called when our custom item needs to fire.
extern fn fire_block_action(this: &Object, _: Sel, _item: id) {
    let action = crate::utils::load::<Action>(this, BLOCK_PTR);
    (action.0)();
}

/// Injects a custom NSMenuItem subclass that contains a slot to hold a block, as well as a method
/// for calling the block.
///
/// In general, we do not want to do more than we need to here - menus are one of the last areas
/// where Carbon still lurks, and subclassing things can get weird.
pub(crate) fn register_menu_item_class() -> *const Class {
    static mut APP_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSMenuItem);
        let mut decl = ClassDecl::new("CacaoMenuItem", superclass).unwrap();
        decl.add_ivar::<usize>(BLOCK_PTR);

        decl.add_method(sel!(dealloc), dealloc_cacao_menuitem as extern fn(&Object, _));
        decl.add_method(sel!(fireBlockAction:), fire_block_action as extern fn(&Object, _, id));

        APP_CLASS = decl.register();
    });

    unsafe {
        APP_CLASS
    }
}
