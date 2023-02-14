#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bindings_with_variant_name,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_cfg_attr_crate_type_name,
    deprecated_in_future,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    enum_intrinsics_non_enums,
    explicit_outlives_requirements,
    exported_private_dependencies,
    for_loops_over_fallibles,
    forbidden_lint_groups,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    implied_bounds_entailment,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    invalid_atomic_ordering,
    invalid_doc_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    let_underscore_drop,
    let_underscore_lock,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_transmutes,
    named_arguments_used_positionally,
    named_asm_labels,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    opaque_hidden_inferred_bound,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    repr_transparent_external_private_fields,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    special_module_name,
    stable_features,
    suspicious_auto_trait_impls,
    temporary_cstring_as_ptr,
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    unexpected_cfgs,
    ungated_async_fn_track_caller,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
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
    unused_macro_rules,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_tuple_struct_fields,
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
