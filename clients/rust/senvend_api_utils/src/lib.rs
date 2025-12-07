use uuid::Version;

/// Error type for UUID parsing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UuidParseError {
    /// Valid version is 4
    InvalidVersion(Version),
    /// If Version field is missing(n.a.)
    MissingVersion,
}
