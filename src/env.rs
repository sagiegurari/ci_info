//! # env
//!
//! Environment utility functions.
//!

#[cfg(test)]
#[path = "./env_test.rs"]
mod env_test;

use std::env;

pub(crate) fn is_env_equal(key: &str, value: &str) -> bool {
    match env::var(key) {
        Ok(current_value) => current_value == value,
        _ => false,
    }
}

pub(crate) fn is_env_defined(key: &str) -> bool {
    match env::var(key) {
        Ok(_) => true,
        _ => false,
    }
}

pub(crate) fn is_any_env_defined(keys: &Vec<String>) -> bool {
    let mut found = false;

    for key in keys.iter() {
        found = is_env_defined(key);

        if found {
            break;
        }
    }

    found
}

pub(crate) fn is_all_env_defined(keys: &Vec<String>) -> bool {
    let mut found = false;

    for key in keys.iter() {
        found = is_env_defined(key);

        if !found {
            break;
        }
    }

    found
}
