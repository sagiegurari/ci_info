use envmnt;
use lazy_static::lazy_static;
use std::env;
use std::sync::{Mutex, MutexGuard};

pub(crate) struct MutexInner;

lazy_static! {
    pub(crate) static ref ENVLOCK: Mutex<MutexInner> = { Mutex::new(MutexInner) };
}

pub(crate) fn get_with_env(vars: Vec<(&str, &str)>) -> crate::CiInfo {
    let _lock = setup_env(vars);
    crate::get()
}

#[inline(always)]
pub(crate) fn setup_env(vars: Vec<(&str, &str)>) -> MutexGuard<'static, MutexInner> {
    let lock = ENVLOCK.lock().unwrap();
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
        "GITHUB_ACTIONS",
        "GITHUB_EVENT_NAME",
        "GITLAB_CI",
        "GO_PIPELINE_LABEL",
        "NODE",
        "HUDSON_URL",
        "JENKINS_URL",
        "BUILD_ID",
        "ghprbPullId",
        "CHANGE_ID",
        "MAGNUM",
        "NETLIFY",
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
        "APPVEYOR_REPO_BRANCH",
        "BUILD_SOURCEBRANCHNAME",
        "BITBUCKET_BRANCH",
        "BITRISE_GIT_BRANCH",
        "BUDDY_EXECUTION_BRANCH",
        "CIRCLE_BRANCH",
        "CIRRUS_BRANCH",
        "CI_BRANCH",
        "CI_COMMIT_REF_NAME",
        "HEROKU_TEST_RUN_BRANCH",
        "BRANCH_NAME",
        "BRANCH",
        "NEVERCODE_BRANCH",
        "SEMAPHORE_GIT_BRANCH",
        "TRAVIS_BRANCH",
    ]);

    for env_var in vars {
        env::set_var(env_var.0, env_var.1);
    }
    return lock;
}
