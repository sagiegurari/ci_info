use super::*;

use std::env;

fn setup_env(vars: Vec<(&str, &str)>) {
    envmnt::remove_all(&vec![
        "APPVEYOR",
        "APPVEYOR_PULL_REQUEST_NUMBER",
        "SYSTEM_TEAMFOUNDATIONCOLLECTIONURI",
        "SYSTEM_PULLREQUEST_PULLREQUESTID",
        "bamboo_planKey",
        "BITBUCKET_COMMIT",
        "BITBUCKET_PR_ID",
        "BITRISE_IO",
        "BITRISE_PULL_REQUEST",
        "BUDDY_WORKSPACE_ID",
        "BUDDY_EXECUTION_PULL_REQUEST_ID",
        "BUILDKITE",
        "BUILDKITE_PULL_REQUEST",
        "CIRCLECI",
        "CIRCLE_PULL_REQUEST",
        "CIRRUS_CI",
        "CIRRUS_PR",
        "CODEBUILD_BUILD_ARN",
        "CI_NAME",
        "DRONE",
        "DRONE_BUILD_EVENT",
        "pull_request",
        "DSARI",
        "GITLAB_CI",
        "GO_PIPELINE_LABEL",
        "HUDSON_URL",
        "JENKINS_URL",
        "BUILD_ID",
        "ghprbPullId",
        "CHANGE_ID",
        "MAGNUM",
        "NETLIFY_BUILD_BASE",
        "PULL_REQUEST",
        "NEVERCODE",
        "NEVERCODE_PULL_REQUEST",
        "RENDER",
        "SAILCI",
        "SAIL_PULL_REQUEST_NUMBER",
        "SEMAPHORE",
        "PULL_REQUEST_NUMBER",
        "SHIPPABLE",
        "IS_PULL_REQUEST",
        "TDDIUM",
        "TDDIUM_PR_ID",
        "STRIDER",
        "TASK_ID",
        "RUN_ID",
        "TEAMCITY_VERSION",
        "TRAVIS",
        "TRAVIS_PULL_REQUEST",
        "NOW_BUILDER",
        "CI",
        "CONTINUOUS_INTEGRATION",
        "BUILD_NUMBER",
    ]);

    for env_var in vars {
        env::set_var(env_var.0, env_var.1);
    }
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

    assert_eq!(info.name.is_some(), info.vendor.is_some());
}

