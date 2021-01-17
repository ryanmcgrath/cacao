pub trait Dispatcher {
    type Message: Send + Sync;

    fn on_ui_message(&self, _message: Self::Message) {}

    fn on_background_message(&self, _message: Self::Message) {}
}
