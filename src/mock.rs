//! # mock
//!
//! Enables to mock a specific CI vendor by setting the relevant environment variables.
//!

use crate::config;
use crate::types::{CiInfo, EnvValue, VendorConfig};

fn get_env_keys(env_info: &Option<EnvValue>) -> Vec<String> {
    match env_info {
        Some(info) => match info {
            EnvValue::Exists(ref key) => vec![key.to_string()],
            EnvValue::AllExists(ref keys) => keys.clone(),
            EnvValue::AnyExists(ref keys) => keys.clone(),
            EnvValue::Value(ref key, ref _value) => vec![key.to_string()],
            EnvValue::NotEqual(ref key, ref _value) => vec![key.to_string()],
            EnvValue::Contains(ref key, ref _value) => vec![key.to_string()],
            EnvValue::NotEmpty(ref key) => vec![key.to_string()],
        },
        None => vec![],
    }
}

pub(crate) fn clear_env(vendor_config_list: &Vec<VendorConfig>) {
    for vendor_config in vendor_config_list.iter() {
        let mut keys = get_env_keys(&Some(vendor_config.ci_env.clone()));
        envmnt::remove_all(&keys);

        keys = get_env_keys(&vendor_config.pr_env);
        envmnt::remove_all(&keys);

        if let Some(ref key) = vendor_config.branch_name_env {
            envmnt::remove(key)
        };
    }
}

fn set_mock_env_key_value_pairs(env_info: &Option<EnvValue>, test_value: &str) {
    let key_value_pairs = match env_info {
        Some(info) => match info {
            EnvValue::Exists(ref key) => vec![(key.to_string(), test_value.to_string())],
            EnvValue::AllExists(ref keys) => {
                let mut key_values = vec![];

                for key in keys {
                    key_values.push((key.to_string(), test_value.to_string()))
                }

                key_values
            }
            EnvValue::AnyExists(ref keys) => vec![(keys[0].to_string(), test_value.to_string())],
            EnvValue::Value(ref key, ref value) => vec![(key.to_string(), value.to_string())],
            EnvValue::NotEqual(ref key, ref _value) => {
                vec![(key.to_string(), test_value.to_string())]
            }
            EnvValue::Contains(ref key, ref value) => vec![(key.to_string(), value.to_string())],
            EnvValue::NotEmpty(ref key) => vec![(key.to_string(), test_value.to_string())],
        },
        None => vec![],
    };

    for key_value_pair in key_value_pairs {
        envmnt::set(key_value_pair.0, key_value_pair.1);
    }
}

pub(crate) fn set_env_for_config(vendor_config: &VendorConfig, branch_name: Option<String>) {
    set_mock_env_key_value_pairs(&Some(vendor_config.ci_env.clone()), "mock_ci");
    set_mock_env_key_value_pairs(&vendor_config.pr_env, "mock_pr");
    if let Some(ref key) = vendor_config.branch_name_env {
        envmnt::set(key, branch_name.unwrap_or("mock_branch".to_string()))
    };
}

fn set_env_for_info(info: &CiInfo, vendor_config_list: Vec<VendorConfig>) {
    if info.ci {
        match info.vendor {
            Some(ref vendor) => {
                for vendor_config in vendor_config_list.iter() {
                    if vendor_config.vendor == *vendor {
                        let mut mock_vendor_config = vendor_config.clone();

                        match info.pr {
                            Some(value) => {
                                if !value {
                                    mock_vendor_config.pr_env = None;
                                }
                            }
                            None => mock_vendor_config.pr_env = None,
                        }

                        if info.branch_name.is_none() {
                            mock_vendor_config.branch_name_env = None;
                        }

                        set_env_for_config(&mock_vendor_config, info.branch_name.clone());

                        break;
                    }
                }
            }
            None => (),
        }
    }
}

pub(crate) fn mock_ci_info(info: &CiInfo) {
    let vendor_config_list = config::create();

    // clear current env
    clear_env(&vendor_config_list);

    set_env_for_info(info, vendor_config_list);
}
