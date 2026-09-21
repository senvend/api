#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

use uuid::Version;

/// Error type for UUID parsing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UuidVersionError {
    /// Valid version is 4
    InvalidVersion(Version),
    /// If Version field is missing(n.a.)
    MissingVersion,
}

impl core::fmt::Display for UuidVersionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl core::error::Error for UuidVersionError {}
