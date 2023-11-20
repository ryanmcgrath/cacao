//! This example showcases setting up a basic application and view with safe area constraints.

use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::{Font, Label};
use cacao::view::{View, ViewDelegate};

struct BasicApp {
    window: Window,
    content_view: View<ContentView>,
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
    label: Label,
    label2: Label,
    label3: Label,
}

impl ViewDelegate for ContentView {
    const NAME: &'static str = "SafeAreaView";

    fn did_load(&mut self, view: View) {
        let color_black = cacao::color::Color::rgb(255, 255, 255);

        let font = Font::system(30.);
        self.label.set_font(&font);
        self.label.set_text("Hello World");
        self.label.set_text_color(&color_black);

        let times_new_roman = Font::with_name("Times New Roman", 30.);
        self.label2.set_font(&times_new_roman);
        self.label2.set_text("Hello World (In 'Times New Roman')");
        self.label2.set_text_color(&color_black);

        let helvetica = Font::with_name("Helvetica", 30.);
        self.label3.set_font(&helvetica);
        self.label3.set_text("Hello World (In 'Helvetica')");
        self.label3.set_text_color(&color_black);

        self.content.add_subview(&self.label);
        self.content.add_subview(&self.label2);
        self.content.add_subview(&self.label3);
        view.add_subview(&self.content);

        // layouts for labels
        cacao::layout::LayoutConstraint::activate(&[
            self.label2.top.constraint_equal_to(&self.label.bottom).offset(2.),
            self.label3.top.constraint_equal_to(&self.label2.bottom).offset(2.),
        ]);

        // Add layout constraints to be 100% excluding the safe area
        // Do last because it will crash because the view needs to be inside the hierarchy
        cacao::layout::LayoutConstraint::activate(&[
            self.content.top.constraint_equal_to(&view.safe_layout_guide.top),
            self.content.leading.constraint_equal_to(&view.safe_layout_guide.leading),
            self.content.trailing.constraint_equal_to(&view.safe_layout_guide.trailing),
            self.content.bottom.constraint_equal_to(&view.safe_layout_guide.bottom),
        ])
    }
}

fn main() {
    App::new(
        "com.test.window",
        BasicApp {
            window: Window::default(),
            content_view: View::with(ContentView::default()),
        },
    )
    .run();
}
