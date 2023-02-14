//! # config
//!
//! Predefined config of CI vendors.
//!

use crate::types::{EnvValue, Vendor, VendorConfig};

pub(crate) fn create() -> Vec<VendorConfig> {
    let mut config = vec![];

    config.push(VendorConfig {
        name: "AppCenter".to_string(),
        vendor: Vendor::AppCenter,
        ci_env: EnvValue::Exists("APPCENTER_BUILD_ID".to_string()),
        pr_env: None,
        branch_name_env: Some("APPCENTER_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "AppCircle".to_string(),
        vendor: Vendor::AppCircle,
        ci_env: EnvValue::Exists("AC_APPCIRCLE".to_string()),
        pr_env: None,
        branch_name_env: Some("AC_GIT_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "AppVeyor".to_string(),
        vendor: Vendor::AppVeyor,
        ci_env: EnvValue::Exists("APPVEYOR".to_string()),
        pr_env: Some(EnvValue::Exists("APPVEYOR_PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: Some("APPVEYOR_REPO_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "AWS CodeBuild".to_string(),
        vendor: Vendor::AWSCodeBuild,
        ci_env: EnvValue::Exists("CODEBUILD_BUILD_ARN".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Azure Pipelines".to_string(),
        vendor: Vendor::AzurePipelines,
        ci_env: EnvValue::Exists("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI".to_string()),
        pr_env: Some(EnvValue::Exists(
            "SYSTEM_PULLREQUEST_PULLREQUESTID".to_string(),
        )),
        branch_name_env: Some("BUILD_SOURCEBRANCHNAME".to_string()),
    });

    config.push(VendorConfig {
        name: "Bamboo".to_string(),
        vendor: Vendor::Bamboo,
        ci_env: EnvValue::Exists("bamboo_planKey".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Bitbucket Pipelines".to_string(),
        vendor: Vendor::BitbucketPipelines,
        ci_env: EnvValue::Exists("BITBUCKET_COMMIT".to_string()),
        pr_env: Some(EnvValue::Exists("BITBUCKET_PR_ID".to_string())),
        branch_name_env: Some("BITBUCKET_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Bitrise".to_string(),
        vendor: Vendor::Bitrise,
        ci_env: EnvValue::Exists("BITRISE_IO".to_string()),
        pr_env: Some(EnvValue::Exists("BITRISE_PULL_REQUEST".to_string())),
        branch_name_env: Some("BITRISE_GIT_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Buddy".to_string(),
        vendor: Vendor::Buddy,
        ci_env: EnvValue::Exists("BUDDY_WORKSPACE_ID".to_string()),
        pr_env: Some(EnvValue::Exists(
            "BUDDY_EXECUTION_PULL_REQUEST_ID".to_string(),
        )),
        branch_name_env: Some("BUDDY_EXECUTION_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Buildkite".to_string(),
        vendor: Vendor::Buildkite,
        ci_env: EnvValue::Exists("BUILDKITE".to_string()),
        pr_env: Some(EnvValue::NotEqual(
            "BUILDKITE_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "CircleCI".to_string(),
        vendor: Vendor::CircleCI,
        ci_env: EnvValue::Exists("CIRCLECI".to_string()),
        pr_env: Some(EnvValue::Exists("CIRCLE_PULL_REQUEST".to_string())),
        branch_name_env: Some("CIRCLE_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Cirrus CI".to_string(),
        vendor: Vendor::CirrusCI,
        ci_env: EnvValue::Exists("CIRRUS_CI".to_string()),
        pr_env: Some(EnvValue::Exists("CIRRUS_PR".to_string())),
        branch_name_env: Some("CIRRUS_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Codefresh".to_string(),
        vendor: Vendor::Codefresh,
        ci_env: EnvValue::Exists("CF_BUILD_ID".to_string()),
        pr_env: Some(EnvValue::Exists("CF_PULL_REQUEST_ID".to_string())),
        branch_name_env: Some("CF_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Codemagic".to_string(),
        vendor: Vendor::Codemagic,
        ci_env: EnvValue::Exists("CM_BUILD_ID".to_string()),
        pr_env: Some(EnvValue::Value(
            "CM_PULL_REQUEST".to_string(),
            "true".to_string(),
        )),
        branch_name_env: Some("CM_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Codeship".to_string(),
        vendor: Vendor::Codeship,
        ci_env: EnvValue::Value("CI_NAME".to_string(), "codeship".to_string()),
        pr_env: None,
        branch_name_env: Some("CI_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Drone".to_string(),
        vendor: Vendor::Drone,
        ci_env: EnvValue::Exists("DRONE".to_string()),
        pr_env: Some(EnvValue::Value(
            "DRONE_BUILD_EVENT".to_string(),
            "pull_request".to_string(),
        )),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "dsari".to_string(),
        vendor: Vendor::DSARI,
        ci_env: EnvValue::Exists("DSARI".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "GitHub Actions".to_string(),
        vendor: Vendor::GitHubActions,
        ci_env: EnvValue::Exists("GITHUB_ACTIONS".to_string()),
        pr_env: Some(EnvValue::Value(
            "GITHUB_EVENT_NAME".to_string(),
            "pull_request".to_string(),
        )),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "GitLab CI".to_string(),
        vendor: Vendor::GitLabCI,
        ci_env: EnvValue::Exists("GITLAB_CI".to_string()),
        pr_env: Some(EnvValue::Exists("CI_MERGE_REQUEST_ID".to_string())),
        branch_name_env: Some("CI_COMMIT_REF_NAME".to_string()),
    });

    config.push(VendorConfig {
        name: "Flow CI".to_string(),
        vendor: Vendor::FlowCI,
        ci_env: EnvValue::Exists("FLOWCI_JOB_BUILD_NUM".to_string()),
        pr_env: None,
        branch_name_env: Some("FLOWCI_GIT_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Gerrit".to_string(),
        vendor: Vendor::Gerrit,
        ci_env: EnvValue::Exists("GERRIT_PROJECT".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "GoCD".to_string(),
        vendor: Vendor::GoCD,
        ci_env: EnvValue::Exists("GO_PIPELINE_LABEL".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Google Cloud Build".to_string(),
        vendor: Vendor::GoogleCloudBuild,
        ci_env: EnvValue::Exists("BUILDER_OUTPUT".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Heroku".to_string(),
        vendor: Vendor::Heroku,
        ci_env: EnvValue::Contains("NODE".to_string(), "heroku".to_string()),
        pr_env: None,
        branch_name_env: Some("HEROKU_TEST_RUN_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Hudson".to_string(),
        vendor: Vendor::Hudson,
        ci_env: EnvValue::Exists("HUDSON_URL".to_string()),
        pr_env: None,
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    config.push(VendorConfig {
        name: "Jenkins".to_string(),
        vendor: Vendor::Jenkins,
        ci_env: EnvValue::AllExists(vec!["JENKINS_URL".to_string(), "BUILD_ID".to_string()]),
        pr_env: Some(EnvValue::AnyExists(vec![
            "ghprbPullId".to_string(),
            "CHANGE_ID".to_string(),
        ])),
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    config.push(VendorConfig {
        name: "JenkinsX".to_string(),
        vendor: Vendor::JenkinsX,
        ci_env: EnvValue::AllExists(vec![
            "JX_CHART_REPOSITORY".to_string(),
            "BUILD_ID".to_string(),
        ]),
        pr_env: Some(EnvValue::Exists("PULL_NUMBER".to_string())),
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    config.push(VendorConfig {
        name: "Layer CI".to_string(),
        vendor: Vendor::LayerCI,
        ci_env: EnvValue::Exists("LAYERCI".to_string()),
        pr_env: Some(EnvValue::Exists("LAYERCI_PULL_REQUEST".to_string())),
        branch_name_env: Some("LAYERCI_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Magnum CI".to_string(),
        vendor: Vendor::MagnumCI,
        ci_env: EnvValue::Exists("MAGNUM".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Netlify CI".to_string(),
        vendor: Vendor::NetlifyCI,
        ci_env: EnvValue::Value("NETLIFY".to_string(), "true".to_string()),
        pr_env: Some(EnvValue::NotEqual(
            "PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: Some("BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Nevercode".to_string(),
        vendor: Vendor::Nevercode,
        ci_env: EnvValue::Exists("NEVERCODE".to_string()),
        pr_env: Some(EnvValue::NotEqual(
            "NEVERCODE_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: Some("NEVERCODE_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Prow".to_string(),
        vendor: Vendor::Prow,
        ci_env: EnvValue::Exists("PROW_JOB_ID".to_string()),
        pr_env: Some(EnvValue::NotEmpty("PULL_NUMBER".to_string())),
        branch_name_env: Some("PULL_BASE_REF".to_string()),
    });

    config.push(VendorConfig {
        name: "Render".to_string(),
        vendor: Vendor::Render,
        ci_env: EnvValue::Exists("RENDER".to_string()),
        pr_env: Some(EnvValue::Value(
            "IS_PULL_REQUEST".to_string(),
            "true".to_string(),
        )),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Sail CI".to_string(),
        vendor: Vendor::SailCI,
        ci_env: EnvValue::Exists("SAILCI".to_string()),
        pr_env: Some(EnvValue::Exists("SAIL_PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Screwdriver".to_string(),
        vendor: Vendor::Screwdriver,
        ci_env: EnvValue::Exists("SCREWDRIVER".to_string()),
        pr_env: Some(EnvValue::NotEmpty("SD_PULL_REQUEST".to_string())),
        branch_name_env: Some("GIT_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Semaphore".to_string(),
        vendor: Vendor::Semaphore,
        ci_env: EnvValue::Exists("SEMAPHORE".to_string()),
        pr_env: Some(EnvValue::Exists("PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: Some("SEMAPHORE_GIT_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Shippable".to_string(),
        vendor: Vendor::Shippable,
        ci_env: EnvValue::Exists("SHIPPABLE".to_string()),
        pr_env: Some(EnvValue::Value(
            "IS_PULL_REQUEST".to_string(),
            "true".to_string(),
        )),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Solano CI".to_string(),
        vendor: Vendor::SolanoCI,
        ci_env: EnvValue::Exists("TDDIUM".to_string()),
        pr_env: Some(EnvValue::Exists("TDDIUM_PR_ID".to_string())),
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "SourceHut".to_string(),
        vendor: Vendor::SourceHut,
        ci_env: EnvValue::Contains("JOB_URL".to_string(), "builds.sr.ht".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Strider CD".to_string(),
        vendor: Vendor::StriderCD,
        ci_env: EnvValue::Exists("STRIDER".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "TaskCluster".to_string(),
        vendor: Vendor::TaskCluster,
        ci_env: EnvValue::AllExists(vec!["TASK_ID".to_string(), "RUN_ID".to_string()]),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "TeamCity".to_string(),
        vendor: Vendor::TeamCity,
        ci_env: EnvValue::Exists("TEAMCITY_VERSION".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Travis CI".to_string(),
        vendor: Vendor::TravisCI,
        ci_env: EnvValue::Exists("TRAVIS".to_string()),
        pr_env: Some(EnvValue::NotEqual(
            "TRAVIS_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: Some("TRAVIS_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Vela".to_string(),
        vendor: Vendor::Vela,
        ci_env: EnvValue::Exists("VELA_BUILD_NUMBER".to_string()),
        pr_env: Some(EnvValue::Exists("VELA_PULL_REQUEST".to_string())),
        branch_name_env: Some("VELA_BUILD_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Vercel".to_string(),
        vendor: Vendor::Vercel,
        ci_env: EnvValue::AnyExists(vec!["NOW_BUILDER".to_string(), "VERCEL".to_string()]),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Woodpecker CI".to_string(),
        vendor: Vendor::WoodpeckerCI,
        ci_env: EnvValue::Value("CI".to_string(), "woodpecker".to_string()),
        pr_env: Some(EnvValue::NotEmpty("CI_COMMIT_PULL_REQUEST".to_string())),
        branch_name_env: Some("CI_COMMIT_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Xcode Cloud".to_string(),
        vendor: Vendor::XcodeCloud,
        ci_env: EnvValue::Exists("CI_XCODE_PROJECT".to_string()),
        pr_env: Some(EnvValue::NotEmpty("CI_PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: Some("CI_BRANCH".to_string()),
    });

    config.push(VendorConfig {
        name: "Xcode Server".to_string(),
        vendor: Vendor::XcodeServer,
        ci_env: EnvValue::Exists("XCS".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    config.push(VendorConfig {
        name: "Unknown".to_string(),
        vendor: Vendor::Unknown,
        ci_env: EnvValue::AnyExists(vec![
            "CI".to_string(),
            "CONTINUOUS_INTEGRATION".to_string(),
            "BUILD_NUMBER".to_string(),
            "RUN_ID".to_string(),
        ]),
        pr_env: None,
        branch_name_env: None,
    });

    config
}
