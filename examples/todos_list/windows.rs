//! We use a few different windows in our app lifecycle, so it's easier to
//! just use a small abstraction here and keep the app delegate clean.
//!
//! This could be a lot cleaner, and is something I'd like to make cleaner on a framework level.

use std::sync::RwLock;

use cacao::macos::window::{Window, WindowConfig, WindowStyle, WindowDelegate, WindowToolbarStyle};
use cacao::notification_center::Dispatcher;

use crate::storage::Message;

use crate::add::AddNewTodoWindow;
use crate::todos::TodosWindow;
use crate::preferences::PreferencesWindow;

#[derive(Default)]
pub struct WindowManager {
    pub main: RwLock<Option<Window<TodosWindow>>>,
    pub preferences: RwLock<Option<Window<PreferencesWindow>>>,
    pub add: RwLock<Option<Window<AddNewTodoWindow>>>
}

/// A helper method to handle checking for window existence, and creating
/// it if not - then showing it.
fn open_or_show<T, F>(window: &RwLock<Option<Window<T>>>, vendor: F)
where
    T: WindowDelegate + 'static,
    F: Fn() -> (WindowConfig, T)
{
    let mut lock = window.write().unwrap();
    
    if let Some(win) = &*lock {
        win.show();
    } else {
        let (config, delegate) = vendor();
        let win = Window::with(config, delegate);
        win.show();
        *lock = Some(win);
    }
}

impl WindowManager {
    pub fn open_main(&self) {
        open_or_show(&self.main, || (
            WindowConfig::default(), TodosWindow::new()
        ));
    }

    /// When we run a sheet, we want to run it on our main window, which is all
    /// this helper is for.
    pub fn begin_sheet<W, F>(&self, window: &Window<W>, completion: F)
    where
        W: WindowDelegate + 'static,
        F: Fn() + Send + Sync + 'static
    {
        let main = self.main.write().unwrap();
        
        if let Some(main_window) = &*main {
            main_window.begin_sheet(window, completion);
        }
    }

    /// Opens a "add file" window, which asks for a code and optional server to
    /// check against. This should, probably, be a sheet - but for now it's fine as a
    /// separate window until I can find time to port that API.
    pub fn open_add(&self) {
        let callback = || {};

        let mut lock = self.add.write().unwrap();

        if let Some(win) = &*lock {
            self.begin_sheet(&win, callback);
        } else {
            let window = Window::with(WindowConfig::default(), AddNewTodoWindow::new());
            self.begin_sheet(&window, callback);
            *lock = Some(window);
        }
    }

    pub fn close_sheet(&self) {
        let mut add = self.add.write().unwrap();

        if let Some(add_window) = &*add {
            let main = self.main.write().unwrap();

            if let Some(main_window) = &*main {
                main_window.end_sheet(&add_window);
            }
        }

        *add = None;
    }

    /// Opens a "add file" window, which asks for a code and optional server to
    /// check against.
    pub fn open_preferences(&self) {
        open_or_show(&self.preferences, || {
            let mut config = WindowConfig::default();
            config.set_initial_dimensions(100., 100., 400., 400.);

            config.set_styles(&[
                WindowStyle::Resizable, WindowStyle::Miniaturizable,
                WindowStyle::Closable, WindowStyle::Titled
            ]);

            config.toolbar_style = WindowToolbarStyle::Preferences;

            (config, PreferencesWindow::new())
        });
    }
}

impl Dispatcher for WindowManager {
    type Message = Message;

    /// Some jank message passing, it's fine for now.
    fn on_ui_message(&self, message: Message) {
        match message {
            Message::OpenMainWindow => {
                self.open_main();
            },

            Message::OpenPreferencesWindow => {
                self.open_preferences();
            },

            Message::CloseSheet => {
                self.close_sheet();
            },

            Message::OpenNewTodoSheet => {
                self.open_add();
            },

            Message::StoreNewTodo(_) => {
                self.close_sheet();
            },

            _ => {}
        }

        if let Some(w) = &*(self.main.read().unwrap()) {
            if let Some(delegate) = &w.delegate {
                delegate.on_message(message.clone());
            }
        }

        if let Some(w) = &*(self.preferences.read().unwrap()) {
            if let Some(delegate) = &w.delegate {
                delegate.on_message(message.clone());
            }
        }

        if let Some(w) = &*(self.add.read().unwrap()) {
            if let Some(delegate) = &w.delegate {
                delegate.on_message(message.clone());
            }
        }
    }
}