#[test]
fn get_no_pr_appveyor() {
    setup_env(vec![("APPVEYOR", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
}

#[test]
fn get_pr_appveyor() {
    setup_env(vec![("APPVEYOR", ""), ("APPVEYOR_PULL_REQUEST_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AppVeyor);
    assert_eq!(info.name.unwrap(), "AppVeyor");
}

#[test]
fn get_no_pr_azure_piplines() {
    setup_env(vec![("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::AzurePipelines);
    assert_eq!(info.name.unwrap(), "Azure Pipelines");
}

#[test]
fn get_pr_azure_piplines() {
    setup_env(vec![
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
    setup_env(vec![("bamboo_planKey", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Bamboo);
    assert_eq!(info.name.unwrap(), "Bamboo");
}

#[test]
fn get_no_pr_bitbucket_piplines() {
    setup_env(vec![("BITBUCKET_COMMIT", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
}

#[test]
fn get_pr_bitbucket_piplines() {
    setup_env(vec![("BITBUCKET_COMMIT", ""), ("BITBUCKET_PR_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::BitbucketPipelines);
    assert_eq!(info.name.unwrap(), "Bitbucket Pipelines");
}

#[test]
fn get_no_pr_bitrise() {
    setup_env(vec![("BITRISE_IO", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
}

#[test]
fn get_pr_bitrise() {
    setup_env(vec![("BITRISE_IO", ""), ("BITRISE_PULL_REQUEST", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Bitrise);
    assert_eq!(info.name.unwrap(), "Bitrise");
}

#[test]
fn get_no_pr_buddy() {
    setup_env(vec![("BUDDY_WORKSPACE_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buddy);
    assert_eq!(info.name.unwrap(), "Buddy");
}

#[test]
fn get_pr_buddy() {
    setup_env(vec![
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
    setup_env(vec![("BUILDKITE", ""), ("BUILDKITE_PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
}

#[test]
fn get_pr_buildkite() {
    setup_env(vec![("BUILDKITE", ""), ("BUILDKITE_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
}

#[test]
fn get_pr2_buildkite() {
    setup_env(vec![("BUILDKITE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Buildkite);
    assert_eq!(info.name.unwrap(), "Buildkite");
}

#[test]
fn get_no_pr_circle_ci() {
    setup_env(vec![("CIRCLECI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
}

#[test]
fn get_pr_circle_ci() {
    setup_env(vec![("CIRCLECI", ""), ("CIRCLE_PULL_REQUEST", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CircleCI);
    assert_eq!(info.name.unwrap(), "CircleCI");
}

#[test]
fn get_no_pr_cirrus_ci() {
    setup_env(vec![("CIRRUS_CI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
}

#[test]
fn get_pr_cirrus_ci() {
    setup_env(vec![("CIRRUS_CI", ""), ("CIRRUS_PR", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::CirrusCI);
    assert_eq!(info.name.unwrap(), "Cirrus CI");
}

#[test]
fn get_aws_codebuild() {
    setup_env(vec![("CODEBUILD_BUILD_ARN", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::AWSCodeBuild);
    assert_eq!(info.name.unwrap(), "AWS CodeBuild");
}

#[test]
fn get_codeship() {
    setup_env(vec![("CI_NAME", "codeship")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Codeship);
    assert_eq!(info.name.unwrap(), "Codeship");
}

#[test]
fn get_no_pr_drone() {
    setup_env(vec![("DRONE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
}

#[test]
fn get_no_pr2_drone() {
    setup_env(vec![("DRONE", ""), ("DRONE_BUILD_EVENT", "test")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
}

#[test]
fn get_pr_drone() {
    setup_env(vec![("DRONE", ""), ("DRONE_BUILD_EVENT", "pull_request")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Drone);
    assert_eq!(info.name.unwrap(), "Drone");
}

#[test]
fn get_dsari() {
    setup_env(vec![("DSARI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::DSARI);
    assert_eq!(info.name.unwrap(), "dsari");
}

#[test]
fn get_gitlab_ci() {
    setup_env(vec![("GITLAB_CI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GitLabCI);
    assert_eq!(info.name.unwrap(), "GitLab CI");
}

#[test]
fn get_gocd() {
    setup_env(vec![("GO_PIPELINE_LABEL", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::GoCD);
    assert_eq!(info.name.unwrap(), "GoCD");
}

#[test]
fn get_hudson() {
    setup_env(vec![("HUDSON_URL", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::Hudson);
    assert_eq!(info.name.unwrap(), "Hudson");
}

#[test]
fn get_no_pr_jenkins() {
    setup_env(vec![("JENKINS_URL", ""), ("BUILD_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Jenkins);
    assert_eq!(info.name.unwrap(), "Jenkins");
}

#[test]
fn get_partial1_jenkins() {
    setup_env(vec![("JENKINS_URL", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_partial2_jenkins() {
    setup_env(vec![("BUILD_ID", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_pr_jenkins() {
    setup_env(vec![
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
    setup_env(vec![
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
    setup_env(vec![("MAGNUM", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::MagnumCI);
    assert_eq!(info.name.unwrap(), "Magnum CI");
}

#[test]
fn get_no_pr_netlify_ci() {
    setup_env(vec![("NETLIFY_BUILD_BASE", ""), ("PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
}

#[test]
fn get_pr_netlify_ci() {
    setup_env(vec![("NETLIFY_BUILD_BASE", ""), ("PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
}

#[test]
fn get_pr2_netlify_ci() {
    setup_env(vec![("NETLIFY_BUILD_BASE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::NetlifyCI);
    assert_eq!(info.name.unwrap(), "Netlify CI");
}

#[test]
fn get_no_pr_nevercode_ci() {
    setup_env(vec![("NEVERCODE", ""), ("NEVERCODE_PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
}

#[test]
fn get_pr_nevercode_ci() {
    setup_env(vec![("NEVERCODE", ""), ("NEVERCODE_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
}

#[test]
fn get_pr2_nevercode_ci() {
    setup_env(vec![("NEVERCODE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Nevercode);
    assert_eq!(info.name.unwrap(), "Nevercode");
}

#[test]
fn get_no_pr_render_ci() {
    setup_env(vec![("RENDER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
}

#[test]
fn get_pr_render_ci() {
    setup_env(vec![("RENDER", ""), ("IS_PULL_REQUEST", "true")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Render);
    assert_eq!(info.name.unwrap(), "Render");
}

#[test]
fn get_no_pr_sail_ci() {
    setup_env(vec![("SAILCI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
}

#[test]
fn get_pr_sail_ci() {
    setup_env(vec![("SAILCI", ""), ("SAIL_PULL_REQUEST_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SailCI);
    assert_eq!(info.name.unwrap(), "Sail CI");
}

#[test]
fn get_no_pr_semaphore() {
    setup_env(vec![("SEMAPHORE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
}

#[test]
fn get_pr_semaphore() {
    setup_env(vec![("SEMAPHORE", ""), ("PULL_REQUEST_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Semaphore);
    assert_eq!(info.name.unwrap(), "Semaphore");
}

#[test]
fn get_no_pr_shippable() {
    setup_env(vec![("SHIPPABLE", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
}

#[test]
fn get_no_pr2_shippable() {
    setup_env(vec![("SHIPPABLE", ""), ("IS_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
}

#[test]
fn get_pr_shippable() {
    setup_env(vec![("SHIPPABLE", ""), ("IS_PULL_REQUEST", "true")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::Shippable);
    assert_eq!(info.name.unwrap(), "Shippable");
}

#[test]
fn get_no_pr_solano_ci() {
    setup_env(vec![("TDDIUM", "")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
}

#[test]
fn get_pr_solano_ci() {
    setup_env(vec![("TDDIUM", ""), ("TDDIUM_PR_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::SolanoCI);
    assert_eq!(info.name.unwrap(), "Solano CI");
}

#[test]
fn get_strider_cd() {
    setup_env(vec![("STRIDER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::StriderCD);
    assert_eq!(info.name.unwrap(), "Strider CD");
}

#[test]
fn get_taskcluster() {
    setup_env(vec![("TASK_ID", ""), ("RUN_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TaskCluster);
    assert_eq!(info.name.unwrap(), "TaskCluster");
}

#[test]
fn get_partial1_taskcluster() {
    setup_env(vec![("TASK_ID", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_partial2_taskcluster() {
    setup_env(vec![("RUN_ID", "")]);

    let info = get();

    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_teamcity() {
    setup_env(vec![("TEAMCITY_VERSION", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::TeamCity);
    assert_eq!(info.name.unwrap(), "TeamCity");
}

#[test]
fn get_no_pr_travis() {
    setup_env(vec![("TRAVIS", ""), ("TRAVIS_PULL_REQUEST", "false")]);

    let info = get();

    assert!(info.ci);
    assert!(!info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
}

#[test]
fn get_pr_travis() {
    setup_env(vec![("TRAVIS", ""), ("TRAVIS_PULL_REQUEST", "123")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
}

#[test]
fn get_pr2_travis() {
    setup_env(vec![("TRAVIS", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
}

#[test]
fn get_ziet_now() {
    setup_env(vec![("NOW_BUILDER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert_eq!(info.vendor.unwrap(), Vendor::ZEITNow);
    assert_eq!(info.name.unwrap(), "ZEIT Now");
}

#[test]
fn get_ci_unknown_1() {
    setup_env(vec![("CI", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_ci_unknown_2() {
    setup_env(vec![("CONTINUOUS_INTEGRATION", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_ci_unknown_3() {
    setup_env(vec![("BUILD_NUMBER", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}

#[test]
fn get_ci_unknown_4() {
    setup_env(vec![("RUN_ID", "")]);

    let info = get();

    assert!(info.ci);
    assert!(info.pr.is_none());
    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
}
