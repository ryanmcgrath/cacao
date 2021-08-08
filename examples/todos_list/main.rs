//! This example implements a "kitchen sink" Todo app, because I suppose that's what all GUI
//! frameworks aspire to do these days. Go figure.
//!
//! This may get extracted into a different repo some day in the future.

use cacao::appkit::App;

mod add;
mod app;
mod menu;
mod preferences;
mod storage;
mod todos;
mod windows;

fn main() {
    App::new(
        "com.cacao.todo",
        app::TodosApp::default()
    ).run();
}
