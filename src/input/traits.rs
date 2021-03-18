//! Various traits used for Labels.

use crate::input::TextField;

/// This trait can be used for implementing custom text field behavior.
#[allow(unused_variables)]
pub trait TextFieldDelegate {
    /// Used to cache subclass creations on the Objective-C side.
    /// You can just set this to be the name of your view type. This
    /// value *must* be unique per-type.
    const NAME: &'static str;

    /// You should rarely (read: probably never) need to implement this yourself.
    /// It simply acts as a getter for the associated `NAME` const on this trait.
    fn subclass_name(&self) -> &'static str {
        Self::NAME
    }

    /// Called when the text field is loaded. You're passed a reference to the underlying text
    /// field for future local use.
    fn did_load(&mut self, view: TextField) {}

    /// Posts a notification when the text is no longer in edit mode.
    fn text_did_end_editing(&self, value: &str) {}

    /// Requests permission to begin editing a text object.
    fn text_should_begin_editing(&self, value: &str) -> bool {
        true
    }

    /// Posts a notification to the default notification center that the text is about to go into edit mode.
    fn text_did_begin_editing(&self, value: &str) {}

    /// Posts a notification when the text changes, and forwards the message to the text field’s cell if it responds.
    fn text_did_change(&self, value: &str) {}

    /// Performs validation on the text field’s new value.
    fn text_should_end_editing(&self, value: &str) -> bool {
        true
    }
}
