//! The main Todos window.

use cacao::appkit::toolbar::Toolbar;
use cacao::appkit::window::{Window, WindowDelegate};
use cacao::view::ViewController;

use crate::storage::Message;

mod toolbar;
use toolbar::TodosToolbar;

mod content_view;
use content_view::TodosContentView;

mod list;

pub struct TodosWindow {
    pub content: ViewController<TodosContentView>,
    pub toolbar: Toolbar<TodosToolbar>,
}

impl TodosWindow {
    pub fn new() -> Self {
        TodosWindow {
            content: ViewController::new(TodosContentView::default()),
            toolbar: Toolbar::new("TodosToolbar", TodosToolbar::default()),
        }
    }

    pub fn on_message(&self, message: Message) {
        if let Some(delegate) = &self.content.view.delegate {
            delegate.on_message(message);
        }
    }
}

impl WindowDelegate for TodosWindow {
    const NAME: &'static str = "TodosWindow";

    fn did_load(&mut self, window: Window) {
        window.set_autosave_name("TodosWindow");
        window.set_minimum_content_size(400, 400);
        window.set_movable_by_background(true);
        window.set_title("Tasks");

        window.set_toolbar(&self.toolbar);
        window.set_content_view_controller(&self.content);
    }
}
