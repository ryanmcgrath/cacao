//! Implements a top level menu, with some examples of how to configure and dispatch events.
//!
//! Some of this might move in to the framework at some point, but it requires a bit more thought.
//! Correctly functioning menus are a key part of what makes a macOS app feel right, though, so
//! this is here for those who might want to use this todos example as a starting point.

use cacao::macos::menu::{Menu, MenuItem};

use crate::storage::{dispatch_ui, Message};

/// Installs the menu.
pub fn menu() -> Vec<Menu> {
    vec![
        Menu::new("", vec![
            MenuItem::about("Todos"),
            MenuItem::Separator,
            
            MenuItem::entry("Preferences").key(",").action(|| {
                dispatch_ui(Message::OpenPreferencesWindow);
            }),
            
            MenuItem::Separator,
            MenuItem::services(),
            MenuItem::Separator,
            MenuItem::hide(),
            MenuItem::hide_others(),
            MenuItem::show_all(),
            MenuItem::Separator,
            MenuItem::quit()
        ]),

        Menu::new("File", vec![
            MenuItem::entry("Open/Show Window").key("n").action(|| {
                dispatch_ui(Message::OpenMainWindow);
            }),

            MenuItem::Separator,
            
            MenuItem::entry("Add Todo").key("+").action(|| {
                dispatch_ui(Message::OpenNewTodoSheet);
            }),

            MenuItem::Separator,
            MenuItem::close_window(),
        ]),

        Menu::new("Edit", vec![
            MenuItem::undo(),
            MenuItem::redo(),
            MenuItem::Separator,
            MenuItem::cut(),
            MenuItem::copy(),
            MenuItem::paste(),
            MenuItem::Separator,
            MenuItem::select_all()
        ]),
        
        Menu::new("View", vec![
            MenuItem::enter_full_screen()
        ]),

        Menu::new("Window", vec![
            MenuItem::minimize(),
            MenuItem::zoom(),
            MenuItem::Separator,
            MenuItem::entry("Bring All to Front")
        ]),

        Menu::new("Help", vec![])
    ]
}
