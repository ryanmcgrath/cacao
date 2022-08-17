//! This implements our ListView, which displays and helps interact with the Todo list. This is a
//! mostly single-threaded example, so we can get away with cutting a few corners and keeping our
//! data store in here - but for a larger app, you'd likely do something else.

use cacao::listview::{ListView, ListViewDelegate, ListViewRow, RowAction, RowActionStyle, RowAnimation, RowEdge};

use crate::storage::{dispatch_ui, Message, TodoStatus, Todos};

mod row;
use row::TodoViewRow;

/// An identifier for the cell(s) we dequeue.
const TODO_ROW: &'static str = "TodoViewRowCell";

/// The list view for todos.
#[derive(Debug, Default)]
pub struct TodosListView {
    view: Option<ListView>,
    todos: Todos,
}

impl TodosListView {
    /// This manages updates to the underlying database. You might opt to do this elsewhere, etc.
    pub fn on_message(&self, message: Message) {
        match message {
            Message::MarkTodoComplete(row) => {
                self.todos.with_mut(row, |todo| todo.status = TodoStatus::Complete);
                if let Some(view) = &self.view {
                    view.reload_rows(&[row]);
                    view.set_row_actions_visible(false);
                }
            },

            Message::MarkTodoIncomplete(row) => {
                self.todos.with_mut(row, |todo| todo.status = TodoStatus::Incomplete);

                if let Some(view) = &self.view {
                    view.reload_rows(&[row]);
                    view.set_row_actions_visible(false);
                }
            },

            Message::StoreNewTodo(todo) => {
                self.todos.insert(todo);
                self.view.as_ref().unwrap().perform_batch_updates(|listview| {
                    // We know we always insert at the 0 index, so this is a simple calculation.
                    // You'd need to diff yourself for anything more complicated.
                    listview.insert_rows(&[0], RowAnimation::SlideDown);
                });
            },

            _ => {},
        }
    }
}

impl ListViewDelegate for TodosListView {
    const NAME: &'static str = "TodosListView";

    /// Essential configuration and retaining of a `ListView` handle to do updates later on.
    fn did_load(&mut self, view: ListView) {
        view.register(TODO_ROW, TodoViewRow::default);
        view.set_uses_alternating_backgrounds(true);
        view.set_row_height(64.);
        self.view = Some(view);
    }

    /// The number of todos we currently have.
    fn number_of_items(&self) -> usize {
        self.todos.len()
    }

    /// For a given row, dequeues a view from the system and passes the appropriate `Transfer` for
    /// configuration.
    fn item_for(&self, row: usize) -> ListViewRow {
        let mut view = self.view.as_ref().unwrap().dequeue::<TodoViewRow>(TODO_ROW);

        if let Some(view) = &mut view.delegate {
            self.todos.with(row, |todo| view.configure_with(todo));
        }

        view.into_row()
    }

    /// Provides support for _swipe-to-reveal_ actions. After a user has completed one of these
    /// actions, we make sure to mark the tableview as done (see the message handlers in this
    /// file).
    fn actions_for(&self, row: usize, edge: RowEdge) -> Vec<RowAction> {
        if let RowEdge::Leading = edge {
            return vec![];
        }

        let mut actions = vec![];

        self.todos.with(row, |todo| match todo.status {
            TodoStatus::Complete => {
                actions.push(RowAction::new(
                    "Mark Incomplete",
                    RowActionStyle::Destructive,
                    move |_action, row| {
                        dispatch_ui(Message::MarkTodoIncomplete(row));
                    },
                ));
            },

            TodoStatus::Incomplete => {
                actions.push(RowAction::new(
                    "Mark Complete",
                    RowActionStyle::Regular,
                    move |_action, row| {
                        dispatch_ui(Message::MarkTodoComplete(row));
                    },
                ));
            },
        });

        actions
    }
}
