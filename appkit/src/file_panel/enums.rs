//! Certain enums that are useful (response types, etc).

use cocoa::foundation::{NSInteger};

pub enum ModalResponse {
    Ok,
    Continue,
    Canceled,
    Stopped,
    Aborted,
    FirstButtonReturned,
    SecondButtonReturned,
    ThirdButtonReturned
}

impl From<NSInteger> for ModalResponse {
    fn from(i: NSInteger) -> Self {
        match i {
            1 => ModalResponse::Ok,
            0 => ModalResponse::Canceled,
            1000 => ModalResponse::FirstButtonReturned,
            1001 => ModalResponse::SecondButtonReturned,
            1002 => ModalResponse::ThirdButtonReturned,
            -1000 => ModalResponse::Stopped,
            -1001 => ModalResponse::Aborted,
            -1002 => ModalResponse::Continue,
            e => { panic!("Unknown NSModalResponse sent back! {}", e); }
        }
    }
}
