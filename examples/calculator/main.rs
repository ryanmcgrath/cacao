//! This example implements a basic macOS Calculator clone. It showcases:
//!
//!     - A single-window app
//!     - Button handling
//!     - Autolayout
//!     - Message dispatching
//!     - Global key/event handling
//!
//! It does not attempt to be a good calculator, and does not implement the
//! extended Calculator view.

use std::sync::RwLock;

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::{Window, WindowConfig, TitleVisibility};
use cacao::appkit::{Event, EventMask, EventMonitor};
use cacao::color::Color;
use cacao::notification_center::Dispatcher;
use cacao::view::View;

mod button_row;
mod calculator;

mod content_view;
use content_view::CalculatorView;

struct CalculatorApp {
    window: Window,
    content: View<CalculatorView>,
    key_monitor: RwLock<Option<EventMonitor>>
}

impl AppDelegate for CalculatorApp {
    fn did_finish_launching(&self) {
        App::activate();

        // Event Monitors need to be started after the App has been activated.
        // We use an RwLock here, but it's possible this entire method can be 
        // &mut self and you wouldn't need these kinds of shenanigans.
        //self.start_monitoring();

        self.window.set_title("Calculator");
        self.window.set_background_color(Color::rgb(49,49,49));
        self.window.set_title_visibility(TitleVisibility::Hidden);
        self.window.set_titlebar_appears_transparent(true);
        self.window.set_movable_by_background(true);
        self.window.set_autosave_name("CacaoCalculatorExampleWindow");
        self.window.set_content_view(&self.content);
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

impl Dispatcher for CalculatorApp {
    type Message = String;

    fn on_ui_message(&self, message: Self::Message) {
        if let Some(delegate) = &self.content.delegate {
            delegate.render_update(message);
        }
    }
}

impl CalculatorApp {
    /// Monitor for key presses, and dispatch if they match an action
    /// we're after.
    pub fn start_monitoring(&self) {
        let mut lock = self.key_monitor.write().unwrap();
        *lock = Some(Event::local_monitor(EventMask::KeyDown, |evt| {
            let characters = evt.characters();
            println!("{}", characters);

            //use calculator::{dispatch, Msg};
            /*match characters.as_ref() {
                "0" => dispatch(Msg::Push(0)),
                "1" => dispatch(Msg::Push(1)),
                "2" => dispatch(Msg::Push(2)),
                "3" => dispatch(Msg::Push(3)),
                "4" => dispatch(Msg::Push(4)),
                "5" => dispatch(Msg::Push(5)),
                "6" => dispatch(Msg::Push(6)),
                "7" => dispatch(Msg::Push(7)),
                "8" => dispatch(Msg::Push(8)),
                "9" => dispatch(Msg::Push(9)),
                "+" => dispatch(Msg::Add),
                "-" => dispatch(Msg::Subtract),
                "*" => dispatch(Msg::Multiply),
                "/" => dispatch(Msg::Divide),
                "=" => dispatch(Msg::Equals),
                "%" => dispatch(Msg::Mod),
                "c" => dispatch(Msg::Clear),
                "." => dispatch(Msg::Decimal),
                _ => {}
            }*/

            None
        }));
    }
}

fn main() {
    let mut config = WindowConfig::default();
    config.set_initial_dimensions(100., 100., 240., 300.);

    App::new("com.example.calculator", CalculatorApp {
        window: Window::new(config),
        content: View::with(CalculatorView::new()),
        key_monitor: RwLock::new(None)
    }).run();
}
