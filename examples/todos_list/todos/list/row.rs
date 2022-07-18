use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::{Font, Label, LineBreakMode};
use cacao::view::{View, ViewDelegate};

use crate::storage::{Todo, TodoStatus};

/// This view is used as a row in our `TodosListView`. It displays information/status for a provided
/// `Todo`, and gets cached and reused as necessary. It should not store anything long-term.
#[derive(Default, Debug)]
pub struct TodoViewRow {
    pub title: Label,
    pub status: Label
}

impl TodoViewRow {
    /// Called when this view is being presented, and configures itself with the given todo.
    pub fn configure_with(&mut self, todo: &Todo) {
        self.title.set_text(&todo.title);

        match todo.status {
            TodoStatus::Incomplete => {
                self.status.set_text_color(Color::SystemRed);
                self.status.set_text("Incomplete");
            },

            TodoStatus::Complete => {
                self.status.set_text_color(Color::SystemBlue);
                self.status.set_text("Complete");
            }
        }
    }
}

impl ViewDelegate for TodoViewRow {
    const NAME: &'static str = "TodoViewRow";

    /// Called when the view is first created; handles setup of layout and associated styling that
    /// doesn't change.
    fn did_load(&mut self, view: View) {
        view.add_subview(&self.title);
        view.add_subview(&self.status);

        self.title.set_line_break_mode(LineBreakMode::TruncateMiddle);

        let font = Font::system(10.);
        self.status.set_font(&font);

        LayoutConstraint::activate(&[
            self.title.top.constraint_equal_to(&view.top).offset(16.),
            self.title.leading.constraint_equal_to(&view.leading).offset(16.),
            self.title.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            self.status.top.constraint_equal_to(&self.title.bottom).offset(8.),
            self.status.leading.constraint_equal_to(&view.leading).offset(16.),
            self.status.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            self.status.bottom.constraint_equal_to(&view.bottom).offset(-16.)
        ]);
    }
}
