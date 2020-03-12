//! Traits that can be used for Toolbar construction. Relatively straightforward, as far as these
//! go. Currently a bit incomplete in that we don't support the customizing workflow, but feel free
//! to pull request it.

use crate::toolbar::item::ToolbarItem;

/// A trait that you can implement to have your struct/etc act as an `NSToolbarDelegate`.
pub trait ToolbarController {
    /// What items are allowed in this toolbar.
    fn allowed_item_identifiers(&self) -> Vec<&'static str>;

    /// The default items in this toolbar.
    fn default_item_identifiers(&self) -> Vec<&'static str>;

    /// For a given `identifier`, return the `ToolbarItem` that should be displayed.
    fn item_for(&self, _identifier: &str) -> ToolbarItem;
}
