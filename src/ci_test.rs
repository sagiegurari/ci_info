use super::*;

use std::env;

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
fn get_test() {
    let info = get();

    assert_eq!(info.ci, info.vendor.is_some());
}
