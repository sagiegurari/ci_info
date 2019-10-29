use super::*;

use crate::test_env::setup_env;

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
    let _lock = setup_env(vec![]);
    let info = get();
    let ci = is_ci();

    assert_eq!(info.ci, ci);
}

#[test]
fn get_test() {
    let _lock = setup_env(vec![]);
    let info = get();

    assert_eq!(info.name.is_some(), info.vendor.is_some());
}

#[test]
fn get_no_pr_appveyor() {
    let _lock = setup_env(vec![("APPVEYOR", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
}

#[test]
fn get_pr_appveyor() {
    let _lock = setup_env(vec![("APPVEYOR", ""), ("APPVEYOR_PULL_REQUEST_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
}

#[test]
fn get_no_pr_azure_piplines() {
    let _lock = setup_env(vec![("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
}

#[test]
fn get_pr_azure_piplines() {
    let _lock = setup_env(vec![
        ("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI", ""),
        ("SYSTEM_PULLREQUEST_PULLREQUESTID", ""),
    ]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
}

#[test]
fn get_bamboo() {
    let _lock = setup_env(vec![("bamboo_planKey", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Bamboo);
    assert_eq!(info.name.unwrap(), "Bamboo");
}

#[test]
fn get_no_pr_bitbucket_piplines() {
    let _lock = setup_env(vec![("BITBUCKET_COMMIT", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
}

#[test]
fn get_pr_bitbucket_piplines() {
    let _lock = setup_env(vec![("BITBUCKET_COMMIT", ""), ("BITBUCKET_PR_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
}

#[test]
fn get_no_pr_bitrise() {
    let _lock = setup_env(vec![("BITRISE_IO", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
}

#[test]
fn get_pr_bitrise() {
    let _lock = setup_env(vec![("BITRISE_IO", ""), ("BITRISE_PULL_REQUEST", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
}

#[test]
fn get_no_pr_buddy() {
    let _lock = setup_env(vec![("BUDDY_WORKSPACE_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
}

#[test]
fn get_pr_buddy() {
    let _lock = setup_env(vec![
        ("BUDDY_WORKSPACE_ID", ""),
        ("BUDDY_EXECUTION_PULL_REQUEST_ID", ""),
    ]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
}

#[test]
fn get_no_pr_buildkite() {
    let _lock = setup_env(vec![("BUILDKITE", ""), ("BUILDKITE_PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
}

#[test]
fn get_pr_buildkite() {
    let _lock = setup_env(vec![("BUILDKITE", ""), ("BUILDKITE_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
}

#[test]
fn get_pr2_buildkite() {
    let _lock = setup_env(vec![("BUILDKITE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
}

#[test]
fn get_no_pr_circle_ci() {
    let _lock = setup_env(vec![("CIRCLECI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
}

#[test]
fn get_pr_circle_ci() {
    let _lock = setup_env(vec![("CIRCLECI", ""), ("CIRCLE_PULL_REQUEST", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
}

#[test]
fn get_no_pr_cirrus_ci() {
    let _lock = setup_env(vec![("CIRRUS_CI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
}

#[test]
fn get_pr_cirrus_ci() {
    let _lock = setup_env(vec![("CIRRUS_CI", ""), ("CIRRUS_PR", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
}

#[test]
fn get_aws_codebuild() {
    let _lock = setup_env(vec![("CODEBUILD_BUILD_ARN", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::AWSCodeBuild);
    assert_eq!(info.name.unwrap(), "AWS CodeBuild");
}

#[test]
fn get_codeship() {
    let _lock = setup_env(vec![("CI_NAME", "codeship")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Codeship);
    assert_eq!(info.name.unwrap(), "Codeship");
}

#[test]
fn get_no_pr_drone() {
    let _lock = setup_env(vec![("DRONE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
}

#[test]
fn get_no_pr2_drone() {
    let _lock = setup_env(vec![("DRONE", ""), ("DRONE_BUILD_EVENT", "test")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
}

#[test]
fn get_pr_drone() {
    let _lock = setup_env(vec![("DRONE", ""), ("DRONE_BUILD_EVENT", "pull_request")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
}

#[test]
fn get_dsari() {
    let _lock = setup_env(vec![("DSARI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::DSARI);
    assert_eq!(info.name.unwrap(), "dsari");
}

#[test]
fn get_no_pr_github_actions() {
    let _lock = setup_env(vec![("GITHUB_ACTIONS", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
}

#[test]
fn get_no_pr2_github_actions() {
    let _lock = setup_env(vec![("GITHUB_ACTIONS", ""), ("GITHUB_EVENT_NAME", "test")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
}

#[test]
fn get_pr_github_actions() {
    let _lock = setup_env(vec![
        ("GITHUB_ACTIONS", ""),
        ("GITHUB_EVENT_NAME", "pull_request"),
    ]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::GitHubActions);
    assert_eq!(info.name.unwrap(), "GitHub Actions");
}

#[test]
fn get_gitlab_ci() {
    let _lock = setup_env(vec![("GITLAB_CI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GitLabCI);
    assert_eq!(info.name.unwrap(), "GitLab CI");
}

#[test]
fn get_gocd() {
    let _lock = setup_env(vec![("GO_PIPELINE_LABEL", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GoCD);
    assert_eq!(info.name.unwrap(), "GoCD");
}

#[test]
fn get_heroku() {
    let _lock = setup_env(vec![("NODE", "/app/.heroku/node/bin/node")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Heroku);
    assert_eq!(info.name.unwrap(), "Heroku");
}

#[test]
fn get_heroku_not_ci() {
    let _lock = setup_env(vec![("NODE", "test")]);

    let info = get();

    assert!(!info.ci);
}

#[test]
fn get_hudson() {
    let _lock = setup_env(vec![("HUDSON_URL", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Hudson);
    assert_eq!(info.name.unwrap(), "Hudson");
}

#[test]
fn get_no_pr_jenkins() {
    let _lock = setup_env(vec![("JENKINS_URL", ""), ("BUILD_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
}

#[test]
fn get_partial1_jenkins() {
    let _lock = setup_env(vec![("JENKINS_URL", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_partial2_jenkins() {
    let _lock = setup_env(vec![("BUILD_ID", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_pr_jenkins() {
    let _lock = setup_env(vec![
        ("JENKINS_URL", ""),
        ("BUILD_ID", ""),
        ("ghprbPullId", ""),
    ]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
}

#[test]
fn get_pr2_jenkins() {
    let _lock = setup_env(vec![
        ("JENKINS_URL", ""),
        ("BUILD_ID", ""),
        ("CHANGE_ID", ""),
    ]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
}

#[test]
fn get_magnum_ci() {
    let _lock = setup_env(vec![("MAGNUM", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::MagnumCI);
    assert_eq!(info.name.unwrap(), "Magnum CI");
}

#[test]
fn get_no_pr_netlify_ci() {
    let _lock = setup_env(vec![("NETLIFY_BUILD_BASE", ""), ("PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
}

#[test]
fn get_pr_netlify_ci() {
    let _lock = setup_env(vec![("NETLIFY_BUILD_BASE", ""), ("PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
}

#[test]
fn get_pr2_netlify_ci() {
    let _lock = setup_env(vec![("NETLIFY_BUILD_BASE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
}

#[test]
fn get_no_pr_nevercode_ci() {
    let _lock = setup_env(vec![("NEVERCODE", ""), ("NEVERCODE_PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
}

#[test]
fn get_pr_nevercode_ci() {
    let _lock = setup_env(vec![("NEVERCODE", ""), ("NEVERCODE_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
}

#[test]
fn get_pr2_nevercode_ci() {
    let _lock = setup_env(vec![("NEVERCODE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
}

#[test]
fn get_no_pr_render_ci() {
    let _lock = setup_env(vec![("RENDER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
}

#[test]
fn get_pr_render_ci() {
    let _lock = setup_env(vec![("RENDER", ""), ("IS_PULL_REQUEST", "true")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
}

#[test]
fn get_no_pr_sail_ci() {
    let _lock = setup_env(vec![("SAILCI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
}

#[test]
fn get_pr_sail_ci() {
    let _lock = setup_env(vec![("SAILCI", ""), ("SAIL_PULL_REQUEST_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
}

#[test]
fn get_no_pr_semaphore() {
    let _lock = setup_env(vec![("SEMAPHORE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
}

#[test]
fn get_pr_semaphore() {
    let _lock = setup_env(vec![("SEMAPHORE", ""), ("PULL_REQUEST_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
}

#[test]
fn get_no_pr_shippable() {
    let _lock = setup_env(vec![("SHIPPABLE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
}

#[test]
fn get_no_pr2_shippable() {
    let _lock = setup_env(vec![("SHIPPABLE", ""), ("IS_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
}

#[test]
fn get_pr_shippable() {
    let _lock = setup_env(vec![("SHIPPABLE", ""), ("IS_PULL_REQUEST", "true")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
}

#[test]
fn get_no_pr_solano_ci() {
    let _lock = setup_env(vec![("TDDIUM", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
}

#[test]
fn get_pr_solano_ci() {
    let _lock = setup_env(vec![("TDDIUM", ""), ("TDDIUM_PR_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
}

#[test]
fn get_strider_cd() {
    let _lock = setup_env(vec![("STRIDER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::StriderCD);
    assert_eq!(info.name.unwrap(), "Strider CD");
}

#[test]
fn get_taskcluster() {
    let _lock = setup_env(vec![("TASK_ID", ""), ("RUN_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TaskCluster);
    assert_eq!(info.name.unwrap(), "TaskCluster");
}

#[test]
fn get_partial1_taskcluster() {
    let _lock = setup_env(vec![("TASK_ID", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_partial2_taskcluster() {
    let _lock = setup_env(vec![("RUN_ID", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_teamcity() {
    let _lock = setup_env(vec![("TEAMCITY_VERSION", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TeamCity);
    assert_eq!(info.name.unwrap(), "TeamCity");
}

#[test]
fn get_no_pr_travis() {
    let _lock = setup_env(vec![("TRAVIS", ""), ("TRAVIS_PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
}

#[test]
fn get_pr_travis() {
    let _lock = setup_env(vec![("TRAVIS", ""), ("TRAVIS_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
}

#[test]
fn get_pr2_travis() {
    let _lock = setup_env(vec![("TRAVIS", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
}

#[test]
fn get_ziet_now() {
    let _lock = setup_env(vec![("NOW_BUILDER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::ZEITNow);
    assert_eq!(info.name.unwrap(), "ZEIT Now");
}

#[test]
fn get_ci_unknown_1() {
    let _lock = setup_env(vec![("CI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_ci_unknown_2() {
    let _lock = setup_env(vec![("CONTINUOUS_INTEGRATION", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_ci_unknown_3() {
    let _lock = setup_env(vec![("BUILD_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_ci_unknown_4() {
    let _lock = setup_env(vec![("RUN_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}
