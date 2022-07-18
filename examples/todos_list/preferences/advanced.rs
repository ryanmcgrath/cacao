use cacao::text::{Label, TextAlign};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::{View, ViewDelegate};

/// A blank advanced preferences view.
#[derive(Debug, Default)]
pub struct AdvancedPreferencesContentView {
    label: Label
}

impl ViewDelegate for AdvancedPreferencesContentView {
    const NAME: &'static str = "AdvancedPreferencesContentView";
    
    fn did_load(&mut self, view: View) {
        self.label.set_text("And this is where advanced preferences would be... if we had any.");
        self.label.set_text_alignment(TextAlign::Center);
        view.add_subview(&self.label);

        LayoutConstraint::activate(&[
            self.label.top.constraint_equal_to(&view.top).offset(100.),
            self.label.leading.constraint_equal_to(&view.leading).offset(16.),
            self.label.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            self.label.bottom.constraint_equal_to(&view.bottom).offset(-100.)
        ]);
    }
}

