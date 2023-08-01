//! This example showcases how to use a `Popover`.
//! This requires multiple types:
//! - A Window with a Controller / View
//! - A Popover
//! - Another Controller / View

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::segmentedcontrol::SegmentedControl;
use cacao::appkit::window::{Window, WindowConfig, WindowController, WindowDelegate};
use cacao::appkit::{App, AppDelegate};
use cacao::button::Button;
use cacao::foundation::NSArray;
use cacao::geometry::{Edge, Rect};
use cacao::image::Image;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::notification_center::Dispatcher;
use cacao::view::{Popover, PopoverConfig, View, ViewController, ViewDelegate};

struct BasicApp {
    window: WindowController<MyWindow>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new("", vec![
                MenuItem::Services,
                MenuItem::Separator,
                MenuItem::Hide,
                MenuItem::HideOthers,
                MenuItem::ShowAll,
                MenuItem::Separator,
                MenuItem::Quit,
            ]),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new("Window", vec![
                MenuItem::Minimize,
                MenuItem::Zoom,
                MenuItem::Separator,
                MenuItem::new("Bring All to Front"),
            ]),
        ]);

        App::activate();

        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct MyWindow {
    controller: Option<ViewController<PopoverExampleContentView>>
}

impl WindowDelegate for MyWindow {
    const NAME: &'static str = "MyWindow";

    fn did_load(&mut self, window: Window) {
        window.set_minimum_content_size(400., 400.);
        window.set_title("A Basic Window!?");
        let view = PopoverExampleContentView::new();
        let controller = ViewController::new(view);
        window.set_content_view_controller(&controller);
        self.controller = Some(controller);
    }

    fn will_close(&self) {
        println!("Closing now!");
    }
}

impl MyWindow {
    pub fn on_message(&self, message: Msg) {
        if let Some(delegate) = self.controller.as_ref().map(|e| &e.view).and_then(|v| v.delegate.as_ref()) {
            delegate.on_message(message);
        }
    }
}

fn main() {
    App::new("com.test.window-delegate", BasicApp {
        window: WindowController::with(WindowConfig::default(), MyWindow::default())
    })
    .run();
}

#[derive(Clone, Debug)]
pub enum Msg {
    Click
}

#[derive(Debug, Default)]
struct PopoverExampleContentView {
    view: Option<View>,
    button: Option<Button>,
    popover: Option<Popover<PopoverExampleContentViewController>>
}

impl PopoverExampleContentView {
    pub fn new() -> Self {
        Self {
            view: None,
            button: None,
            popover: None
        }
    }

    pub fn on_message(&self, message: Msg) {
        match message {
            Msg::Click => {
                let Some(ref popover) = self.popover else { return };
                let Some(ref button) = self.button else { return };
                popover.show_popover(Rect::zero(), button, Edge::MaxY);
            }
        }
    }
}

impl ViewDelegate for PopoverExampleContentView {
    const NAME: &'static str = "PopoverExampleContentView";

    fn did_load(&mut self, view: cacao::view::View) {
        let mut button = Button::new("Show");
        button.set_action(|_| dispatch_ui(Msg::Click));

        let controller = PopoverExampleContentViewController::new();
        let config = PopoverConfig {
            animates: false,
            ..Default::default()
        };
        let popover = Popover::new(controller, config);
        self.popover = Some(popover);

        view.add_subview(&button);

        LayoutConstraint::activate(&[
            button.center_x.constraint_equal_to(&view.center_x),
            button.center_y.constraint_equal_to(&view.center_y)
        ]);

        self.view = Some(view);
        self.button = Some(button);
    }
}

pub fn dispatch_ui(message: Msg) {
    println!("Dispatching UI message: {:?}", message);
    App::<BasicApp, Msg>::dispatch_main(message);
}

impl Dispatcher for BasicApp {
    type Message = Msg;

    // Handles a message that came over on the main (UI) thread.
    fn on_ui_message(&self, message: Self::Message) {
        if let Some(d) = &self.window.window.delegate {
            d.on_message(message)
        }
    }
}

#[derive(Debug)]
struct PopoverExampleContentViewController {
    pub control: SegmentedControl
}

impl PopoverExampleContentViewController {
    fn new() -> Self {
        let images = NSArray::from(vec![
            &*Image::symbol(cacao::image::SFSymbol::AtSymbol, "Hello").0,
            &*Image::symbol(cacao::image::SFSymbol::PaperPlane, "Hello").0,
            &*Image::symbol(cacao::image::SFSymbol::PaperPlaneFilled, "Hello").0,
        ]);
        let mut control = SegmentedControl::new(images, cacao::appkit::segmentedcontrol::TrackingMode::SelectOne);
        control.set_action(|index| {
            println!("Selected Index {index}");
        });
        Self { control }
    }
}

impl ViewDelegate for PopoverExampleContentViewController {
    const NAME: &'static str = "PopoverExampleContentViewController";

    fn did_load(&mut self, view: View) {
        view.add_subview(&self.control);
    }
}
