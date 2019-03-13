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
fn is_any_env_defined_empty() {
    let found = is_any_env_defined(&vec![]);

    assert!(!found);
}

#[test]
fn is_any_env_defined_found() {
    env::set_var("ANY_ENV_VAR_FOUND_VALUE", "EMPTY");

    let found = is_any_env_defined(&vec![
        "ENV_VAR_NOT_FOUND".to_string(),
        "ANY_ENV_VAR_FOUND_VALUE".to_string(),
    ]);

    assert!(found);
}

#[test]
fn is_any_env_defined_not_found() {
    let found = is_any_env_defined(&vec!["ENV_VAR_NOT_FOUND".to_string()]);

    assert!(!found);
}

#[test]
fn is_all_env_defined_empty() {
    let found = is_all_env_defined(&vec![]);

    assert!(!found);
}

#[test]
fn is_all_env_defined_found() {
    env::set_var("ALL_ENV_VAR_FOUND_VALUE1", "EMPTY");
    env::set_var("ALL_ENV_VAR_FOUND_VALUE2", "EMPTY");

    let found = is_all_env_defined(&vec![
        "ALL_ENV_VAR_FOUND_VALUE1".to_string(),
        "ALL_ENV_VAR_FOUND_VALUE2".to_string(),
    ]);

    assert!(found);
}

#[test]
fn is_all_env_defined_partial() {
    env::set_var("ALL_ENV_VAR_FOUND_VALUE1", "EMPTY");

    let found = is_all_env_defined(&vec![
        "ALL_ENV_VAR_FOUND_VALUE1".to_string(),
        "ENV_VAR_NOT_FOUND".to_string(),
    ]);

    assert!(!found);
}

#[test]
fn is_all_env_defined_not_found() {
    let found = is_all_env_defined(&vec![
        "ENV_VAR_NOT_FOUND1".to_string(),
        "ENV_VAR_NOT_FOUND2".to_string(),
    ]);

    assert!(!found);
}
