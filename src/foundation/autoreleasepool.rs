use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::Id;

/// A wrapper around `NSAutoReleasePool`. The core `App` structures create and manage one of these,
/// but it's conceivable that users might need to create their own.
///
/// When this is dropped, we automatically send a `drain` message to the underlying pool. You can
/// also call `drain()` yourself if you need to drain for whatever reason.
pub struct AutoReleasePool(pub Id<Object>);

impl AutoReleasePool {
    /// Creates and returns a new `AutoReleasePool`. You need to take care to keep this alive for
    /// as long as you need it.
    pub fn new() -> Self {
        AutoReleasePool(unsafe {
            Id::from_retained_ptr(msg_send![class!(NSAutoreleasePool), new])
        })
    }

    /// Drains the underlying AutoreleasePool.
    pub fn drain(&self) {
        let _: () = unsafe { msg_send![&*self.0, drain] };
    }

    /// Run a function with a one-off AutoReleasePool.
    ///
    /// This will create a custom NSAutoreleasePool, run your handler, and automatically drain
    /// when done. This is (roughly, ish) equivalent to `@autoreleasepool {}` under ARC. If you
    /// need to perform Cocoa calls on a different thread, it's important to ensure they're backed
    /// with an autorelease pool - otherwise your memory footprint will continue to grow.
    pub fn run<F>(handler: F)
    where
        F: Fn() + 'static
    {
        let _pool = AutoReleasePool::new();
        handler();
    }
}

impl Drop for AutoReleasePool {
    /// Drains the underlying NSAutoreleasePool.
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![&*self.0, drain] };
    }
}
