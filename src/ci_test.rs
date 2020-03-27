use super::*;
use crate::test_env::{clear_env, get_with_env, TestVendorConfig};
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
    let lock = clear_env();
    let info = get();
    let ci = is_ci();
    drop(lock);
    assert_eq!(info.ci, ci);
}

#[test]
fn get_test() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CI_INFO_GET_TEST".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert_eq!(info.ci, info.vendor.is_some());
}
