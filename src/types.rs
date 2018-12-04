//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

#[cfg_attr(feature = "serde-1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Copy)]
/// Supported vendors enum
pub enum Vendor {
    /// CI vendor
    TRAVIS,
    /// CI vendor
    CIRCLE,
    /// CI vendor
    GITLAB,
    /// CI vendor
    APPVEYOR,
    /// CI vendor
    CODESHIP,
    /// CI vendor
    DRONE,
    /// CI vendor
    MAGNUM,
    /// CI vendor
    SEMAPHORE,
    /// CI vendor
    JENKINS,
    /// CI vendor
    BAMBOO,
    /// CI vendor
    TFS,
    /// CI vendor
    TEAMCITY,
    /// CI vendor
    BUILDKITE,
    /// CI vendor
    HUDSON,
    /// CI vendor
    TASKCLUSTER,
    /// CI vendor
    GOCD,
    /// CI vendor
    BITBUCKET,
    /// CI vendor
    CODEBUILD,
}

#[cfg_attr(feature = "serde-1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Copy)]
/// CI info
pub struct CiInfo {
    /// The CI vendor
    pub vendor: Option<Vendor>,
    /// True if CI environment
    pub ci: bool,
}

impl CiInfo {
    /// Returns new instance
    pub fn new() -> CiInfo {
        CiInfo {
            vendor: None,
            ci: false,
        }
    }
}
