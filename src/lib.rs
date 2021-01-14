#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bindings_with_variant_name,
    broken_intra_doc_links,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    drop_bounds,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_codeblock_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_autolinks,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    panic_fmt,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_doc_tests,
    private_in_public,
    private_intra_doc_links,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unsupported_naked_functions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
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
    useless_deprecated,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    invalid_html_tags,
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
#[cfg(test)]
#[path = "./vendors_test.rs"]
mod vendors_test;

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
