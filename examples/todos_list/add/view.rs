//! Implements a view for adding a new task/todo. This is still a bit cumbersome as patterns are
//! worked out, but a quick explainer: when the user clicks the "add" button, we dispatch a message
//! which flows back through here, and then we grab the value and process it.
//!
//! Ownership in Rust makes it tricky to do this right, and the TextField widget may undergo more
//! changes before version 0.1. This approach is unlikely to break as an example while those
//! changes are poked and prodded at, even if it is a bit verbose and confusing.

use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::Label;
use cacao::view::{View, ViewDelegate};

use cacao::button::Button;
use cacao::input::TextField;

use crate::storage::{dispatch_ui, Message};

#[derive(Debug, Default)]
pub struct AddNewTodoContentView {
    pub view: Option<View>,
    pub input: Option<TextField>,
    pub button: Option<Button>,
}

impl AddNewTodoContentView {
    pub fn on_message(&self, message: Message) {
        match message {
            Message::ProcessNewTodo => {
                if let Some(input) = &self.input {
                    let task = input.get_value();
                    if task != "" {
                        dispatch_ui(Message::StoreNewTodo(task));
                    }
                }
            },

            _ => {},
        }
    }
}

impl ViewDelegate for AddNewTodoContentView {
    const NAME: &'static str = "AddNewTodoContentView";

    fn did_load(&mut self, view: View) {
        let instructions = Label::new();
        instructions.set_text("Let's be real: we both know this task isn't getting done.");

        let input = TextField::new();

        let mut button = Button::new("Add");
        button.set_key_equivalent("\r");
        button.set_action(|| dispatch_ui(Message::ProcessNewTodo));

        view.add_subview(&instructions);
        view.add_subview(&input);
        view.add_subview(&button);

        LayoutConstraint::activate(&[
            instructions.top.constraint_equal_to(&view.top).offset(16.),
            instructions.leading.constraint_equal_to(&view.leading).offset(16.),
            instructions.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            input.top.constraint_equal_to(&instructions.bottom).offset(8.),
            input.leading.constraint_equal_to(&view.leading).offset(16.),
            input.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            button.top.constraint_equal_to(&input.bottom).offset(8.),
            button.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            button.bottom.constraint_equal_to(&view.bottom).offset(-16.),
        ]);

        self.view = Some(view);
        self.input = Some(input);
        self.button = Some(button);
    }
}
