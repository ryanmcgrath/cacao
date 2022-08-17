use objc::{msg_send, sel, sel_impl};

use crate::foundation::id;
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};
use crate::utils::os;

/// A SafeAreaLayoutGuide should exist on all view types, and ensures that there are anchor points
/// that work within the system constraints. On macOS 11+, this will ensure you work around system
/// padding transprently - on macOS 10.15 and under, this will transparently map to the normal
/// edges, as the underlying properties were not supported there.
#[derive(Clone, Debug)]
pub struct SafeAreaLayoutGuide {
    /// A pointer to the Objective-C runtime top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    pub right: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    pub center_y: LayoutAnchorY,
}

impl SafeAreaLayoutGuide {
    /// Given a view pointer, will extract the safe area layout guide properties and return a
    /// `SafeAreaLayoutGuide` composed of them.
    pub fn new(view: id) -> Self {
        // For versions prior to Big Sur, we'll just use the default view anchors in place.
        let guide: id = match os::is_minimum_version(11) {
            true => unsafe { msg_send![view, layoutMarginsGuide] },
            false => view,
        };

        Self {
            top: LayoutAnchorY::top(guide),
            left: LayoutAnchorX::left(guide),
            leading: LayoutAnchorX::leading(guide),
            right: LayoutAnchorX::right(guide),
            trailing: LayoutAnchorX::trailing(guide),
            bottom: LayoutAnchorY::bottom(guide),
            width: LayoutAnchorDimension::width(guide),
            height: LayoutAnchorDimension::height(guide),
            center_x: LayoutAnchorX::center(guide),
            center_y: LayoutAnchorY::center(guide),
        }
    }
}
