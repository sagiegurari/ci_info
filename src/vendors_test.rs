use crate::test_env::{get_with_env, TestVendorConfig};
use crate::types::{EnvValue, Vendor};

#[test]
fn get_appcenter() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("APPCENTER_BUILD_ID".to_string()),
        pr_env: None,
        branch_name_env: Some("APPCENTER_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::AppCenter);
    assert_eq!(info.name.unwrap(), "AppCenter");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_appveyor() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("APPVEYOR".to_string()),
        pr_env: None,
        branch_name_env: Some("APPVEYOR_REPO_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_appveyor() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("APPVEYOR".to_string()),
        pr_env: Some(EnvValue::Exists("APPVEYOR_PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: Some("APPVEYOR_REPO_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_azure_piplines() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI".to_string()),
        pr_env: None,
        branch_name_env: Some("BUILD_SOURCEBRANCHNAME".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_azure_piplines() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI".to_string()),
        pr_env: Some(EnvValue::Exists(
            "SYSTEM_PULLREQUEST_PULLREQUESTID".to_string(),
        )),
        branch_name_env: Some("BUILD_SOURCEBRANCHNAME".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_bamboo() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("bamboo_planKey".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Bamboo);
    assert_eq!(info.name.unwrap(), "Bamboo");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_bitbucket_piplines() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BITBUCKET_COMMIT".to_string()),
        pr_env: None,
        branch_name_env: Some("BITBUCKET_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_bitbucket_piplines() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BITBUCKET_COMMIT".to_string()),
        pr_env: Some(EnvValue::Exists("BITBUCKET_PR_ID".to_string())),
        branch_name_env: Some("BITBUCKET_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_bitrise() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BITRISE_IO".to_string()),
        pr_env: None,
        branch_name_env: Some("BITRISE_GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_bitrise() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BITRISE_IO".to_string()),
        pr_env: Some(EnvValue::Exists("BITRISE_PULL_REQUEST".to_string())),
        branch_name_env: Some("BITRISE_GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_buddy() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUDDY_WORKSPACE_ID".to_string()),
        pr_env: None,
        branch_name_env: Some("BUDDY_EXECUTION_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_buddy() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUDDY_WORKSPACE_ID".to_string()),
        pr_env: Some(EnvValue::Exists(
            "BUDDY_EXECUTION_PULL_REQUEST_ID".to_string(),
        )),
        branch_name_env: Some("BUDDY_EXECUTION_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_buildkite() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUILDKITE".to_string()),
        pr_env: Some(EnvValue::Value(
            "BUILDKITE_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_buildkite() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUILDKITE".to_string()),
        pr_env: Some(EnvValue::Value(
            "BUILDKITE_PULL_REQUEST".to_string(),
            "123".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr2_buildkite() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUILDKITE".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_circle_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CIRCLECI".to_string()),
        pr_env: None,
        branch_name_env: Some("CIRCLE_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_circle_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CIRCLECI".to_string()),
        pr_env: Some(EnvValue::Exists("CIRCLE_PULL_REQUEST".to_string())),
        branch_name_env: Some("CIRCLE_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_cirrus_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CIRRUS_CI".to_string()),
        pr_env: None,
        branch_name_env: Some("CIRRUS_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_cirrus_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CIRRUS_CI".to_string()),
        pr_env: Some(EnvValue::Exists("CIRRUS_PR".to_string())),
        branch_name_env: Some("CIRRUS_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_aws_codebuild() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CODEBUILD_BUILD_ARN".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::AWSCodeBuild);
    assert_eq!(info.name.unwrap(), "AWS CodeBuild");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_codeship() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value("CI_NAME".to_string(), "codeship".to_string()),
        pr_env: None,
        branch_name_env: Some("CI_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Codeship);
    assert_eq!(info.name.unwrap(), "Codeship");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_drone() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("DRONE".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_drone() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("DRONE".to_string()),
        pr_env: Some(EnvValue::Value(
            "DRONE_BUILD_EVENT".to_string(),
            "test".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_drone() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("DRONE".to_string()),
        pr_env: Some(EnvValue::Value(
            "DRONE_BUILD_EVENT".to_string(),
            "pull_request".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_dsari() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("DSARI".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::DSARI);
    assert_eq!(info.name.unwrap(), "dsari");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_github_actions() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("GITHUB_ACTIONS".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_github_actions() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("GITHUB_ACTIONS".to_string()),
        pr_env: Some(EnvValue::Value(
            "GITHUB_EVENT_NAME".to_string(),
            "test".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_github_actions() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("GITHUB_ACTIONS".to_string()),
        pr_env: Some(EnvValue::Value(
            "GITHUB_EVENT_NAME".to_string(),
            "pull_request".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_gitlab_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("GITLAB_CI".to_string()),
        pr_env: None,
        branch_name_env: Some("CI_COMMIT_REF_NAME".to_string()),
    });

    assert!(info.ci);
    assert_eq!(info.pr.unwrap(), false);
    assert_eq!(info.vendor.unwrap(), Vendor::GitLabCI);
    assert_eq!(info.name.unwrap(), "GitLab CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_gitlab_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("GITLAB_CI".to_string()),
        pr_env: Some(EnvValue::Exists("CI_MERGE_REQUEST_ID".to_string())),
        branch_name_env: Some("CI_COMMIT_REF_NAME".to_string()),
    });

    assert!(info.ci);
    assert_eq!(info.pr.unwrap(), true);
    assert_eq!(info.vendor.unwrap(), Vendor::GitLabCI);
    assert_eq!(info.name.unwrap(), "GitLab CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_gocd() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("GO_PIPELINE_LABEL".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GoCD);
    assert_eq!(info.name.unwrap(), "GoCD");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_heroku() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value("NODE".to_string(), "/app/.heroku/node/bin/node".to_string()),
        pr_env: None,
        branch_name_env: Some("HEROKU_TEST_RUN_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Heroku);
    assert_eq!(info.name.unwrap(), "Heroku");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_heroku_not_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value("NODE".to_string(), "test".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(!info.ci);
}

#[test]
fn get_hudson() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("HUDSON_URL".to_string()),
        pr_env: None,
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Hudson);
    assert_eq!(info.name.unwrap(), "Hudson");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_jenkins() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::AllExists(vec!["JENKINS_URL".to_string(), "BUILD_ID".to_string()]),
        pr_env: None,
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_partial1_jenkins() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("JENKINS_URL".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(!info.ci);
}

#[test]
fn get_partial2_jenkins() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUILD_ID".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(!info.ci);
}

#[test]
fn get_pr_jenkins() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::AllExists(vec!["JENKINS_URL".to_string(), "BUILD_ID".to_string()]),
        pr_env: Some(EnvValue::Exists("ghprbPullId".to_string())),
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr2_jenkins() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::AllExists(vec!["JENKINS_URL".to_string(), "BUILD_ID".to_string()]),
        pr_env: Some(EnvValue::Exists("CHANGE_ID".to_string())),
        branch_name_env: Some("BRANCH_NAME".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_magnum_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("MAGNUM".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::MagnumCI);
    assert_eq!(info.name.unwrap(), "Magnum CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_netlify_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value("NETLIFY".to_string(), "true".to_string()),
        pr_env: Some(EnvValue::Value(
            "PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: Some("BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_netlify_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value("NETLIFY".to_string(), "true".to_string()),
        pr_env: Some(EnvValue::Value(
            "PULL_REQUEST".to_string(),
            "123".to_string(),
        )),
        branch_name_env: Some("BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr2_netlify_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value("NETLIFY".to_string(), "true".to_string()),
        pr_env: None,
        branch_name_env: Some("BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_nevercode_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("NEVERCODE".to_string()),
        pr_env: Some(EnvValue::Value(
            "NEVERCODE_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: Some("NEVERCODE_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_nevercode_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("NEVERCODE".to_string()),
        pr_env: Some(EnvValue::Value(
            "NEVERCODE_PULL_REQUEST".to_string(),
            "123".to_string(),
        )),
        branch_name_env: Some("NEVERCODE_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr2_nevercode_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("NEVERCODE".to_string()),
        pr_env: None,
        branch_name_env: Some("NEVERCODE_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_render_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("RENDER".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_render_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("RENDER".to_string()),
        pr_env: Some(EnvValue::Value(
            "IS_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_render_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("RENDER".to_string()),
        pr_env: Some(EnvValue::Value(
            "IS_PULL_REQUEST".to_string(),
            "true".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_sail_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SAILCI".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_sail_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SAILCI".to_string()),
        pr_env: Some(EnvValue::Exists("SAIL_PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_screwdriver_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SCREWDRIVER".to_string()),
        pr_env: None,
        branch_name_env: Some("GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Screwdriver);
    assert_eq!(info.name.unwrap(), "Screwdriver");
    assert!(info.branch_name.is_some());
}

#[test]
fn get_no_pr2_screwdriver_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SCREWDRIVER".to_string()),
        pr_env: Some(EnvValue::Value(
            "SD_PULL_REQUEST".to_string(),
            "".to_string(),
        )),
        branch_name_env: Some("GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Screwdriver);
    assert_eq!(info.name.unwrap(), "Screwdriver");
    assert!(info.branch_name.is_some());
}

#[test]
fn get_pr_screwdriver_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SCREWDRIVER".to_string()),
        pr_env: Some(EnvValue::Value(
            "SD_PULL_REQUEST".to_string(),
            "1".to_string(),
        )),
        branch_name_env: Some("GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Screwdriver);
    assert_eq!(info.name.unwrap(), "Screwdriver");
    assert!(info.branch_name.is_some());
}

#[test]
fn get_no_pr_semaphore() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SEMAPHORE".to_string()),
        pr_env: None,
        branch_name_env: Some("SEMAPHORE_GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_semaphore() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SEMAPHORE".to_string()),
        pr_env: Some(EnvValue::Exists("PULL_REQUEST_NUMBER".to_string())),
        branch_name_env: Some("SEMAPHORE_GIT_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_no_pr_shippable() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SHIPPABLE".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_shippable() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SHIPPABLE".to_string()),
        pr_env: Some(EnvValue::Value(
            "IS_PULL_REQUEST".to_string(),
            "123".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_shippable() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("SHIPPABLE".to_string()),
        pr_env: Some(EnvValue::Value(
            "IS_PULL_REQUEST".to_string(),
            "true".to_string(),
        )),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_solano_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TDDIUM".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_solano_ci() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TDDIUM".to_string()),
        pr_env: Some(EnvValue::Exists("TDDIUM_PR_ID".to_string())),
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_sourcehut() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Value(
            "JOB_URL".to_string(),
            "https://builds.sr.ht/~test/job/1".to_string(),
        ),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::SourceHut);
    assert_eq!(info.name.unwrap(), "SourceHut");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_strider_cd() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("STRIDER".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::StriderCD);
    assert_eq!(info.name.unwrap(), "Strider CD");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_taskcluster() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::AllExists(vec!["TASK_ID".to_string(), "RUN_ID".to_string()]),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TaskCluster);
    assert_eq!(info.name.unwrap(), "TaskCluster");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_partial1_taskcluster() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TASK_ID".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_partial2_taskcluster() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("RUN_ID".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_teamcity() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TEAMCITY_VERSION".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TeamCity);
    assert_eq!(info.name.unwrap(), "TeamCity");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_travis() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TRAVIS".to_string()),
        pr_env: Some(EnvValue::Value(
            "TRAVIS_PULL_REQUEST".to_string(),
            "false".to_string(),
        )),
        branch_name_env: Some("TRAVIS_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr_travis() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TRAVIS".to_string()),
        pr_env: Some(EnvValue::Value(
            "TRAVIS_PULL_REQUEST".to_string(),
            "123".to_string(),
        )),
        branch_name_env: Some("TRAVIS_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_pr2_travis() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("TRAVIS".to_string()),
        pr_env: None,
        branch_name_env: Some("TRAVIS_BRANCH".to_string()),
    });

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "mock_branch");
}

#[test]
fn get_vercel() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("NOW_BUILDER".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Vercel);
    assert_eq!(info.name.unwrap(), "Vercel");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_1() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CI".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_2() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CONTINUOUS_INTEGRATION".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_3() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("BUILD_NUMBER".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_4() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("RUN_ID".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}
