//! # env
//!
//! Environment utility functions.
//!

#[cfg(test)]
#[path = "./env_test.rs"]
mod env_test;

use envmnt;

pub(crate) fn is_env_equal(key: &str, value: &str) -> bool {
    envmnt::is_equal(key, value)
}

pub(crate) fn is_env_defined(key: &str) -> bool {
    envmnt::exists(key)
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
