use objc_id::ShareId;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, NSString};
use crate::layout::Layout;
use crate::appkit::toolbar::ToolbarItem;
use crate::view::{View, ViewController, ViewDelegate};
use crate::utils::{os, Controller};

/// A SplitViewItem wraps a ViewController, and provides system hooks for operating in a
/// SplitView(Controller).
///
/// This is typically created for you when you create a `SplitViewController`, but is exported in
/// case you need to hook into the underlying platform pieces.
#[derive(Debug)]
pub struct SplitViewItem<T> {
    /// The underlying Objective-C Object.
    pub objc: ShareId<Object>,

    /// The wrapped ViewController.
    pub view_controller: ViewController<T>
}

impl<T> SplitViewItem<T>
where
    T: ViewDelegate + 'static
{
    /// Creates and returns a new `SplitViewItem`. This has no special properties out the default
    /// split view item type.
    pub fn item(view: T) -> Self {
        let view_controller = ViewController::new(view);

        SplitViewItem {
            objc: unsafe {
                ShareId::from_ptr(msg_send![class!(NSSplitViewItem),
                    splitViewItemWithViewController:&*view_controller.objc
                ])
            },

            view_controller
        }
    }

    /// Creates and returns a new `SplitViewItem`. The returned item is optimized to be a
    /// "sidebar"; that is, a typically left-most view that should be treated as such.
    ///
    /// On macOS Big Sur, this automatically gets the vibrancy backed sidebar view and will extend
    /// extend to the top of the window provided the other necessary window flags are set. On macOS
    /// versions prior to Big Sur, this returns a standard SplitViewItem.
    pub fn sidebar(view: T) -> Self {
        #[cfg(target_os = "macos")]
        {
            if !os::is_minimum_version(11) {
                return Self::item(view);
            }

            let view_controller = ViewController::new(view);

            SplitViewItem {
                objc: unsafe {
                    ShareId::from_ptr(msg_send![class!(NSSplitViewItem),
                        sidebarWithViewController:&*view_controller.objc
                    ])
                },

                view_controller
            }
        }

        // Non-macOS platforms default to the old-school API, where everything is just a generic
        // item.
        #[cfg(not(target_os = "macos"))]
        Self::item(view)
    }

    /// Sets the titlebar separator style for this `SplitView`.
    ///
    /// You'd use this if, say, you wanted a border under one part of the `SplitViewController` but
    /// not the other. This API was introduced in macOS 11.0 (Big Sur) and is a noop on anything
    /// prior.
    #[cfg(feature = "appkit")]
    pub fn set_titlebar_separator_style(&self, style: crate::foundation::NSInteger) {
        #[cfg(target_os = "macos")]
        if os::is_minimum_version(11) {
            unsafe {
                let _: () = msg_send![&*self.objc, setTitlebarSeparatorStyle:style];
            }
        }
    }
}

/// A SplitViewController manages two or more view controllers in a split-pane view.
///
/// You typically use this controller as a content view controller for a `Window`. With it, you can
/// build interfaces like those found in Mail.app or Xcode. Dividers can be configured to save
/// their positions so that users can adjust them as they please.
///
/// Note that the third pane is optional; you can opt to leave it `None`, in which case there's no
/// allocation there, or you can set a placeholder and use it as a details pane.
///
/// A note on property names: the Cocoa(Touch) controllers tend to view these as:
///
/// `|sidebar|details|content|`
///
/// This pattern fits things such as a the aforementioned apps (e.g, Mail). Cacao takes the
/// position that most apps really end up doing the following, though:
///
/// `|sidebar|content|details|`
///
/// where details may or may not be visible (e.g, chat applications often work this way).
#[derive(Debug)]
pub struct SplitViewController<Sidebar, Content, Details> {
    /// A reference to the underlying Objective-C split view controller.
    pub objc: ShareId<Object>,

    /// A reference to the sidebar `SplitViewItem`.
    pub sidebar: SplitViewItem<Sidebar>,

    /// A reference to the content `SplitViewItem`.
    pub content: SplitViewItem<Content>,

    /// An optional reference to the details `SplitViewItem`, if set.
    pub details: Option<SplitViewItem<Details>>
}

impl<Sidebar, Content, Details> SplitViewController<Sidebar, Content, Details>
where
    Sidebar: ViewDelegate + 'static,
    Content: ViewDelegate + 'static,
    Details: ViewDelegate + 'static
{
    /// Creates and returns a new `SplitViewController`.
    pub fn new(sidebar: Sidebar, content: Content, details: Option<Details>) -> Self {
        let sidebar = SplitViewItem::sidebar(sidebar);
        let content = SplitViewItem::item(content);

        let details = match details {
            Some(vc) => Some(SplitViewItem::item(vc)),
            None => None
        };

        let objc = unsafe {
            let vc: id = msg_send![class!(NSSplitViewController), new];
            let _: () = msg_send![vc, addSplitViewItem:&*sidebar.objc];
            let _: () = msg_send![vc, addSplitViewItem:&*content.objc];

            if let Some(details) = &details {
                let _: () = msg_send![vc, addSplitViewItem:&*details.objc];
            }

            ShareId::from_ptr(vc)
        };

        SplitViewController { objc, sidebar, content, details }
    }
}

impl<Sidebar, Content, Details> SplitViewController<Sidebar, Content, Details> {
    /// Toggles the sidebar, if it exists, with an animation. If there's no sidebar in this split view
    /// (which is highly unlikely, unless you went out of your way to duck this) then it will do
    /// nothing.
    pub fn toggle_sidebar(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, toggleSidebar:nil];
        }
    }

    /// Sets the autosave name for the underlying `SplitView`.
    ///
    /// Setting this name causes the system to persist separator locations to a defaults database,
    /// and the position(s) will be restored upon the user reopening the application.
    pub fn set_autosave_name(&self, name: &str) {
        let name = NSString::new(name);

        unsafe {
            let split_view: id = msg_send![&*self.objc, splitView];
            let _: () = msg_send![split_view, setAutosaveName:&*name];
        }
    }
}

impl<Sidebar, Content, Details> Controller for SplitViewController<Sidebar, Content, Details> {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }
}
