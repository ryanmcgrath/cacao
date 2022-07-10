//! Implements a stock-ish Preferences window.

use cacao::appkit::window::{Window, WindowDelegate};
use cacao::appkit::toolbar::Toolbar;
use cacao::view::ViewController;

use crate::storage::Message;

mod toolbar;
use toolbar::PreferencesToolbar;

mod general;
use general::GeneralPreferencesContentView;

mod advanced;
use advanced::AdvancedPreferencesContentView;

mod toggle_option_view;

pub struct PreferencesWindow {
    pub toolbar: Toolbar<PreferencesToolbar>,
    pub general: ViewController<GeneralPreferencesContentView>,
    pub advanced: ViewController<AdvancedPreferencesContentView>,
    window: Option<Window>
}

impl PreferencesWindow {
    pub fn new() -> Self {
        PreferencesWindow {
            toolbar: Toolbar::new("PreferencesToolbar", PreferencesToolbar::default()),
            general: ViewController::new(GeneralPreferencesContentView::default()),
            advanced: ViewController::new(AdvancedPreferencesContentView::default()),
            window: None
        }
    }

    pub fn on_message(&self, message: Message) {
        let window = self.window.as_ref().unwrap();

        match message {
            Message::SwitchPreferencesToGeneralPane => {
                window.set_title("General");
                window.set_content_view_controller(&self.general);
            },

            Message::SwitchPreferencesToAdvancedPane => {
                window.set_title("Advanced");
                window.set_content_view_controller(&self.advanced);
            },

            _ => {}
        }
    }
}

impl WindowDelegate for PreferencesWindow {
    const NAME: &'static str = "PreferencesWindow";

    fn did_load(&mut self, window: Window) {
        window.set_autosave_name("TodosPreferencesWindow");
        window.set_movable_by_background(true);
        window.set_toolbar(&self.toolbar);

        self.window = Some(window);

        self.on_message(Message::SwitchPreferencesToGeneralPane);
    }
}
