//! This module contains some lightweight wrappers over certain data types that we use throughout
//! the framework. Some of it is pulled/inspired from Servo's cocoa-rs (e.g, the "id" type). While
//! this isn't a clone of their module (we don't need everything from there, but remaining
//! compatible in case an end-user wants to drop that low is deal), it's worth linking their
//! license and repository - they've done really incredible work and it's 100% worth acknowledging.
//!
//! - [core-foundation-rs Repository](https://github.com/servo/core-foundation-rs)
//! - [core-foundation-rs MIT License](https://github.com/servo/core-foundation-rs/blob/master/LICENSE-MIT)
//! - [core-foundation-rs Apache License](https://github.com/servo/core-foundation-rs/blob/master/LICENSE-APACHE)

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use objc::runtime;
pub use objc::runtime::{BOOL, NO, YES};

pub mod autoreleasepool;
pub use autoreleasepool::AutoReleasePool;

pub mod array;
pub use array::NSArray;

pub mod string;
pub use string::NSString;

pub mod dictionary;
pub use dictionary::NSDictionary;

#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;

#[allow(non_upper_case_globals)]
pub const nil: id = 0 as id;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;
