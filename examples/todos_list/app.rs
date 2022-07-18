//! Implements the start of the App lifecycle. Handles creating the required menu and window
//! components and message dispatching.

use cacao::appkit::{App, AppDelegate};
use cacao::notification_center::Dispatcher;

use crate::menu::menu;
use crate::storage::{Defaults, Message};
use crate::windows::WindowManager;

/// This handles routing lifecycle events, and maintains our `WindowManager`.
#[derive(Default)]
pub struct TodosApp {
    pub window_manager: WindowManager
}

impl AppDelegate for TodosApp {
    /// Sets the menu, activates the app, opens the main window and requests notification
    /// permissions.
    fn did_finish_launching(&self) {
        Defaults::register();

        App::set_menu(menu());
        App::activate();
        
        self.window_manager.open_main();
    }
}

impl Dispatcher for TodosApp {
    type Message = Message;

    /// Handles a message that came over on the main (UI) thread.
    fn on_ui_message(&self, message: Self::Message) {
        self.window_manager.on_ui_message(message);
    }
}
