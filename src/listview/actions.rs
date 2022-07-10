use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

use block::ConcreteBlock;

use crate::color::Color;
use crate::foundation::{id, NSString, NSUInteger};
use crate::image::Image;

/// Represents the "type" or "style" of row action. A `Regular` action is
/// nothing special; do whatever you want. A `Destructive` action will have
/// a special animation when being deleted, among any other system specialties.
#[derive(Debug)]
pub enum RowActionStyle {
    /// The stock, standard, regular action.
    Regular,

    /// Use this to denote that an action is destructive.
    Destructive
}

impl Default for RowActionStyle {
    fn default() -> Self {
        RowActionStyle::Regular
    }
}

impl From<RowActionStyle> for NSUInteger {
    fn from(style: RowActionStyle) -> Self {
        match style {
            RowActionStyle::Regular => 0,
            RowActionStyle::Destructive => 1
        }
    }
}

/// Represents an action that can be displayed when a user swipes-to-reveal
/// on a ListViewRow. You return this from the appropriate delegate method,
/// and the system will handle displaying the necessary pieces for you.
#[derive(Debug)]
pub struct RowAction(pub Id<Object>);

impl RowAction {
    /// Creates and returns a new `RowAction`. You'd use this handler to
    /// configure whatever action you want to show when a user swipes-to-reveal
    /// on your ListViewRow.
    ///
    /// Additional configuration can be done after initialization, if need be.
    ///
    /// These run on the main thread, as they're UI handlers - so we can avoid Send + Sync on
    /// our definitions.
    pub fn new<F>(title: &str, style: RowActionStyle, handler: F) -> Self
    where
        F: Fn(RowAction, usize) + 'static
    {
        let title = NSString::new(title);
        let block = ConcreteBlock::new(move |action: id, row: NSUInteger| {
            let action = RowAction(unsafe {
                Id::from_ptr(action)
            });

            handler(action, row as usize);
        });
        let block = block.copy();
        let style = style as NSUInteger;

        RowAction(unsafe {
            let cls = class!(NSTableViewRowAction);
            Id::from_ptr(msg_send![cls, rowActionWithStyle:style
                title:&*title
                handler:block
            ])
        })
    }

    /// Sets the title of this action.
    pub fn set_title(&mut self, title: &str) {
        let title = NSString::new(title);

        unsafe {
            let _: () = msg_send![&*self.0, setTitle:&*title];
        }
    }

    /// Sets the background color of this action.
    pub fn set_background_color<C: AsRef<Color>>(&mut self, color: C) {
        let color: id = color.as_ref().into();

        unsafe {
            let _: () = msg_send![&*self.0, setBackgroundColor:color];
        }
    }

    /// Sets the style of this action.
    pub fn set_style(&mut self, style: RowActionStyle) {
        let style = style as NSUInteger;

        unsafe {
            let _: () = msg_send![&*self.0, setStyle:style];
        }
    }

    /// Sets an optional image for this action.
    pub fn set_image(&mut self, image: Image) {
        unsafe {
            let _: () = msg_send![&*self.0, setImage:&*image.0];
        }
    }
}
