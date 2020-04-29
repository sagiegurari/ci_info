//! # ci
//!
//! Loads CI environment information.
//!

#[cfg(test)]
#[path = "./ci_test.rs"]
mod ci_test;

use crate::config;
use crate::types::{CiInfo, EnvValue, Vendor};
use envmnt;

fn validate(env_info: &EnvValue) -> bool {
    match env_info {
        EnvValue::Exists(ref key) => envmnt::exists(key),
        EnvValue::AllExists(ref keys) => envmnt::is_all_exists(keys),
        EnvValue::AnyExists(ref keys) => envmnt::is_any_exists(keys),
        EnvValue::Value(ref key, ref value) => envmnt::is_equal(key, value),
        EnvValue::NotEqual(ref key, ref value) => !envmnt::is_equal(key, value),
        EnvValue::Contains(ref key, ref value) => envmnt::contains_ignore_case(key, value),
        EnvValue::NotEmpty(ref key) => {
            let value = envmnt::get_or(key, "");
            !value.is_empty()
        }
    }
}

/// Loads and returns the CI info of the current environment.
pub(crate) fn get() -> CiInfo {
    let mut info = CiInfo::new();
    let vendor_config_list = config::create();

    for vendor_config in vendor_config_list.iter() {
        let found = validate(&vendor_config.ci_env);

        if found {
            info.ci = true;
            info.vendor = match vendor_config.vendor {
                Vendor::Unknown => None,
                _ => Some(vendor_config.vendor),
            };
            info.name = match vendor_config.vendor {
                Vendor::Unknown => None,
                _ => Some(vendor_config.name.clone()),
            };

            info.pr = match vendor_config.pr_env {
                Some(ref env_value) => {
                    let is_pr = validate(env_value);

                    Some(is_pr)
                }
                None => None,
            };

            info.branch_name = match vendor_config.branch_name_env {
                Some(ref env_key) => {
                    let value = envmnt::get_or(env_key, "");

                    if value.len() > 0 {
                        Some(value.to_string())
                    } else {
                        None
                    }
                }
                None => None,
            };

            break;
        }
    }

    info
}

/// Returns true if a CI environment is detected.
pub(crate) fn is_ci() -> bool {
    let info = get();

    info.ci
}
