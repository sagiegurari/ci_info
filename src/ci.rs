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

/// Loads and returns the CI info of the current environment.
pub(crate) fn get() -> CiInfo {
    let mut info = CiInfo::new();
    let vendor_config_list = config::create();

    for vendor_config in vendor_config_list.iter() {
        let found = match vendor_config.ci_env {
            EnvValue::Exists(ref key) => envmnt::exists(key),
            EnvValue::AllExists(ref keys) => envmnt::is_all_exists(keys),
            EnvValue::AnyExists(ref keys) => envmnt::is_any_exists(keys),
            EnvValue::Value(ref key, ref value) => envmnt::is_equal(key, value),
            EnvValue::NotEqual(ref key, ref value) => !envmnt::is_equal(key, value),
            EnvValue::Contains(ref key, ref value) => envmnt::contains_ignore_case(key, value),
        };

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
                    let is_pr = match env_value {
                        EnvValue::Exists(ref key) => envmnt::exists(key),
                        EnvValue::AllExists(ref keys) => envmnt::is_all_exists(keys),
                        EnvValue::AnyExists(ref keys) => envmnt::is_any_exists(keys),
                        EnvValue::Value(ref key, ref value) => envmnt::is_equal(key, value),
                        EnvValue::NotEqual(ref key, ref value) => !envmnt::is_equal(key, value),
                        EnvValue::Contains(ref key, ref value) => {
                            envmnt::contains_ignore_case(key, value)
                        }
                    };

                    Some(is_pr)
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
