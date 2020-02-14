use super::*;

use crate::test_env::{get_with_env, setup_env};

use std::env;

#[test]
fn validate_exists_true() {
    env::set_var("VALIDATE_EXISTS_TRUE", "");
    let output = validate(&EnvValue::Exists("VALIDATE_EXISTS_TRUE".to_string()));
    assert!(output);
}

#[test]
fn validate_exists_false() {
    let output = validate(&EnvValue::Exists("VALIDATE_EXISTS_FALSE".to_string()));
    assert!(!output);
}

#[test]
fn validate_all_exists_true() {
    env::set_var("VALIDATE_ALL_EXISTS_TRUE1", "");
    env::set_var("VALIDATE_ALL_EXISTS_TRUE2", "");
    let output = validate(&EnvValue::AllExists(vec![
        "VALIDATE_ALL_EXISTS_TRUE1".to_string(),
        "VALIDATE_ALL_EXISTS_TRUE1".to_string(),
    ]));
    assert!(output);
}

#[test]
fn validate_all_exists_false() {
    env::set_var("VALIDATE_ALL_EXISTS_FALSE1", "");
    let output = validate(&EnvValue::AllExists(vec![
        "VALIDATE_ALL_EXISTS_FALSE1".to_string(),
        "VALIDATE_ALL_EXISTS_FALSE2".to_string(),
    ]));
    assert!(!output);
}

#[test]
fn validate_any_exists_true() {
    env::set_var("VALIDATE_ANY_EXISTS_TRUE2", "");
    let output = validate(&EnvValue::AnyExists(vec![
        "VALIDATE_ANY_EXISTS_TRUE1".to_string(),
        "VALIDATE_ANY_EXISTS_TRUE2".to_string(),
    ]));
    assert!(output);
}

#[test]
fn validate_any_exists_false() {
    let output = validate(&EnvValue::AnyExists(vec![
        "VALIDATE_ANY_EXISTS_FALSE1".to_string(),
        "VALIDATE_ANY_EXISTS_FALSE2".to_string(),
    ]));
    assert!(!output);
}

#[test]
fn validate_value_true() {
    env::set_var("VALIDATE_VALUE_TRUE", "test");
    let output = validate(&EnvValue::Value(
        "VALIDATE_VALUE_TRUE".to_string(),
        "test".to_string(),
    ));
    assert!(output);
}

#[test]
fn validate_value_different() {
    env::set_var("VALIDATE_VALUE_DIFFERENT", "test1");
    let output = validate(&EnvValue::Value(
        "VALIDATE_VALUE_DIFFERENT".to_string(),
        "test2".to_string(),
    ));
    assert!(!output);
}

#[test]
fn validate_value_not_exists() {
    let output = validate(&EnvValue::Value(
        "VALIDATE_VALUE_NOT_EXISTS".to_string(),
        "test".to_string(),
    ));
    assert!(!output);
}

#[test]
fn validate_not_equal_true() {
    env::set_var("VALIDATE_NOT_EQUAL_TRUE", "test1");
    let output = validate(&EnvValue::NotEqual(
        "VALIDATE_NOT_EQUAL_TRUE".to_string(),
        "test2".to_string(),
    ));
    assert!(output);
}

#[test]
fn validate_not_equal_same() {
    env::set_var("VALIDATE_NOT_EQUAL_SAME", "test");
    let output = validate(&EnvValue::NotEqual(
        "VALIDATE_NOT_EQUAL_SAME".to_string(),
        "test".to_string(),
    ));
    assert!(!output);
}

#[test]
fn validate_not_equal_not_exists() {
    let output = validate(&EnvValue::NotEqual(
        "VALIDATE_NOT_EQUAL_NOT_EXISTS".to_string(),
        "test".to_string(),
    ));
    assert!(output);
}

#[test]
fn validate_contains_true() {
    env::set_var("VALIDATE_CONTAINS_TRUE", "start test end");
    let output = validate(&EnvValue::Contains(
        "VALIDATE_CONTAINS_TRUE".to_string(),
        "TEST".to_string(),
    ));
    assert!(output);
}

