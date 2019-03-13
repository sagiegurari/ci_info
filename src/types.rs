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
    AppVeyor,
    /// CI vendor
    AzurePipelines,
    /// CI vendor
    Bamboo,
    /// CI vendor
    BitbucketPipelines,
    /// CI vendor
    Bitrise,
    /// CI vendor
    Buddy,
    /// CI vendor
    Buildkite,
    /// CI vendor
    CircleCI,
    /// CI vendor
    CirrusCI,
    /// CI vendor
    AWSCodeBuild,
    /// CI vendor
    Codeship,
    /// CI vendor
    Drone,
    /// CI vendor
    DSARI,
    /// CI vendor
    GitLabCI,
    /// CI vendor
    GoCD,
    /// CI vendor
    Hudson,
    /// CI vendor
    Jenkins,
    /// CI vendor
    MagnumCI,
    /// CI vendor
    NetlifyCI,
    /// CI vendor
    SailCI,
    /// CI vendor
    Semaphore,
    /// CI vendor
    Shippable,
    /// CI vendor
    SolanoCI,
    /// CI vendor
    StriderCD,
    /// CI vendor
    TaskCluster,
    /// CI vendor
    TeamCity,
    /// CI vendor
    TravisCI,
    /// CI vendor
    Unknown,
}

pub(crate) enum EnvValue {
    /// Env name
    Exists(String),
    /// Env names
    AllExists(Vec<String>),
    /// Env names
    AnyExists(Vec<String>),
    /// Env name and value
    Value(String, String),
    /// Env name and value which should not be defined
    NotEqual(String, String),
}

/// Vendor detection info
pub(crate) struct VendorConfig {
    /// vendor name
    pub(crate) name: String,
    /// The CI vendor
    pub(crate) vendor: Vendor,
    /// CI env var name
    pub(crate) ci_env: EnvValue,
    /// PR env var name
    pub(crate) pr_env: Option<EnvValue>,
}

#[cfg_attr(feature = "serde-1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// CI info
pub struct CiInfo {
    /// The CI vendor
    pub vendor: Option<Vendor>,
    /// vendor name
    pub name: Option<String>,
    /// True if CI environment
    pub ci: bool,
    /// True if currently running a PR build, None if unknown
    pub pr: Option<bool>,
}

impl CiInfo {
    /// Returns new instance
    pub fn new() -> CiInfo {
        CiInfo {
            vendor: None,
            name: None,
            ci: false,
            pr: None,
        }
    }
}
