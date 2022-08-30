use std::sync::RwLock;

use cacao::input::{TextField, TextFieldDelegate};
use cacao::text::{Label, TextAlign};
use cacao::uikit::{App, AppDelegate, Scene, SceneConfig, SceneConnectionOptions, SceneSession, Window, WindowSceneDelegate};

use cacao::color::Color;
use cacao::image::{Image, ImageView};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::{View, ViewController, ViewDelegate};

#[derive(Default)]
struct TestApp;

impl AppDelegate for TestApp {
    fn config_for_scene_session(&self, session: SceneSession, _options: SceneConnectionOptions) -> SceneConfig {
        SceneConfig::new("Default Configuration", session.role())
    }
}
#[derive(Debug, Default)]
pub struct ConsoleLogger(String);

impl TextFieldDelegate for ConsoleLogger {
    const NAME: &'static str = "ConsoleLogger";

    fn text_should_begin_editing(&self, value: &str) -> bool {
        println!("{} should begin editing: {}", self.0, value);
        true
    }

    fn text_did_change(&self, value: &str) {
        println!("{} text did change to {}", self.0, value);
    }

    fn text_did_end_editing(&self, value: &str) {
        println!("{} did end editing: {}", self.0, value);
    }

    fn text_should_end_editing(&self, value: &str) -> bool {
        println!("{} should end editing: {}", self.0, value);
        true
    }
}

pub struct RootView {
    pub green: View,
    pub blue: View,
    pub label: Label,
    pub image: ImageView,
    pub input: TextField<ConsoleLogger>
}

impl Default for RootView {
    fn default() -> Self {
        RootView {
            green: View::new(),
            blue: View::new(),
            label: Label::new(),
            image: ImageView::new(),
            input: TextField::with(ConsoleLogger("input_1".to_string()))
        }
    }
}

impl ViewDelegate for RootView {
    const NAME: &'static str = "RootView";

    fn did_load(&mut self, view: View) {
        self.label.set_text("my label");
        self.label.set_text_color(Color::SystemWhite);
        self.label.set_background_color(Color::SystemRed);
        self.label.layer.set_corner_radius(16.);
        self.label.set_text_alignment(TextAlign::Center);

        view.add_subview(&self.label);

        self.green.set_background_color(Color::SystemGreen);
        view.add_subview(&self.green);

        self.blue.set_background_color(Color::SystemBlue);
        view.add_subview(&self.blue);

        let image_bytes = include_bytes!("../../test-data/favicon.ico");
        self.image = ImageView::new();
        self.image.set_image(&Image::with_data(image_bytes));
        view.add_subview(&self.image);

        self.input.set_text("my input box 1");
        view.add_subview(&self.input);

        LayoutConstraint::activate(&[
            self.label.leading.constraint_equal_to(&view.leading).offset(16.),
            self.label.top.constraint_equal_to(&view.top).offset(16.),
            self.label.height.constraint_equal_to_constant(100.),
            self.label.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            self.green.top.constraint_equal_to(&self.label.bottom).offset(16.),
            self.green.leading.constraint_equal_to(&view.leading).offset(16.),
            self.green.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            self.green.height.constraint_equal_to_constant(120.),
            self.input.center_x.constraint_equal_to(&self.green.center_x),
            self.input.center_y.constraint_equal_to(&self.green.center_y),
            self.blue.top.constraint_equal_to(&self.green.bottom).offset(16.),
            self.blue.leading.constraint_equal_to(&view.leading).offset(16.),
            self.blue.trailing.constraint_equal_to(&view.trailing).offset(-16.),
            self.blue.bottom.constraint_equal_to(&view.bottom).offset(-16.),
            self.image.center_x.constraint_equal_to(&self.blue.center_x),
            self.image.center_y.constraint_equal_to(&self.blue.center_y)
        ]);
    }
}

#[derive(Default)]
pub struct WindowScene {
    pub window: RwLock<Option<Window>>,
    pub root_view_controller: RwLock<Option<ViewController<RootView>>>
}

impl WindowSceneDelegate for WindowScene {
    fn will_connect(&self, scene: Scene, session: SceneSession, options: SceneConnectionOptions) {
        let bounds = scene.get_bounds();
        let mut window = Window::new(bounds);
        window.set_window_scene(scene);

        let root_view_controller = ViewController::new(RootView::default());
        window.set_root_view_controller(&root_view_controller);
        window.show();

        {
            let mut w = self.window.write().unwrap();
            *w = Some(window);

            let mut vc = self.root_view_controller.write().unwrap();
            *vc = Some(root_view_controller);
        }
    }
}

fn main() {
    App::new(TestApp::default(), || Box::new(WindowScene::default())).run();
}
