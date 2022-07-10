//! This implements a "database" for our Todos app. It makes the assumption that we're only ever
//! doing this stuff on the main thread; in a more complicated app, you'd probably make different
//! choices.

use std::rc::Rc;
use std::cell::RefCell;

/// The status of a Todo.
#[derive(Debug)]
pub enum TodoStatus {
    /// Yet to be completed.
    Incomplete,

    /// Completed. ;P
    Complete
}

/// A Todo. Represents... something to do.
#[derive(Debug)]
pub struct Todo {
    /// The title of this todo.
    pub title: String,

    /// The status of this todo.
    pub status: TodoStatus
}

/// A single-threaded Todos "database".
#[derive(Debug, Default)]
pub struct Todos(Rc<RefCell<Vec<Todo>>>);

impl Todos {
    /// Insert a new Todo.
    pub fn insert(&self, title: String) {
        let mut stack = self.0.borrow_mut();

        let mut todos = vec![Todo {
            title: title,
            status: TodoStatus::Incomplete
        }];

        todos.append(&mut stack);

        *stack = todos;
    }

    /// Edit a Todo at the row specified.
    pub fn with_mut<F>(&self, row: usize, handler: F)
    where
        F: Fn(&mut Todo)
    {
        let mut stack = self.0.borrow_mut();

        if let Some(todo) = stack.get_mut(row) {
            handler(todo);
        }
    }

    /// Run a block with the given Todo.
    pub fn with<F>(&self, row: usize, mut handler: F)
    where
        F: FnMut(&Todo)
    {
        let stack = self.0.borrow();

        if let Some(todo) = stack.get(row) {
            handler(todo);
        }
    }

    /// Returns the total number of Todos.
    pub fn len(&self) -> usize {
        let stack = self.0.borrow();
        stack.len()
    }
}
