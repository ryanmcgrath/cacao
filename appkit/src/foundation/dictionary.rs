//! A wrapper for `NSDictionary`, which aims to make dealing with the class throughout this
//! framework a tad bit simpler.

use objc::runtime::Object;
use objc_id::Id;

#[derive(Debug)]
pub struct NSDictionary(Id<Object>);

impl NSDictionary {}
