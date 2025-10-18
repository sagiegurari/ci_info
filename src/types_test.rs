use super::*;

#[test]
fn ci_info_new() {
    let info = CiInfo::new();

    assert!(info.vendor.is_none());
    assert!(info.name.is_none());
    assert!(!info.ci);
    assert!(info.pr.is_none());
    assert!(info.branch_name.is_none());
}

// ===== Iterator Feature Tests =====
// These tests verify Vendor enum iteration functionality when iter feature is enabled

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_basic() {
    use strum::IntoEnumIterator;

    let variants: Vec<Vendor> = Vendor::iter().collect();
    assert!(
        !variants.is_empty(),
        "Iterator should produce at least one variant"
    );
}

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_count() {
    use strum::IntoEnumIterator;

    // Update this constant when adding or removing variants
    // This test will fail if Vendor enum changes, prompting developer to verify all changes
    const EXPECTED_VARIANT_COUNT: usize = 51;
    let actual_count = Vendor::iter().count();

    assert_eq!(
        actual_count, EXPECTED_VARIANT_COUNT,
        "Vendor enum has {} variants but expected {}. \
         Did you add/remove variants? Update EXPECTED_VARIANT_COUNT in this test.",
        actual_count, EXPECTED_VARIANT_COUNT
    );
}

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_uniqueness() {
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    let variants: Vec<Vendor> = Vendor::iter().collect();
    let unique: HashSet<_> = variants.iter().collect();

    assert_eq!(
        variants.len(),
        unique.len(),
        "Iterator produced duplicate variants"
    );
}

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_contains_known_vendors() {
    use strum::IntoEnumIterator;

    let variants: Vec<Vendor> = Vendor::iter().collect();

    // Verify some well-known CI vendors are included in iteration
    // This catches potential issues where specific variants might be accidentally excluded
    assert!(
        variants.contains(&Vendor::GitHubActions),
        "GitHubActions should be in iterator"
    );
    assert!(
        variants.contains(&Vendor::GitLabCI),
        "GitLabCI should be in iterator"
    );
    assert!(
        variants.contains(&Vendor::CircleCI),
        "CircleCI should be in iterator"
    );
    assert!(
        variants.contains(&Vendor::TravisCI),
        "TravisCI should be in iterator"
    );
    assert!(
        variants.contains(&Vendor::Unknown),
        "Unknown variant should be in iterator"
    );
}

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_copy_semantics() {
    use strum::IntoEnumIterator;

    // Vendor implements Copy, so iterator should work without .cloned()
    // This verifies Copy semantics are preserved with iterator
    let first_pass: Vec<Vendor> = Vendor::iter().collect();
    let second_pass: Vec<Vendor> = Vendor::iter().collect();

    assert_eq!(
        first_pass, second_pass,
        "Multiple iterations should produce identical results"
    );
}

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_functional_operations() {
    use strum::IntoEnumIterator;

    // Demonstrate practical usage patterns - filtering, mapping, etc.
    let github_or_gitlab: Vec<Vendor> = Vendor::iter()
        .filter(|v| matches!(v, Vendor::GitHubActions | Vendor::GitLabCI))
        .collect();

    assert_eq!(github_or_gitlab.len(), 2, "Should find exactly 2 variants");
    assert!(github_or_gitlab.contains(&Vendor::GitHubActions));
    assert!(github_or_gitlab.contains(&Vendor::GitLabCI));
}

#[test]
#[cfg(feature = "iter")]
fn vendor_iter_debug_formatting() {
    use strum::IntoEnumIterator;

    // Verify all variants can be debug-formatted (useful for logging/debugging)
    for vendor in Vendor::iter() {
        let debug_str = format!("{:?}", vendor);
        assert!(
            !debug_str.is_empty(),
            "Debug formatting should produce non-empty string"
        );
    }
}
