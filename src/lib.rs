#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # ci_info
//!
//! Provides current CI environment information.
//!
//! This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current CI environment.<br>
//! The code is based on the [ci-info](https://github.com/watson/ci-info) npm module.
//!
//! # Examples
//!
//! ## Get CI environment information
//!
//! ```
//! fn main() {
//!     // Just check if a CI environment is detected.
//!     let ci = ci_info::is_ci();
//!     println!("Is CI: {}", ci);
//!
//!     // Get CI environment information
//!     let info = ci_info::get();
//!     println!("Is CI: {}", info.ci);
//!     if let Some(vendor) = info.vendor {
//!         println!("Vendor: {:#?}", vendor);
//!         println!("Name: {:#?}", info.name.unwrap());
//!     }
//!     if let Some(pr) = info.pr {
//!         println!("Is PR: {:#?}", pr);
//!     }
//!     if let Some(branch_name) = info.branch_name {
//!         println!("Branch Name: {:#?}", branch_name);
//!     }
//! }
//! ```
//!
//! ## Check if a CI environment is detected
//!
//! ```
//! fn main() {
//!     let ci = ci_info::is_ci();
//!
//!     println!("Is CI: {}", ci);
//! }
//! ```
//!
//! ## Mocking CI environment
//!
//! ```
//! use ci_info::types::{CiInfo, Vendor};
//!
//! fn main() {
//!     // create the CI info manually
//!     let mut mock_info = CiInfo::new();
//!     mock_info.vendor = Some(Vendor::TravisCI);
//!     mock_info.ci = true;
//!     mock_info.pr = Some(true);
//!     mock_info.branch_name = Some("dev_branch".to_string());
//!
//!     // mock environment
//!     ci_info::mock_ci(&mock_info);
//!
//!     let info = ci_info::get();
//!
//!     assert!(info.ci);
//!     assert!(info.pr.unwrap());
//!     assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
//!     assert_eq!(info.name.unwrap(), "Travis CI");
//!     assert_eq!(info.branch_name.unwrap(), "dev_branch");
//!
//!     // clear CI environment
//!     mock_info = CiInfo::new();
//!     ci_info::mock_ci(&mock_info);
//!
//!     let info = ci_info::get();
//!
//!     assert!(!info.ci);
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! ci_info = "*"
//! ```
//!
//! There is optional `serde` support that can be enabled via the `serde-1` feature:
//!
//! ```ini
//! [dependencies]
//! ci_info = { version = "*", features = ["serde-1"] }
//! ```
//!
//! ## Iterating Over Vendor Variants (Optional Feature)
//!
//! When `iter` feature is enabled, you can iterate over all known CI vendor variants:
//!
//! ```toml
//! [dependencies]
//! ci_info = { version = "*", features = ["iter"] }
//! ```
//!
//! Example usage:
//!
//! ```rust
//! # #[cfg(feature = "iter")]
//! # {
//! use ci_info::types::Vendor;
//! use strum::IntoEnumIterator;
//!
//! // List all supported CI vendors
//! for vendor in Vendor::iter() {
//!     println!("Supported CI vendor: {:?}", vendor);
//! }
//!
//! // Count vendors
//! let vendor_count = Vendor::iter().count();
//! println!("Total CI vendors supported: {}", vendor_count);
//!
//! // Filter for specific vendors
//! let cloud_vendors: Vec<_> = Vendor::iter()
//!     .filter(|v| matches!(v,
//!         Vendor::GitHubActions |
//!         Vendor::GitLabCI |
//!         Vendor::CircleCI
//!     ))
//!     .collect();
//! # }
//! ```
//!
//! This feature uses [strum](https://crates.io/crates/strum) library's `EnumIter` derive macro.
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/ci_info/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/ci_info/blob/master/LICENSE) open source license.
//!

#[cfg(feature = "serde-1")]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[path = "./config_test.rs"]
mod config_test;
#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;
#[cfg(test)]
#[path = "./test_env.rs"]
mod test_env;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod ci;
mod config;
mod mock;
pub mod types;

use crate::types::CiInfo;

/// Loads and returns the CI info of the current environment.
///
/// # Example
///
/// ```
/// fn main() {
///     // Just check if a CI environment is detected.
///     let ci = ci_info::is_ci();
///     println!("Is CI: {}", ci);
///
///     // Get CI environment information
///     let info = ci_info::get();
///     println!("Is CI: {}", info.ci);
///     if info.vendor.is_some() {
///         println!("Vendor: {:#?}", info.vendor.unwrap());
///         println!("Name: {:#?}", info.name.unwrap());
///     }
///     if info.pr.is_some() {
///         println!("Is PR: {:#?}", info.pr.unwrap());
///     }
///     if info.branch_name.is_some() {
///         println!("Branch Name: {:#?}", info.branch_name.unwrap());
///     }
/// }
/// ```
pub fn get() -> CiInfo {
    ci::get()
}

/// Returns true if a CI environment is detected.
///
/// # Example
///
/// ```
/// fn main() {
///     let ci = ci_info::is_ci();
///
///     println!("Is CI: {}", ci);
/// }
/// ```
pub fn is_ci() -> bool {
    ci::is_ci()
}

/// This function will modify the current environment variables to mock the
/// requested CI vendor.
///
/// # Example
///
/// ```
/// use ci_info::types::{CiInfo, Vendor};
///
/// fn main() {
///     // create the CI info manually
///     let mut mock_info = CiInfo::new();
///     mock_info.vendor = Some(Vendor::TravisCI);
///     mock_info.ci = true;
///     mock_info.pr = Some(true);
///     mock_info.branch_name = Some("dev_branch".to_string());
///
///     // mock environment
///     ci_info::mock_ci(&mock_info);
///
///     let info = ci_info::get();
///
///     assert!(info.ci);
///     assert!(info.pr.unwrap());
///     assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
///     assert_eq!(info.name.unwrap(), "Travis CI");
///     assert_eq!(info.branch_name.unwrap(), "dev_branch");
///
///     // clear CI environment
///     mock_info = CiInfo::new();
///     ci_info::mock_ci(&mock_info);
///
///     let info = ci_info::get();
///
///     assert!(!info.ci);
/// }
/// ```
pub fn mock_ci(info: &CiInfo) {
    mock::mock_ci_info(info);
}
