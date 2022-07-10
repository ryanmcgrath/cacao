//! Messages that we used to thread control throughout the application.
//! If you come from React/Redux, you can liken it to that world.

use cacao::appkit::App;

use crate::app::TodosApp;

mod defaults;
pub use defaults::Defaults;

mod todos;
pub use todos::{Todos, Todo, TodoStatus};

/// Message passing is our primary way of instructing UI changes without needing to do
/// constant crazy referencing in Rust. Dispatch a method using either `dispatch_ui` for the main
/// thread, or `dispatch` for a background thread, and the main `TodosApp` will receive the
/// message. From there, it can filter down to components, or just handle it as necessary.
#[derive(Clone, Debug)]
pub enum Message {
    /// (Re)Open the main window.
    OpenMainWindow,

    /// Open the Preferences window.
    OpenPreferencesWindow,

    /// Switch the Preferences panel to the General section.
    SwitchPreferencesToGeneralPane,

    /// Switch the Preferences panel to the Advanced section.
    SwitchPreferencesToAdvancedPane,

    /// Open a "add new todo" window, as a modal sheet.
    OpenNewTodoSheet,

    /// Close the current active sheet, usually the receive window.
    CloseSheet,

    /// Called to instruct the app to process a todo from the input box.
    ProcessNewTodo,

    /// Called when there's a new Todo to store.
    StoreNewTodo(String),

    /// Mark a todo as complete.
    MarkTodoComplete(usize),

    /// Mark a todo as incomplete.
    MarkTodoIncomplete(usize)
}

/// Dispatch a message on a background thread.
pub fn dispatch_ui(message: Message) {
    println!("Dispatching UI message: {:?}", message);
    App::<TodosApp, Message>::dispatch_main(message);
}
