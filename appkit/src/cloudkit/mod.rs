//! This module wraps a portion of the CloudKit API. This is a fairly extensive API, and is not
//! easy to wrap - if you use this and need something that's not implemented, please consider
//! helping out with an implementation and pull request.

pub mod share;
pub use share::CKShareMetaData;
