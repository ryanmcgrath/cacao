//! This module contains some lightweight wrappers over Foundation data types.
//!
//! Some of it is pulled/inspired from Servo's cocoa-rs (e.g, the "id" type). While
//! this isn't a clone of their module (we don't need everything from there, but remaining
//! compatible in case an end-user wants to drop that low is deal), it's worth linking their
//! license and repository - they've done really incredible work and it's 100% worth acknowledging.
//!
//! - [core-foundation-rs Repository](https://github.com/servo/core-foundation-rs)
//! - [core-foundation-rs MIT License](https://github.com/servo/core-foundation-rs/blob/master/LICENSE-MIT)
//! - [core-foundation-rs Apache License](https://github.com/servo/core-foundation-rs/blob/master/LICENSE-APACHE)
//!
//! ## Why?
//! A good question. The existing wrappers tend to use traits over `id`, which works well for some
//! cases, but I found it frustrating and messy trying to work with them. These provide the API I
//! was looking for, and help provide all the proper `retain`/`release` logic needed for the
//! Objective-C side.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use objc::runtime;

mod autoreleasepool;
pub use autoreleasepool::AutoReleasePool;

mod array;
pub use array::NSArray;

mod class;
pub(crate) use class::ClassMap;
pub use class::{load_or_register_class, load_or_register_class_with_optional_generated_suffix};

mod data;
pub use data::NSData;

mod dictionary;
pub use dictionary::NSMutableDictionary;

mod number;
pub use number::NSNumber;

mod string;
pub use string::NSString;

// Separate named module to not conflict with the `url` crate. Go figure.
mod urls;
pub use urls::{NSURLBookmarkCreationOption, NSURLBookmarkResolutionOption, NSURL};

/// More or less maps over to Objective-C's `id` type, which... can really be anything.
#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;

/// Exactly what it sounds like.
#[allow(non_upper_case_globals)]
pub const nil: id = 0 as id;

/// Platform-specific.
#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;

/// Platform-specific.
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

/// Platform-specific.
#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;

/// Platform-specific.
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;
