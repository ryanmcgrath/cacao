//! This example showcases setting up a basic application and view with safe area constraints.

use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::{Font, Label};
use cacao::view::{View, ViewDelegate};

struct BasicApp {
    window: Window,
    content_view: View<ContentView>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::activate();

        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Safe Area Demo");
        self.window.set_content_view(&self.content_view);
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct ContentView {
    content: View,
    label: Label
}

impl ViewDelegate for ContentView {
    const NAME: &'static str = "SafeAreaView";

    fn did_load(&mut self, view: View) {
        let font = Font::system(30.);
        self.label.set_font(&font);
        self.label.set_text("Hello World");
        self.label.set_text_color(cacao::color::Color::rgb(255, 255, 255));

        self.content.add_subview(&self.label);
        view.add_subview(&self.content);

        // Add layout constraints to be 100% excluding the safe area
        // Do last because it will crash because the view needs to be inside the hierarchy
        cacao::layout::LayoutConstraint::activate(&[
            self.content.top.constraint_equal_to(&view.safe_layout_guide.top),
            self.content.leading.constraint_equal_to(&view.safe_layout_guide.leading),
            self.content.trailing.constraint_equal_to(&view.safe_layout_guide.trailing),
            self.content.bottom.constraint_equal_to(&view.safe_layout_guide.bottom)
        ])
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::default(),
        content_view: View::with(ContentView::default())
    })
    .run();
}
