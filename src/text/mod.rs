//! The `text` module encompasses various widgets for rendering and interacting
//! with text.

mod attributed_string;
pub use attributed_string::AttributedString;

mod label;
pub use label::Label;

mod enums;
pub use enums::{LineBreakMode, TextAlign};

mod font;
pub use font::Font;
