use cacao::layout::{LayoutConstraint, Layout};
use cacao::button::Button;
use cacao::color::Color;
use cacao::view::View;

use crate::calculator::Msg;
use crate::content_view::{button, BUTTON_WIDTH, BUTTON_HEIGHT};

pub struct ButtonRow {
    pub view: View,
    pub buttons: Vec<Button>
}

impl ButtonRow {
    pub fn new(x: [Msg; 4], color: Color, action_color: Color) -> Self {
        let view = View::new();

        let buttons: Vec<Button> = x.iter().map(|y| {
            let button = button(match y {
                Msg::Clear => "C",
                Msg::Add => "+",
                Msg::Subtract => "-",
                Msg::Multiply => "X",
                Msg::Divide => "/",
                Msg::Invert => "+/-",
                Msg::Mod => "%",
                Msg::Push(i) if *i == 1 => "1",
                Msg::Push(i) if *i == 2 => "2",
                Msg::Push(i) if *i == 3 => "3",
                Msg::Push(i) if *i == 4 => "4",
                Msg::Push(i) if *i == 5 => "5",
                Msg::Push(i) if *i == 6 => "6",
                Msg::Push(i) if *i == 7 => "7",
                Msg::Push(i) if *i == 8 => "8",
                Msg::Push(i) if *i == 9 => "9",
                _ => "W"

            }, y.clone());
            
            view.add_subview(&button);
            button
        }).collect();

        buttons[0].set_background_color(color.clone());
        buttons[1].set_background_color(color.clone());
        buttons[2].set_background_color(color);
        buttons[3].set_background_color(action_color);

        let width = &buttons[0].width;

        LayoutConstraint::activate(&[
            buttons[0].top.constraint_equal_to(&view.top),
            buttons[0].leading.constraint_equal_to(&view.leading),
            buttons[0].bottom.constraint_equal_to(&view.bottom),
            width.constraint_equal_to_constant(BUTTON_WIDTH),

            buttons[1].top.constraint_equal_to(&view.top),
            buttons[1].leading.constraint_equal_to(&buttons[0].trailing).offset(1.),
            buttons[1].bottom.constraint_equal_to(&view.bottom),
            buttons[1].width.constraint_equal_to(&width),

            buttons[2].top.constraint_equal_to(&view.top),
            buttons[2].leading.constraint_equal_to(&buttons[1].trailing).offset(1.),
            buttons[2].bottom.constraint_equal_to(&view.bottom),
            buttons[2].width.constraint_equal_to(&width),

            buttons[3].top.constraint_equal_to(&view.top),
            buttons[3].leading.constraint_equal_to(&buttons[2].trailing).offset(1.),
            buttons[3].trailing.constraint_equal_to(&view.trailing),
            buttons[3].bottom.constraint_equal_to(&view.bottom),
            buttons[3].width.constraint_equal_to(&width),
            
            view.height.constraint_equal_to_constant(BUTTON_HEIGHT)
        ]);

        Self {
            view,
            buttons
        }
    }
}

