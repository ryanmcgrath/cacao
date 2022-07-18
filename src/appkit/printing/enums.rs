//! Enums used through the general printing flow.

use crate::foundation::NSUInteger;

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
