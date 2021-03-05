use objc_id::ShareId;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use crate::foundation::{id, nil, NSString};
use crate::layout::{Layout};
use crate::macos::toolbar::ToolbarItem;
use crate::view::{View, ViewController, ViewDelegate};
use crate::utils::Controller;

#[derive(Debug)]
pub struct SplitViewItem<T> {
    pub objc: ShareId<Object>,
    pub view_controller: ViewController<T>
}

impl<T> SplitViewItem<T>
where
    T: ViewDelegate + 'static
{
    pub fn item(view: T) -> Self {
        let view_controller = ViewController::new(view);

        let objc = unsafe {
            ShareId::from_ptr(msg_send![class!(NSSplitViewItem), splitViewItemWithViewController:&*view_controller.objc])
        };

        SplitViewItem {
            objc,
            view_controller
        }
    }

    pub fn sidebar(view: T) -> Self {
        let view_controller = ViewController::new(view);

        let objc = unsafe {
            ShareId::from_ptr(msg_send![class!(NSSplitViewItem), sidebarWithViewController:&*view_controller.objc])
        };

        SplitViewItem {
            objc,
            view_controller
        }
    }

    pub fn set_titlebar_separator_style(&self, style: crate::foundation::NSInteger) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitlebarSeparatorStyle:style];
        }
    }
}

#[derive(Debug)]
pub struct SplitViewController<Sidebar, Content> {
    pub objc: ShareId<Object>,
    pub sidebar: SplitViewItem<Sidebar>,
    pub content: SplitViewItem<Content>,
}

impl<Sidebar, Content> SplitViewController<Sidebar, Content>
where
    Sidebar: ViewDelegate + 'static,
    Content: ViewDelegate + 'static
{
    pub fn new(sidebar: Sidebar, content: Content) -> Self {
        let sidebar = SplitViewItem::sidebar(sidebar);
        let content = SplitViewItem::item(content);

        let objc = unsafe {
            let vc: id = msg_send![class!(NSSplitViewController), new];
            let _: () = msg_send![vc, addSplitViewItem:&*sidebar.objc];
            let _: () = msg_send![vc, addSplitViewItem:&*content.objc];
            ShareId::from_ptr(vc)
        };

        SplitViewController { objc, sidebar, content }
    }

    /// Toggles the sidebar, if it exists, with an animation. If there's no sidebar in this split view 
    /// (which is highly unlikely, unless you went out of your way to duck this) then it will do
    /// nothing.
    pub fn toggle_sidebar(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, toggleSidebar:nil];
        }
    }

    pub fn set_autosave_name(&self, name: &str) {
        let name = NSString::new(name);

        unsafe {
            let split_view: id = msg_send![&*self.objc, splitView];
            let _: () = msg_send![split_view, setAutosaveName:&*name];
        }
    }

    /*/// This method can be used to acquire an item for Toolbar instances that tracks a specified
    /// divider (`divider_index`) of this split view. This method is only supported on macOS 11.0+;
    /// it will return `None` on 10.15 and below.
    ///
    /// You should call this and pass + store the item in your Toolbar, and vend it to the system
    /// with your `ToolbarDelegate`.
    pub fn tracking_separator_toolbar_item(&self, divider_index: usize) -> Option<ToolbarItem> {
        if crate::utils::os::is_minimum_version(11) {
            unsafe {
                let split_view: id = msg_send![&*self.objc, splitView];
                let item: id = msg_send![class!(NSTrackingSeparatorToolbarItem), trackingSeparatorToolbarItemWithIdentifier:
                    splitView:split_view
                    dividerIndex:divider_index as NSInteger
                ];
            }
        }

        None
    }*/
}

impl<Sidebar, Content> Controller for SplitViewController<Sidebar, Content> {
    fn get_backing_node(&self) -> ShareId<Object> {
        self.objc.clone()
    }
}
