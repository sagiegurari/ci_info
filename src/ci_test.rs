use super::*;

use std::env;

fn setup_env(
    key: &str,
    value: &str,
) {
    env::remove_var("TRAVIS");
    env::remove_var("CIRCLECI");
    env::remove_var("GITLAB_CI");
    env::remove_var("APPVEYOR");
    env::remove_var("CI_NAME");
    env::remove_var("DRONE");
    env::remove_var("MAGNUM");
    env::remove_var("SEMAPHORE");
    env::remove_var("JENKINS_URL");
    env::remove_var("bamboo_planKey");
    env::remove_var("TF_BUILD");
    env::remove_var("TEAMCITY_VERSION");
    env::remove_var("BUILDKITE");
    env::remove_var("HUDSON_URL");
    env::remove_var("TASK_ID");
    env::remove_var("RUN_ID");
    env::remove_var("GO_PIPELINE_LABEL");
    env::remove_var("BITBUCKET_COMMIT");
    env::remove_var("CODEBUILD_BUILD_ARN");

    env::remove_var("CI");
    env::remove_var("CONTINUOUS_INTEGRATION");
    env::remove_var("BUILD_NUMBER");

    env::set_var(key, value);
}

#[test]
fn is_env_equal_same() {
    env::set_var("CI_TEST_SAME", "YES");

    let same = is_env_equal("CI_TEST_SAME", "YES");

    assert!(same);
}

#[test]
fn is_env_equal_different() {
    env::set_var("CI_TEST_DIFF", "NO");

    let same = is_env_equal("CI_TEST_DIFF", "YES");

    assert!(!same);
}

#[test]
fn is_env_equal_not_defined() {
    let same = is_env_equal("CI_TEST_NOT_DEFINED", "BAD");

    assert!(!same);
}

#[test]
fn is_env_defined_found() {
    env::set_var("ENV_VAR_FOUND_VALUE", "EMPTY");

    let found = is_env_defined("ENV_VAR_FOUND_VALUE");

    assert!(found);
}

#[test]
fn is_env_defined_empty() {
    env::set_var("ENV_VAR_FOUND_EMPTY", "");

    let found = is_env_defined("ENV_VAR_FOUND_EMPTY");

    assert!(found);
}

#[test]
fn is_env_defined_not_found() {
    let found = is_env_defined("ENV_VAR_NOT_FOUND");

    assert!(!found);
}

#[test]
fn is_ci_test() {
    let info = get();
    let ci = is_ci();

    assert_eq!(info.ci, ci);
}

#[test]
fn get_test() {
    let info = get();

    assert_eq!(info.ci, info.vendor.is_some());
}

#[test]
fn get_travis() {
    setup_env("TRAVIS", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::TRAVIS);
}

#[test]
fn get_circle() {
    setup_env("CIRCLECI", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::CIRCLE);
}

#[test]
fn get_gitlab() {
    setup_env("GITLAB_CI", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::GITLAB);
}

#[test]
fn get_appveyor() {
    setup_env("APPVEYOR", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::APPVEYOR);
}

#[test]
fn get_codeship() {
    setup_env("CI_NAME", "codeship");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::CODESHIP);
}

#[test]
fn get_codeship_wrong_value() {
    setup_env("CI_NAME", "test");

    let info = get();

    assert!(!info.ci);
    assert!(info.vendor.is_none());
}

#[test]
fn get_drone() {
    setup_env("DRONE", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::DRONE);
}

#[test]
fn get_magnum() {
    setup_env("MAGNUM", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::MAGNUM);
}

#[test]
fn get_semaphore() {
    setup_env("SEMAPHORE", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::SEMAPHORE);
}

#[test]
fn get_jenkins() {
    setup_env("JENKINS_URL", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::JENKINS);
}

#[test]
fn get_bamboo() {
    setup_env("bamboo_planKey", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::BAMBOO);
}

#[test]
fn get_tfs() {
    setup_env("TF_BUILD", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::TFS);
}

#[test]
fn get_teamcity() {
    setup_env("TEAMCITY_VERSION", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::TEAMCITY);
}

#[test]
fn get_buildkite() {
    setup_env("BUILDKITE", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::BUILDKITE);
}

#[test]
fn get_hudson() {
    setup_env("HUDSON_URL", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::HUDSON);
}

#[test]
fn get_taskcluster_taskid() {
    setup_env("TASK_ID", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::TASKCLUSTER);
}

#[test]
fn get_taskcluster_runid() {
    setup_env("RUN_ID", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::TASKCLUSTER);
}

#[test]
fn get_gocd() {
    setup_env("GO_PIPELINE_LABEL", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::GOCD);
}

#[test]
fn get_bitbucket() {
    setup_env("BITBUCKET_COMMIT", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::BITBUCKET);
}

#[test]
fn get_codebuild() {
    setup_env("CODEBUILD_BUILD_ARN", "");

    let info = get();

    assert!(info.ci);
    assert_eq!(info.vendor.unwrap(), Vendor::CODEBUILD);
}

#[test]
fn get_none() {
    setup_env("BAD", "");

    let info = get();

    assert!(!info.ci);
    assert!(info.vendor.is_none());
}
