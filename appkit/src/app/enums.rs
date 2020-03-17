//! Various types used at the AppController level.

use crate::foundation::NSUInteger;

/// Used for determining how an application should handle quitting/terminating.
/// You return this in your `AppController` `should_terminate` method.
#[derive(Copy, Clone, Debug)]
pub enum TerminateResponse {
    /// Proceed with termination.
    Now,

    /// App should not be terminated.
    Cancel,

    /// It might be fine to proceed with termination later. Returning this value causes 
    /// Cocoa to run the run loop until `should_terminate()` returns `true` or `false`.
    ///
    /// This return value is for primarily for cases where you need to provide alerts 
    /// in order to decide whether to quit.
    Later
}

impl From<TerminateResponse> for NSUInteger {
    fn from(response: TerminateResponse) -> NSUInteger {
        match response {
            TerminateResponse::Now => 1,
            TerminateResponse::Cancel => 0,
            TerminateResponse::Later => 2
        }
    }
}
