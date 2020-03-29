
/// Controllers interested in processing messages can implement this to respond to messages as
/// they're dispatched. All messages come in on the main thread.
pub trait Dispatcher {
    type Message: Send + Sync;

    fn on_message(&self, _message: Self::Message) {}
}
