use cacao::text::Label;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::switch::Switch;
use cacao::view::{View};

/// A reusable widget for a toggle; this is effectively a standard checkbox/label combination for
/// toggling a boolean value.
#[derive(Debug)]
pub struct ToggleOptionView {
    pub view: View,
    pub switch: Switch,
    pub title: Label,
    pub subtitle: Label
}

impl Default for ToggleOptionView {
    /// Creates and returns a stock toggle view.
    fn default() -> Self {
        let view = View::new();

        let switch = Switch::new("");
        view.add_subview(&switch);

        let title = Label::new();
        view.add_subview(&title);

        let subtitle = Label::new();
        view.add_subview(&subtitle);

        LayoutConstraint::activate(&[
            switch.top.constraint_equal_to(&view.top),
            switch.leading.constraint_equal_to(&view.leading),
            switch.width.constraint_equal_to_constant(24.),

            title.top.constraint_equal_to(&view.top),
            title.leading.constraint_equal_to(&switch.trailing),
            title.trailing.constraint_equal_to(&view.trailing),

            subtitle.top.constraint_equal_to(&title.bottom),
            subtitle.leading.constraint_equal_to(&switch.trailing),
            subtitle.trailing.constraint_equal_to(&view.trailing),
            subtitle.bottom.constraint_equal_to(&view.bottom)
        ]);

        ToggleOptionView {
            view,
            switch,
            title,
            subtitle
        }
    }
}

impl ToggleOptionView {
    /// Configures the widget. The handler will be fired on each state change of the checkbox; you
    /// can toggle your settings and such there.
    pub fn configure<F>(&mut self, text: &str, subtitle: &str, state: bool, handler: F)
    where
        F: Fn() + Send + Sync + 'static
    {
        self.title.set_text(text);
        self.subtitle.set_text(subtitle);
        self.switch.set_action(handler);
        self.switch.set_checked(state);
    }
}
