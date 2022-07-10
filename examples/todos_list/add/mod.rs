//! Implements a window for adding a new Todo.

use cacao::appkit::window::{Window, WindowDelegate};
use cacao::view::ViewController;

use crate::storage::{dispatch_ui, Message};

mod view;
use view::AddNewTodoContentView;

pub struct AddNewTodoWindow {
    pub content: ViewController<AddNewTodoContentView>,
}

impl AddNewTodoWindow {
    pub fn new() -> Self {
        let content = ViewController::new(AddNewTodoContentView::default());

        AddNewTodoWindow {
            content: content
        }
    }

    pub fn on_message(&self, message: Message) {
        if let Some(delegate) = &self.content.view.delegate {
            delegate.on_message(message);
        }
    }
}

impl WindowDelegate for AddNewTodoWindow {
    const NAME: &'static str = "AddNewTodoWindow";

    fn did_load(&mut self, window: Window) {
        window.set_autosave_name("AddNewTodoWindow");
        window.set_minimum_content_size(300, 100);
        window.set_title("Add a New Task");
        window.set_content_view_controller(&self.content);
    }

    fn cancel(&self) {
        dispatch_ui(Message::CloseSheet);
    }
}
