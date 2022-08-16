/// A trait for handling dispatched messages on the AppDelegate.
///
/// You can use this for a jank message dispatching mechanism. It has no guarantees concerning
/// performance, but is good enough for many applications. Implement this trait on your struct
/// that implements `AppDelegate`, and then dispatch messages like the following:
///
/// ```rust,compile_fail
/// App::<YourAppDelegate, YourMessageType>::dispatch_main(your_message);
/// ```
///
/// This will asynchronously loop a message back to the "top" of your app, via your app delegate.
/// You can process it from there.
pub trait Dispatcher {
    /// The type of Message you're sending. This should be lightweight and thread safe.
    type Message: Send + Sync;

    /// Called when a message is looped back on the _main_ queue. This is where all UI work should
    /// be happening.
    fn on_ui_message(&self, _message: Self::Message) {}

    /// Called when a message is looped back on a background queue.
    fn on_background_message(&self, _message: Self::Message) {}
}
