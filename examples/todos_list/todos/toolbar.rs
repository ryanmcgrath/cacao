//! The main Todos window toolbar. Contains a button to enable adding a new task.

use cacao::appkit::toolbar::{ItemIdentifier, Toolbar, ToolbarDelegate, ToolbarDisplayMode, ToolbarItem};
use cacao::button::Button;

use crate::storage::{dispatch_ui, Message};

#[derive(Debug)]
pub struct TodosToolbar(ToolbarItem);

impl Default for TodosToolbar {
    fn default() -> Self {
        TodosToolbar({
            let mut item = ToolbarItem::new("AddTodoButton");
            item.set_title("Add Todo");
            item.set_button(Button::new("+ New"));

            item.set_action(|| {
                dispatch_ui(Message::OpenNewTodoSheet);
            });

            item
        })
    }
}

impl ToolbarDelegate for TodosToolbar {
    const NAME: &'static str = "TodosToolbar";

    fn did_load(&mut self, toolbar: Toolbar) {
        toolbar.set_display_mode(ToolbarDisplayMode::IconOnly);
    }

    fn allowed_item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![ItemIdentifier::Custom("AddTodoButton")]
    }

    fn default_item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![ItemIdentifier::Custom("AddTodoButton")]
    }

    // We only have one item, so we don't care about the identifier.
    fn item_for(&self, _identifier: &str) -> &ToolbarItem {
        &self.0
    }
}
