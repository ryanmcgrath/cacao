//! This example builds on the AutoLayout example, but adds in animation
//! via `AnimationContext`. Views and layout anchors have special proxy objects that can be cloned
//! into handlers, enabling basic animation support within `AnimationContext`.
//!
//! This one is a bit kludgier than some other examples, but the comments throughout this should
//! clarify why that is.

use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint, LayoutConstraintAnimatorProxy};
use cacao::view::{View, ViewAnimatorProxy};

use cacao::appkit::menu::Menu;
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{AnimationContext, App, AppDelegate};
use cacao::appkit::{Event, EventMask, EventMonitor};

struct BasicApp {
    window: Window<AppWindow>,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(Menu::standard());
        App::activate();

        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

/// This map is the four different animation frames that we display, per view type.
/// Why do we have this here?
///
/// Well, it's because there's no random number generator in the standard library, and I really
/// dislike when examples need crates attached to 'em.
///
/// The basic mapping logic is this: each entry is a view's frame(s), and each frame is an array
/// of:
///
/// [top, left, width, height, alpha]
///
/// We then treat each frame index as follows:
///
/// w: 0
/// a: 1
/// s: 2
/// d: 3
const ANIMATIONS: [[[f64; 5]; 4]; 3] = [
    // Blue
    [
        [44., 16., 100., 100., 1.],
        [128., 84., 144., 124., 1.],
        [32., 32., 44., 44., 0.7],
        [328., 157., 200., 200., 0.7],
    ],
    // Red
    [
        [44., 132., 100., 100., 1.],
        [40., 47., 80., 64., 0.7],
        [84., 220., 600., 109., 1.0],
        [48., 600., 340., 44., 0.7],
    ],
    // Green
    [
        [44., 248., 100., 100., 1.],
        [420., 232., 420., 244., 0.7],
        [310., 440., 150., 238., 0.7],
        [32., 32., 44., 44., 1.],
    ],
];

/// A helper method for generating frame constraints that we want to be animating.
fn apply_styles(view: &View, parent: &View, background_color: Color, animation_table_index: usize) -> [LayoutConstraint; 4] {
    view.set_background_color(background_color);
    view.layer.set_corner_radius(16.);
    parent.add_subview(view);

    let animation = ANIMATIONS[animation_table_index][0];

    [
        view.top.constraint_equal_to(&parent.top).offset(animation[0]),
        view.left.constraint_equal_to(&parent.left).offset(animation[1]),
        view.width.constraint_equal_to_constant(animation[2]),
        view.height.constraint_equal_to_constant(animation[3]),
    ]
}

#[derive(Default)]
struct AppWindow {
    content: View,
    blue: View,
    red: View,
    green: View,
    key_monitor: Option<EventMonitor>,
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Animation Example (Use W/A/S/D to change state!)");
        window.set_minimum_content_size(300., 300.);

        window.set_content_view(&self.content);

        let blue_frame = apply_styles(&self.blue, &self.content, Color::SystemBlue, 0);
        let red_frame = apply_styles(&self.red, &self.content, Color::SystemRed, 1);
        let green_frame = apply_styles(&self.green, &self.content, Color::SystemGreen, 2);

        let alpha_animators = [&self.blue, &self.red, &self.green]
            .iter()
            .map(|view| view.animator.clone())
            .collect::<Vec<ViewAnimatorProxy>>();

        let constraint_animators = [blue_frame, red_frame, green_frame]
            .iter()
            .map(|frame| {
                LayoutConstraint::activate(frame);

                vec![
                    frame[0].animator.clone(),
                    frame[1].animator.clone(),
                    frame[2].animator.clone(),
                    frame[3].animator.clone(),
                ]
            })
            .collect::<Vec<Vec<LayoutConstraintAnimatorProxy>>>();

        // Monitor key change events for w/a/s/d, and then animate each view to their correct
        // frame and alpha value.
        self.key_monitor = Some(Event::local_monitor(EventMask::KeyDown, move |evt| {
            let characters = evt.characters();

            let animation_index = match characters.as_ref() {
                "w" => 0,
                "a" => 1,
                "s" => 2,
                "d" => 3,
                _ => 4,
            };

            if animation_index == 4 {
                return None;
            }

            let alpha_animators = alpha_animators.clone();
            let constraint_animators = constraint_animators.clone();

            AnimationContext::run(move |_ctx| {
                alpha_animators.iter().enumerate().for_each(move |(index, view)| {
                    let animation = ANIMATIONS[index][animation_index];
                    view.set_alpha(animation[4]);
                });

                constraint_animators.iter().enumerate().for_each(move |(index, frame)| {
                    let animation = ANIMATIONS[index][animation_index];
                    frame[0].set_offset(animation[0]);
                    frame[1].set_offset(animation[1]);
                    frame[2].set_offset(animation[2]);
                    frame[3].set_offset(animation[3]);
                });
            });

            None
        }));
    }
}

fn main() {
    App::new(
        "com.test.window",
        BasicApp {
            window: Window::with(WindowConfig::default(), AppWindow::default()),
        },
    )
    .run();
}
