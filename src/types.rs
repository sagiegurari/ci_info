//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

#[cfg_attr(feature = "serde-1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Copy)]
#[non_exhaustive]
/// Supported vendors enum
pub enum Vendor {
    /// CI vendor
    Agola,
    /// CI vendor
    AppCenter,
    /// CI vendor
    AppCircle,
    /// CI vendor
    AppVeyor,
    /// CI vendor
    AWSCodeBuild,
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
    Codefresh,
    /// CI vendor
    Codemagic,
    /// CI vendor
    Codeship,
    /// CI vendor
    Drone,
    /// CI vendor
    DSARI,
    /// CI vendor
    EARTHLY,
    /// CI vendor
    FlowCI,
    /// CI vendor
    GiteaActions,
    /// CI vendor
    GitLabCI,
    /// CI vendor
    GitHubActions,
    /// CI vendor
    Gerrit,
    /// CI vendor
    GoCD,
    /// CI vendor
    GoogleCloudBuild,
    /// CI vendor
    HarnessCI,
    /// CI vendor
    Heroku,
    /// CI vendor
    Hudson,
    /// CI vendor
    Jenkins,
    /// CI vendor
    JenkinsX,
    /// CI vendor
    LayerCI,
    /// CI vendor
    MagnumCI,
    /// CI vendor
    NetlifyCI,
    /// CI vendor
    Nevercode,
    /// CI vendor
    Prow,
    /// CI vendor
    Render,
    /// CI vendor
    SailCI,
    /// CI vendor
    Screwdriver,
    /// CI vendor
    Semaphore,
    /// CI vendor
    SolanoCI,
    /// CI vendor
    SourceHut,
    /// CI vendor
    StriderCD,
    /// CI vendor
    TaskCluster,
    /// CI vendor
    TeamCity,
    /// CI vendor
    TravisCI,
    /// CI vendor
    Vela,
    /// CI vendor
    Vercel,
    /// CI vendor
    WoodpeckerCI,
    /// CI vendor
    XcodeCloud,
    /// CI vendor
    XcodeServer,
    /// CI vendor
    Unknown,
}

#[derive(Debug, Clone)]
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
    /// Env name contains the provided value (case insensitive)
    Contains(String, String),
    /// Env value exists and not empty
    NotEmpty(String),
}

/// Vendor detection info
#[derive(Debug, Clone)]
pub(crate) struct VendorConfig {
    /// vendor name
    pub(crate) name: String,
    /// The CI vendor
    pub(crate) vendor: Vendor,
    /// CI env var name
    pub(crate) ci_env: EnvValue,
    /// PR env var name
    pub(crate) pr_env: Option<EnvValue>,
    /// Branch name env var name
    pub(crate) branch_name_env: Option<String>,
}

#[cfg_attr(feature = "serde-1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// The branch name for the given build
    pub branch_name: Option<String>,
}

impl CiInfo {
    /// Returns new instance
    pub fn new() -> CiInfo {
        Default::default()
    }
}
