use super::*;
use crate::test_env::{clear_env, get_with_env, TestVendorConfig};
use crate::types::EnvValue;
use doc_comment as _;

#[test]
fn get_test() {
    let info = get_with_env(TestVendorConfig {
        ci_env: EnvValue::Exists("CI_INFO_LIB_GET_TEST".to_string()),
        pr_env: None,
        branch_name_env: None,
    });

    assert_eq!(info.ci, info.vendor.is_some());
}

#[test]
fn is_ci_test() {
    let lock = clear_env();
    let info = get();
    let ci = is_ci();
    drop(lock);
    assert_eq!(info.ci, ci);
}
