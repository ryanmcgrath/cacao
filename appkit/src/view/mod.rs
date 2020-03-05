
pub(crate) static VIEW_CONTROLLER_PTR: &str = "rstViewControllerPtr";

pub(crate) mod class;
pub(crate) mod controller;

pub mod traits;
pub use traits::*;

pub mod view;
pub use view::View;
