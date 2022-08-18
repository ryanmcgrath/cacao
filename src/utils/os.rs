//! Helper methods for OS version checking.

use lazy_static::lazy_static;
use os_info::Version;

lazy_static! {
    /// A cached struct containing OS version for runtime checks.
    pub static ref OS_VERSION: os_info::Info = os_info::get();
}

/// In rare cases we need to check whether something is a specific version of macOS. This is a
/// runtime check thhat returns a boolean indicating whether the current version is a minimum target.
#[inline(always)]
pub fn is_minimum_version(minimum_major: u64) -> bool {
    match OS_VERSION.version() {
        Version::Semantic(os_major, _, _) => *os_major >= minimum_major,
        _ => false
    }
}

/// In rare cases we need to check whether something is a specific version of macOS. This is a
/// runtime check thhat returns a boolean indicating whether the current version is a minimum target.
#[inline(always)]
pub fn is_minimum_semversion(major: u64, minor: u64, patch: u64) -> bool {
    let target = Version::Semantic(major, minor, patch);
    OS_VERSION.version() > &target
}
