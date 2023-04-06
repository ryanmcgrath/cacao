use cacao::appkit::FocusRingType;
use cacao::button::{BezelStyle, Button};
use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::{Font, Label, TextAlign};
use cacao::view::{View, ViewDelegate};

use crate::button_row::ButtonRow;
use crate::calculator::{dispatch, Msg};

pub const BUTTON_WIDTH: f64 = 57.;
pub const BUTTON_HEIGHT: f64 = 47.;

pub fn button(text: &str, msg: Msg) -> Button {
    let mut button = Button::new(text);
    button.set_bordered(false);
    button.set_bezel_style(BezelStyle::SmallSquare);
    button.set_focus_ring_type(FocusRingType::None);
    button.set_action(move |_| dispatch(msg.clone()));
    button.set_key_equivalent(&*text.to_lowercase());

    let font = Font::system(22.);
    button.set_font(&font);
    button.set_text_color(Color::SystemWhite);

    button
}

pub struct CalculatorView {
    pub results_wrapper: View,
    pub label: Label,
    pub row0: ButtonRow,
    pub row1: ButtonRow,
    pub row2: ButtonRow,
    pub row3: ButtonRow,
    pub dot: Button,
    pub zero: Button,
    pub equals: Button,
}

impl CalculatorView {
    pub fn new() -> Self {
        let results_wrapper = View::new();

        let label = Label::new();
        let font = Font::system(40.);
        label.set_font(&font);
        label.set_text("0");
        label.set_text_color(Color::rgb(255, 255, 255));
        label.set_text_alignment(TextAlign::Right);

        Self {
            results_wrapper,
            label,

            row0: ButtonRow::new(
                [Msg::Clear, Msg::Invert, Msg::Mod, Msg::Divide],
                Color::rgb(69, 69, 69),
                Color::rgb(255, 148, 10),
            ),

            row1: ButtonRow::new(
                [Msg::Push(7), Msg::Push(8), Msg::Push(9), Msg::Multiply],
                Color::rgb(100, 100, 100),
                Color::rgb(255, 148, 10),
            ),

            row2: ButtonRow::new(
                [Msg::Push(4), Msg::Push(5), Msg::Push(6), Msg::Subtract],
                Color::rgb(100, 100, 100),
                Color::rgb(255, 148, 10),
            ),

            row3: ButtonRow::new(
                [Msg::Push(1), Msg::Push(2), Msg::Push(3), Msg::Add],
                Color::rgb(100, 100, 100),
                Color::rgb(255, 148, 10),
            ),

            zero: button("0", Msg::Push(0)),
            dot: button(".", Msg::Decimal),
            equals: button("=", Msg::Equals),
        }
    }

    pub fn render_update(&self, message: String) {
        self.label.set_text(&message);
    }
}

impl ViewDelegate for CalculatorView {
    const NAME: &'static str = "CalculatorView";

    fn did_load(&mut self, view: View) {
        view.set_background_color(Color::rgb(49, 49, 49));
        self.zero.set_background_color(Color::rgb(100, 100, 100));
        self.dot.set_background_color(Color::rgb(100, 100, 100));
        self.equals.set_background_color(Color::rgb(255, 148, 10));

        self.zero.set_key_equivalent("0");

        view.add_subview(&self.row0.view);
        view.add_subview(&self.row1.view);
        view.add_subview(&self.row2.view);
        view.add_subview(&self.row3.view);

        for button in &[&self.zero, &self.dot, &self.equals] {
            view.add_subview(button);
        }

        self.results_wrapper.add_subview(&self.label);
        view.add_subview(&self.results_wrapper);

        LayoutConstraint::activate(&[
            self.results_wrapper.top.constraint_equal_to(&view.top),
            self.results_wrapper.leading.constraint_equal_to(&view.leading),
            self.results_wrapper.trailing.constraint_equal_to(&view.trailing),
            self.results_wrapper.height.constraint_equal_to_constant(80.),
            self.label
                .leading
                .constraint_equal_to(&self.results_wrapper.leading)
                .offset(22.),
            self.label
                .trailing
                .constraint_equal_to(&self.results_wrapper.trailing)
                .offset(-16.),
            self.label
                .bottom
                .constraint_equal_to(&self.results_wrapper.bottom)
                .offset(-4.),
            // Buttons laid out from top-left
            self.row0
                .view
                .top
                .constraint_equal_to(&self.results_wrapper.bottom)
                .offset(1.),
            self.row0.view.leading.constraint_equal_to(&view.leading),
            self.row0.view.trailing.constraint_equal_to(&view.trailing),
            self.row1.view.top.constraint_equal_to(&self.row0.view.bottom).offset(1.),
            self.row1.view.leading.constraint_equal_to(&view.leading),
            self.row1.view.trailing.constraint_equal_to(&view.trailing),
            self.row2.view.top.constraint_equal_to(&self.row1.view.bottom).offset(1.),
            self.row2.view.leading.constraint_equal_to(&view.leading),
            self.row2.view.trailing.constraint_equal_to(&view.trailing),
            self.row3.view.top.constraint_equal_to(&self.row2.view.bottom).offset(1.),
            self.row3.view.leading.constraint_equal_to(&view.leading),
            self.row3.view.trailing.constraint_equal_to(&view.trailing),
            self.zero.top.constraint_equal_to(&self.row3.view.bottom).offset(1.),
            self.zero.leading.constraint_equal_to(&view.leading),
            self.zero.bottom.constraint_equal_to(&view.bottom),
            self.dot.top.constraint_equal_to(&self.row3.view.bottom).offset(1.),
            self.dot.leading.constraint_equal_to(&self.zero.trailing).offset(1.),
            self.dot.bottom.constraint_equal_to(&view.bottom),
            self.dot.width.constraint_equal_to_constant(BUTTON_WIDTH),
            self.dot.height.constraint_equal_to_constant(BUTTON_HEIGHT),
            self.equals.top.constraint_equal_to(&self.row3.view.bottom).offset(1.),
            self.equals.leading.constraint_equal_to(&self.dot.trailing).offset(1.),
            self.equals.trailing.constraint_equal_to(&view.trailing),
            self.equals.bottom.constraint_equal_to(&view.bottom),
            self.equals.width.constraint_equal_to_constant(BUTTON_WIDTH),
            self.equals.height.constraint_equal_to_constant(BUTTON_HEIGHT),
        ])
    }
}
