//! This example showcases how to do custom drawing on an ImageView
//! with CoreGraphics. Feel free to modify it and play around!

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};

use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::View;

use cacao::image::{DrawConfig, Image, ImageView};

struct BasicApp {
    window: Window,
    content_view: View,
    image_view: ImageView,
    image: Image,
}

impl Default for BasicApp {
    fn default() -> Self {
        let config = DrawConfig {
            source: (100., 100.),
            target: (800., 800.),
            resize: cacao::image::ResizeBehavior::Stretch,
        };

        Self {
            window: Window::default(),
            content_view: View::new(),
            image_view: ImageView::new(),
            image: Image::draw(config, |_cg_rect, context| {
                context.move_to_point(11.25, 8.19);
                context.add_line_to_point(11.25, 5.);
                context.add_line_to_point(6.56, 5.);
                context.add_curve_to_point(6.25, 5., 6., 5.25, 6., 5.56);
                context.add_line_to_point(6., 16.44);
                context.add_curve_to_point(6., 16.75, 6.25, 17., 6.56, 17.);
                context.add_line_to_point(14.44, 17.);
                context.add_curve_to_point(14.75, 17., 15., 16.75, 15., 16.44);
                context.add_line_to_point(15., 8.75);
                context.add_line_to_point(11.81, 8.75);
                context.add_curve_to_point(11.5, 8.75, 11.25, 8.5, 11.25, 8.19);
                context.close_path();

                context.move_to_point(12.75, 13.72);
                context.add_curve_to_point(12.75, 13.87, 12.62, 14., 12.47, 14.);
                context.add_line_to_point(8.53, 14.);
                context.add_curve_to_point(8.38, 14., 8.25, 13.87, 8.25, 13.72);
                context.add_line_to_point(8.25, 13.53);
                context.add_curve_to_point(8.25, 13.38, 8.38, 13.25, 8.53, 13.25);
                context.add_line_to_point(12.47, 13.25);
                context.add_curve_to_point(12.62, 13.25, 12.75, 13.38, 12.75, 13.53);
                context.add_line_to_point(12.75, 13.72);
                context.close_path();

                context.move_to_point(12.75, 12.22);
                context.add_curve_to_point(12.75, 12.37, 12.62, 12.5, 12.47, 12.5);
                context.add_line_to_point(8.53, 12.5);
                context.add_curve_to_point(8.38, 12.5, 8.25, 12.37, 8.25, 12.22);
                context.add_line_to_point(8.25, 12.03);
                context.add_curve_to_point(8.25, 11.88, 8.38, 11.75, 8.53, 11.75);
                context.add_line_to_point(12.47, 11.75);
                context.add_curve_to_point(12.62, 11.75, 12.75, 11.88, 12.75, 12.03);
                context.add_line_to_point(12.75, 12.22);
                context.close_path();

                context.move_to_point(12.75, 10.53);
                context.add_line_to_point(12.75, 10.72);
                context.add_curve_to_point(12.75, 10.87, 12.62, 11., 12.47, 11.);
                context.add_line_to_point(8.53, 11.);
                context.add_curve_to_point(8.38, 11., 8.25, 10.87, 8.25, 10.72);
                context.add_line_to_point(8.25, 10.53);
                context.add_curve_to_point(8.25, 10.38, 8.38, 10.25, 8.53, 10.25);
                context.add_line_to_point(12.47, 10.25);
                context.add_curve_to_point(12.62, 10.25, 12.75, 10.38, 12.75, 10.53);
                context.close_path();

                context.move_to_point(15., 7.86);
                context.add_line_to_point(15., 8.);
                context.add_line_to_point(12., 8.);
                context.add_line_to_point(12., 5.);
                context.add_line_to_point(12.14, 5.);
                context.add_curve_to_point(12.29, 5., 12.44, 5.06, 12.54, 5.16);
                context.add_line_to_point(14.84, 7.46);
                context.add_curve_to_point(14.94, 7.57, 15., 7.71, 15., 7.86);
                context.close_path();

                context.set_rgb_fill_color(1., 1., 1., 1.);
                context.fill_path();

                true
            }),
        }
    }
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![Menu::new(
            "",
            vec![
                MenuItem::Services,
                MenuItem::Separator,
                MenuItem::Hide,
                MenuItem::HideOthers,
                MenuItem::ShowAll,
                MenuItem::Separator,
                MenuItem::Quit,
            ],
        )]);

        App::activate();
        self.window.set_title("Hello World!");

        self.image_view.set_background_color(Color::SystemBlue);
        self.image_view.set_image(&self.image);
        self.content_view.add_subview(&self.image_view);

        LayoutConstraint::activate(&[
            self.image_view.top.constraint_equal_to(&self.content_view.top),
            self.image_view.leading.constraint_equal_to(&self.content_view.leading),
            self.image_view.trailing.constraint_equal_to(&self.content_view.trailing),
            self.image_view.bottom.constraint_equal_to(&self.content_view.bottom),
        ]);

        self.window.set_content_view(&self.content_view);
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

fn main() {
    let app_delegate = BasicApp::default();
    let app = App::new("com.hello.world", app_delegate);
    app.run()
}
