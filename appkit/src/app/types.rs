//! Various types used at the AppController level.

use cocoa::foundation::NSUInteger;

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

/// Used for handling printing files. You return this in relevant `AppController` methods.
#[derive(Copy, Clone, Debug)]
pub enum PrintResponse {
    /// Printing was cancelled.
    Cancelled,

    /// Printing was a success.
    Success,

    /// Printing failed.
    Failure,

    /// For when the result of printing cannot be returned immediately (e.g, if printing causes a sheet to appear).
    /// If your method returns PrintResponse::ReplyLater it must always invoke `App::reply_to_open_or_print()` when the 
    /// entire print operation has been completed, successfully or not.
    ReplyLater
}

impl From<PrintResponse> for NSUInteger {
    fn from(response: PrintResponse) -> NSUInteger {
        match response {
            PrintResponse::Cancelled => 0,
            PrintResponse::Success => 1,
            PrintResponse::Failure => 3,
            PrintResponse::ReplyLater => 2
        }
    }
}
