#![deny(
    absolute_path_starting_with_module, anonymous_parameters, box_pointers, const_err, dead_code,
    deprecated, elided_lifetime_in_path, exceeding_bitshifts,
    illegal_floating_point_literal_pattern, improper_ctypes, incoherent_fundamental_impls,
    invalid_type_param_default, late_bound_lifetime_arguments, legacy_constructor_visibility,
    legacy_directory_ownership, legacy_imports, missing_copy_implementations,
    missing_debug_implementations, missing_docs, missing_fragment_specifier, mutable_transmutes,
    no_mangle_const_items, no_mangle_generic_items, non_camel_case_types,
    non_shorthand_field_patterns, non_snake_case, non_upper_case_globals, overflowing_literals,
    parenthesized_params_in_types_and_modules, path_statements, patterns_in_fns_without_body,
    plugin_as_library, private_in_public, private_no_mangle_fns, private_no_mangle_statics,
    pub_use_of_private_extern_crate, renamed_and_removed_lints, safe_extern_statics,
    safe_packed_borrows, single_use_lifetime, stable_features, trivial_casts, trivial_numeric_casts,
    type_alias_bounds, tyvar_behind_raw_pointer, unconditional_recursion, unions_with_drop_fields,
    unknown_crate_types, unreachable_code, unreachable_patterns, unreachable_pub, unsafe_code,
    unstable_features, unstable_name_collision, unused_allocation, unused_assignments,
    unused_attributes, unused_comparisons, unused_doc_comment, unused_extern_crates,
    unused_features, unused_import_braces, unused_imports, unused_macros, unused_must_use,
    unused_mut, unused_parens, unused_qualifications, unused_results, unused_unsafe,
    unused_variables, variant_size_differences, while_true
)]
#![warn(unknown_lints)]
#![allow(
    bare_trait_object, box_pointers, elided_lifetime_in_path, missing_debug_implementations,
    single_use_lifetime, trivial_casts, unnecessary_extern_crate, unused_results,
    variant_size_differences, warnings
)]
#![cfg_attr(feature = "clippy", feature(plugin))]

//! # ci_info
//!
//! Provides current CI environment information.
//!
//! This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current CI environment.<br>
//! The code is based on the [ci-info](https://github.com/watson/ci-info) npm module.
//!
//! # Examples
//!
//! ## Get CI environment information.
//!
//! ```
//! extern crate ci_info;
//!
//! fn main() {
//!     let info = ci_info::get();
//!
//!     println!("Is CI: {}", info.ci);
//!     if info.ci {
//!         println!("Vendor: {:#?}", info.vendor.unwrap());
//!     }
//! }
//! ```
//!
//! ## Check if a CI environment is detected.
//!
//! ```
//! extern crate ci_info;
//!
//! fn main() {
//!     let ci = ci_info::is_ci();
//!
//!     println!("Is CI: {}", ci);
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
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/ci_info/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/ci_info/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

mod ci;
pub mod types;

use types::CiInfo;

/// Loads and returns the CI info of the current environment.
///
/// # Example
///
/// ```
/// extern crate ci_info;
///
/// fn main() {
///     let info = ci_info::get();
///
///     println!("Is CI: {}", info.ci);
///     if info.ci {
///         println!("Vendor: {:#?}", info.vendor.unwrap());
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
/// extern crate ci_info;
///
/// fn main() {
///     let ci = ci_info::is_ci();
///
///     println!("Is CI: {}", ci);
/// }
/// ```
pub fn is_ci() -> bool {
    ci::is_ci()
}
