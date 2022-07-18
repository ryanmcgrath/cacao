//! The root view controller for our Todos window. All we do here is attach our ListView and pass
//! messages downwards. You could theoretically remove this layer of indirection, but a more
//! complicated app probably wouldn't, and I figure it's worth having this here for those who might
//! use this example as a jumping-off point.

use cacao::layout::{Layout, LayoutConstraint};
use cacao::listview::ListView;
use cacao::view::{View, ViewDelegate};

use super::list::TodosListView;
use crate::storage::Message;

#[derive(Debug)]
pub struct TodosContentView {
    pub todos_list_view: ListView<TodosListView>
}

impl Default for TodosContentView {
    fn default() -> Self {
        TodosContentView {
            todos_list_view: ListView::with(TodosListView::default())
        }
    }
}

impl TodosContentView {
    pub fn on_message(&self, message: Message) {
        if let Some(delegate) = &self.todos_list_view.delegate {
            delegate.on_message(message);
        }
    }
}

impl ViewDelegate for TodosContentView {
    const NAME: &'static str = "TodosContentView";

    fn did_load(&mut self, view: View) {
        view.add_subview(&self.todos_list_view);

        LayoutConstraint::activate(&[
            self.todos_list_view.top.constraint_equal_to(&view.top),
            self.todos_list_view.leading.constraint_equal_to(&view.leading),
            self.todos_list_view.trailing.constraint_equal_to(&view.trailing),
            self.todos_list_view.bottom.constraint_equal_to(&view.bottom)
        ]);
    }
}
