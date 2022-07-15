//! The main guts of the Preferences window. We store all our preferences in
//! `UserDefaults`, so there's not too much extra needed here - we can do most
//! event handlers inline.

use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::{View, ViewDelegate};

use crate::storage::Defaults;

use super::toggle_option_view::ToggleOptionView;

/// A general preferences view.
#[derive(Debug, Default)]
pub struct GeneralPreferencesContentView {
    pub example_option: ToggleOptionView
}

impl ViewDelegate for GeneralPreferencesContentView {
    const NAME: &'static str = "GeneralPreferencesContentView";

    fn did_load(&mut self, view: View) {
        self.example_option.configure(
            "An example preference",
            "This can be true, or it can be false.",
            Defaults::should_whatever(), // initial value
            Defaults::toggle_should_whatever
        );

        view.add_subview(&self.example_option.view);

        LayoutConstraint::activate(&[
            self.example_option.view.top.constraint_equal_to(&view.top).offset(22.),
            self.example_option
                .view
                .leading
                .constraint_equal_to(&view.leading)
                .offset(22.),
            self.example_option
                .view
                .trailing
                .constraint_equal_to(&view.trailing)
                .offset(-22.),
            self.example_option.view.bottom.constraint_equal_to(&view.bottom).offset(-22.)
        ]);
    }
}