#[test]
fn validate_contains_false() {
    env::set_var("VALIDATE_CONTAINS_FALSE", "start end");
    let output = validate(&EnvValue::Contains(
        "VALIDATE_CONTAINS_FALSE".to_string(),
        "test".to_string(),
    ));
    assert!(!output);
}

#[test]
fn validate_contains_not_exists() {
    let output = validate(&EnvValue::Contains(
        "VALIDATE_CONTAINS_NOT_EXISTS".to_string(),
        "test".to_string(),
    ));
    assert!(!output);
}

#[test]
fn is_ci_test() {
    let lock = setup_env(vec![]);
    let info = get();
    let ci = is_ci();
    drop(lock);
    assert_eq!(info.ci, ci);
}

#[test]
fn get_test() {
    let info = get_with_env(vec![]);
    assert_eq!(info.name.is_some(), info.vendor.is_some());
}

#[test]
fn get_no_pr_appveyor() {
    let info = get_with_env(vec![
        ("APPVEYOR", ""),
        ("APPVEYOR_REPO_BRANCH", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_appveyor() {
    let info = get_with_env(vec![
        ("APPVEYOR", ""),
        ("APPVEYOR_REPO_BRANCH", "test_branch"),
        ("APPVEYOR_PULL_REQUEST_NUMBER", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_azure_piplines() {
    let info = get_with_env(vec![
        ("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI", ""),
        ("BUILD_SOURCEBRANCHNAME", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_azure_piplines() {
    let info = get_with_env(vec![
        ("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI", ""),
        ("BUILD_SOURCEBRANCHNAME", "test_branch"),
        ("SYSTEM_PULLREQUEST_PULLREQUESTID", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_bamboo() {
    let info = get_with_env(vec![("bamboo_planKey", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Bamboo);
    assert_eq!(info.name.unwrap(), "Bamboo");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_bitbucket_piplines() {
    let info = get_with_env(vec![
        ("BITBUCKET_COMMIT", ""),
        ("BITBUCKET_BRANCH", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_bitbucket_piplines() {
    let info = get_with_env(vec![
        ("BITBUCKET_COMMIT", ""),
        ("BITBUCKET_BRANCH", "test_branch"),
        ("BITBUCKET_PR_ID", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_bitrise() {
    let info = get_with_env(vec![
        ("BITRISE_IO", ""),
        ("BITRISE_GIT_BRANCH", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_bitrise() {
    let info = get_with_env(vec![
        ("BITRISE_IO", ""),
        ("BITRISE_GIT_BRANCH", "test_branch"),
        ("BITRISE_PULL_REQUEST", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_buddy() {
    let info = get_with_env(vec![
        ("BUDDY_WORKSPACE_ID", ""),
        ("BUDDY_EXECUTION_BRANCH", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_buddy() {
    let info = get_with_env(vec![
        ("BUDDY_WORKSPACE_ID", ""),
        ("BUDDY_EXECUTION_BRANCH", "test_branch"),
        ("BUDDY_EXECUTION_PULL_REQUEST_ID", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_buildkite() {
    let info = get_with_env(vec![("BUILDKITE", ""), ("BUILDKITE_PULL_REQUEST", "false")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_buildkite() {
    let info = get_with_env(vec![("BUILDKITE", ""), ("BUILDKITE_PULL_REQUEST", "123")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr2_buildkite() {
    let info = get_with_env(vec![("BUILDKITE", "")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_circle_ci() {
    let info = get_with_env(vec![("CIRCLECI", ""), ("CIRCLE_BRANCH", "test_branch")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_circle_ci() {
    let info = get_with_env(vec![
        ("CIRCLECI", ""),
        ("CIRCLE_BRANCH", "test_branch"),
        ("CIRCLE_PULL_REQUEST", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_cirrus_ci() {
    let info = get_with_env(vec![("CIRRUS_CI", ""), ("CIRRUS_BRANCH", "test_branch")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_cirrus_ci() {
    let info = get_with_env(vec![
        ("CIRRUS_CI", ""),
        ("CIRRUS_BRANCH", "test_branch"),
        ("CIRRUS_PR", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_aws_codebuild() {
    let info = get_with_env(vec![("CODEBUILD_BUILD_ARN", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::AWSCodeBuild);
    assert_eq!(info.name.unwrap(), "AWS CodeBuild");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_codeship() {
    let info = get_with_env(vec![("CI_NAME", "codeship"), ("CI_BRANCH", "test_branch")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Codeship);
    assert_eq!(info.name.unwrap(), "Codeship");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_drone() {
    let info = get_with_env(vec![("DRONE", "")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_drone() {
    let info = get_with_env(vec![("DRONE", ""), ("DRONE_BUILD_EVENT", "test")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_drone() {
    let info = get_with_env(vec![("DRONE", ""), ("DRONE_BUILD_EVENT", "pull_request")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_dsari() {
    let info = get_with_env(vec![("DSARI", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::DSARI);
    assert_eq!(info.name.unwrap(), "dsari");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_github_actions() {
    let info = get_with_env(vec![("GITHUB_ACTIONS", "")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_github_actions() {
    let info = get_with_env(vec![("GITHUB_ACTIONS", ""), ("GITHUB_EVENT_NAME", "test")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_github_actions() {
    let info = get_with_env(vec![
        ("GITHUB_ACTIONS", ""),
        ("GITHUB_EVENT_NAME", "pull_request"),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_gitlab_ci() {
    let info = get_with_env(vec![
        ("GITLAB_CI", ""),
        ("CI_COMMIT_REF_NAME", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GitLabCI);
    assert_eq!(info.name.unwrap(), "GitLab CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_gocd() {
    let info = get_with_env(vec![("GO_PIPELINE_LABEL", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GoCD);
    assert_eq!(info.name.unwrap(), "GoCD");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_heroku() {
    let info = get_with_env(vec![
        ("NODE", "/app/.heroku/node/bin/node"),
        ("HEROKU_TEST_RUN_BRANCH", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Heroku);
    assert_eq!(info.name.unwrap(), "Heroku");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_heroku_not_ci() {
    let info = get_with_env(vec![("NODE", "test")]);

    assert!(!info.ci);
}

#[test]
fn get_hudson() {
    let info = get_with_env(vec![("HUDSON_URL", ""), ("BRANCH_NAME", "test_branch")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Hudson);
    assert_eq!(info.name.unwrap(), "Hudson");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_jenkins() {
    let info = get_with_env(vec![
        ("JENKINS_URL", ""),
        ("BRANCH_NAME", "test_branch"),
        ("BUILD_ID", ""),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_partial1_jenkins() {
    let info = get_with_env(vec![("JENKINS_URL", "")]);

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_partial2_jenkins() {
    let info = get_with_env(vec![("BUILD_ID", "")]);

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_jenkins() {
    let info = get_with_env(vec![
        ("JENKINS_URL", ""),
        ("BUILD_ID", ""),
        ("BRANCH_NAME", "test_branch"),
        ("ghprbPullId", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr2_jenkins() {
    let info = get_with_env(vec![
        ("JENKINS_URL", ""),
        ("BUILD_ID", ""),
        ("BRANCH_NAME", "test_branch"),
        ("CHANGE_ID", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_magnum_ci() {
    let info = get_with_env(vec![("MAGNUM", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::MagnumCI);
    assert_eq!(info.name.unwrap(), "Magnum CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_netlify_ci() {
    let info = get_with_env(vec![
        ("NETLIFY", "true"),
        ("BRANCH", "test_branch"),
        ("PULL_REQUEST", "false"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_netlify_ci() {
    let info = get_with_env(vec![
        ("NETLIFY", "true"),
        ("BRANCH", "test_branch"),
        ("PULL_REQUEST", "123"),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr2_netlify_ci() {
    let info = get_with_env(vec![("NETLIFY", "true"), ("BRANCH", "test_branch")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_nevercode_ci() {
    let info = get_with_env(vec![
        ("NEVERCODE", ""),
        ("NEVERCODE_BRANCH", "test_branch"),
        ("NEVERCODE_PULL_REQUEST", "false"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_nevercode_ci() {
    let info = get_with_env(vec![
        ("NEVERCODE", ""),
        ("NEVERCODE_BRANCH", "test_branch"),
        ("NEVERCODE_PULL_REQUEST", "123"),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr2_nevercode_ci() {
    let info = get_with_env(vec![("NEVERCODE", ""), ("NEVERCODE_BRANCH", "test_branch")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_render_ci() {
    let info = get_with_env(vec![("RENDER", "")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_render_ci() {
    let info = get_with_env(vec![("RENDER", ""), ("IS_PULL_REQUEST", "true")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_sail_ci() {
    let info = get_with_env(vec![("SAILCI", "")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_sail_ci() {
    let info = get_with_env(vec![("SAILCI", ""), ("SAIL_PULL_REQUEST_NUMBER", "")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_semaphore() {
    let info = get_with_env(vec![
        ("SEMAPHORE", ""),
        ("SEMAPHORE_GIT_BRANCH", "test_branch"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_semaphore() {
    let info = get_with_env(vec![
        ("SEMAPHORE", ""),
        ("SEMAPHORE_GIT_BRANCH", "test_branch"),
        ("PULL_REQUEST_NUMBER", ""),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_no_pr_shippable() {
    let info = get_with_env(vec![("SHIPPABLE", "")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr2_shippable() {
    let info = get_with_env(vec![("SHIPPABLE", ""), ("IS_PULL_REQUEST", "123")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_shippable() {
    let info = get_with_env(vec![("SHIPPABLE", ""), ("IS_PULL_REQUEST", "true")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_solano_ci() {
    let info = get_with_env(vec![("TDDIUM", "")]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_pr_solano_ci() {
    let info = get_with_env(vec![("TDDIUM", ""), ("TDDIUM_PR_ID", "")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_strider_cd() {
    let info = get_with_env(vec![("STRIDER", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::StriderCD);
    assert_eq!(info.name.unwrap(), "Strider CD");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_taskcluster() {
    let info = get_with_env(vec![("TASK_ID", ""), ("RUN_ID", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TaskCluster);
    assert_eq!(info.name.unwrap(), "TaskCluster");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_partial1_taskcluster() {
    let info = get_with_env(vec![("TASK_ID", "")]);

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_partial2_taskcluster() {
    let info = get_with_env(vec![("RUN_ID", "")]);

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_teamcity() {
    let info = get_with_env(vec![("TEAMCITY_VERSION", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TeamCity);
    assert_eq!(info.name.unwrap(), "TeamCity");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_no_pr_travis() {
    let info = get_with_env(vec![
        ("TRAVIS", ""),
        ("TRAVIS_BRANCH", "test_branch"),
        ("TRAVIS_PULL_REQUEST", "false"),
    ]);

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr_travis() {
    let info = get_with_env(vec![
        ("TRAVIS", ""),
        ("TRAVIS_BRANCH", "test_branch"),
        ("TRAVIS_PULL_REQUEST", "123"),
    ]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_pr2_travis() {
    let info = get_with_env(vec![("TRAVIS", ""), ("TRAVIS_BRANCH", "test_branch")]);

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "test_branch");
}

#[test]
fn get_ziet_now() {
    let info = get_with_env(vec![("NOW_BUILDER", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::ZEITNow);
    assert_eq!(info.name.unwrap(), "ZEIT Now");
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_1() {
    let info = get_with_env(vec![("CI", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_2() {
    let info = get_with_env(vec![("CONTINUOUS_INTEGRATION", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_3() {
    let info = get_with_env(vec![("BUILD_NUMBER", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}

#[test]
fn get_ci_unknown_4() {
    let info = get_with_env(vec![("RUN_ID", "")]);

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(info.branch_name.is_none());
}
