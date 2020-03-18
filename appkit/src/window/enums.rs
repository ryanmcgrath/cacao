//! Enums used in Window construction and handling.

pub enum WindowTitleVisibility {
    Visible,
    Hidden
}

impl From<WindowTitleVisibility> for usize {
    fn from(visibility: WindowTitleVisibility) -> usize {
        match visibility {
            WindowTitleVisibility::Visible => 0,
            WindowTitleVisibility::Hidden => 1
        }
    }
}
