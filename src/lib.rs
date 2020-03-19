#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    array_into_iter,
    bindings_with_variant_name,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    indirect_structural_match,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolon,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

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
//! fn main() {
//!     // Just check if a CI environment is detected.
//!     let ci = ci_info::is_ci();
//!     println!("Is CI: {}", ci);
//!
//!     // Get CI environment information
//!     let info = ci_info::get();
//!     println!("Is CI: {}", info.ci);
//!     if info.vendor.is_some() {
//!         println!("Vendor: {:#?}", info.vendor.unwrap());
//!         println!("Name: {:#?}", info.name.unwrap());
//!     }
//!     if info.pr.is_some() {
//!         println!("Is PR: {:#?}", info.pr.unwrap());
//!     }
//!     if info.branch_name.is_some() {
//!         println!("Branch Name: {:#?}", info.branch_name.unwrap());
//!     }
//! }
//! ```
//!
//! ## Check if a CI environment is detected.
//!
//! ```
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
//! There is optional `serde` support that can be enabled via the `serde-1` feature:
//!
//! ```ini
//! [dependencies]
//! ci_info = { version = "*", features = ["serde-1"] }
//! ```
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
#[path = "./lib_test.rs"]
mod lib_test;
#[cfg(test)]
#[path = "./test_env.rs"]
mod test_env;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod ci;
mod config;
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
