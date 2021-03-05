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
            MenuItem::About("Todos".to_string()),
            MenuItem::Separator,
            
            MenuItem::new("Preferences").key(",").action(|| {
                dispatch_ui(Message::OpenPreferencesWindow);
            }),
            
            MenuItem::Separator,
            MenuItem::Services,
            MenuItem::Separator,
            MenuItem::Hide,
            MenuItem::HideOthers,
            MenuItem::ShowAll,
            MenuItem::Separator,
            MenuItem::Quit
        ]),

        Menu::new("File", vec![
            MenuItem::new("Open/Show Window").key("n").action(|| {
                dispatch_ui(Message::OpenMainWindow);
            }),

            MenuItem::Separator,
            
            MenuItem::new("Add Todo").key("+").action(|| {
                dispatch_ui(Message::OpenNewTodoSheet);
            }),

            MenuItem::Separator,
            MenuItem::CloseWindow
        ]),

        Menu::new("Edit", vec![
            MenuItem::Undo,
            MenuItem::Redo,
            MenuItem::Separator,
            MenuItem::Cut,
            MenuItem::Copy,
            MenuItem::Paste,
            MenuItem::Separator,
            MenuItem::SelectAll
        ]),
        
        Menu::new("View", vec![
            MenuItem::EnterFullScreen
        ]),

        Menu::new("Window", vec![
            MenuItem::Minimize,
            MenuItem::Zoom,
            MenuItem::Separator,
            MenuItem::new("Bring All to Front")
        ]),

        Menu::new("Help", vec![])
    ]
}
