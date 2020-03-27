use crate::config;
use crate::mock;
use crate::types::{CiInfo, EnvValue, Vendor, VendorConfig};
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

pub(crate) struct MutexInner;

lazy_static! {
    pub(crate) static ref ENVLOCK: Mutex<MutexInner> = { Mutex::new(MutexInner) };
}

/// Vendor detection info
#[derive(Debug, Clone)]
pub(crate) struct TestVendorConfig {
    /// CI env var name
    pub(crate) ci_env: EnvValue,
    /// PR env var name
    pub(crate) pr_env: Option<EnvValue>,
    /// Branch name env var name
    pub(crate) branch_name_env: Option<String>,
}

pub(crate) fn get_with_env(test_config: TestVendorConfig) -> CiInfo {
    let _lock = setup_env(test_config);
    crate::get()
}

#[inline(always)]
pub(crate) fn clear_env() -> MutexGuard<'static, MutexInner> {
    let lock = ENVLOCK.lock().unwrap();
    let vendor_config_list = config::create();
    mock::clear_env(&vendor_config_list);
    return lock;
}

#[inline(always)]
fn setup_env(test_config: TestVendorConfig) -> MutexGuard<'static, MutexInner> {
    let lock = ENVLOCK.lock().unwrap();

    let vendor_config = VendorConfig {
        name: "".to_string(),
        vendor: Vendor::Unknown,
        ci_env: test_config.ci_env,
        pr_env: test_config.pr_env,
        branch_name_env: test_config.branch_name_env,
    };

    let vendor_config_list = config::create();
    mock::clear_env(&vendor_config_list);
    mock::set_env_for_config(&vendor_config, None);

    return lock;
}
