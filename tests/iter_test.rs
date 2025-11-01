//! Integration tests for Vendor enum iteration feature
//!
//! These tests verify iteration works correctly from external crate perspective,
//! ensuring public API is properly exposed when iter feature is enabled.

#[cfg(feature = "iter")]
mod iter_tests {
    use ci_info::types::Vendor;
    use strum::IntoEnumIterator;

    #[test]
    fn can_iterate_all_vendors() {
        let count = Vendor::iter().count();

        // We expect to support many CI vendors (50+)
        // This test will catch if iteration somehow produces very few variants
        assert!(
            count > 50,
            "Expected many CI vendors (50+), but got {}. \
             Check if EnumIter derive is working correctly.",
            count
        );
    }

    #[test]
    fn iteration_is_deterministic() {
        // Iteration order should be consistent across multiple calls
        // This is important for reproducible behavior in applications
        let first: Vec<_> = Vendor::iter().collect();
        let second: Vec<_> = Vendor::iter().collect();

        assert_eq!(
            first, second,
            "Iterator should produce same order across multiple invocations"
        );
    }

    #[test]
    fn can_collect_into_various_containers() {
        use std::collections::HashSet;

        // Verify iterator works with different collection types
        let vec: Vec<_> = Vendor::iter().collect();
        let hashset: HashSet<_> = Vendor::iter().collect();

        assert_eq!(vec.len(), hashset.len(), "All variants should be unique");
    }

    #[test]
    fn iteration_with_chaining() {
        // Demonstrate that iterator can be chained with other operations
        let non_unknown_count = Vendor::iter()
            .filter(|v| !matches!(v, Vendor::Unknown))
            .count();

        let total_count = Vendor::iter().count();

        // Unknown variant exists, so filtered count should be one less
        assert_eq!(
            non_unknown_count,
            total_count - 1,
            "Filtering out Unknown should reduce count by 1"
        );
    }

    #[test]
    fn iteration_supports_find() {
        // Demonstrate find operation works on iterator
        let github_actions = Vendor::iter().find(|v| matches!(v, Vendor::GitHubActions));

        assert!(
            github_actions.is_some(),
            "Should be able to find GitHubActions variant"
        );
        assert_eq!(github_actions.unwrap(), Vendor::GitHubActions);
    }

    #[test]
    fn iteration_supports_any() {
        // Demonstrate any() predicate works
        let has_gitlab = Vendor::iter().any(|v| matches!(v, Vendor::GitLabCI));

        assert!(has_gitlab, "Iterator should contain GitLabCI");
    }

    #[test]
    fn iteration_supports_all() {
        // Demonstrate all() predicate works
        let all_are_vendors = Vendor::iter().all(|_v| true); // All variants are valid Vendor types

        assert!(all_are_vendors, "All items should be Vendor variants");
    }

    #[test]
    fn debug_print_all_vendors() {
        // Useful for manual verification during code review
        // Also verifies Debug trait works with all variants
        println!("\n=== All Supported CI Vendors ===");
        for (idx, vendor) in Vendor::iter().enumerate() {
            println!("{}. {:?}", idx + 1, vendor);
        }
        println!("Total: {} vendors\n", Vendor::iter().count());
    }

    #[test]
    fn iteration_works_with_partition() {
        // Demonstrate partition operation
        let (cloud_based, others): (Vec<_>, Vec<_>) = Vendor::iter().partition(|v| {
            matches!(
                v,
                Vendor::GitHubActions
                    | Vendor::GitLabCI
                    | Vendor::CircleCI
                    | Vendor::AzurePipelines
                    | Vendor::AWSCodeBuild
                    | Vendor::GoogleCloudBuild
            )
        });

        assert!(
            !cloud_based.is_empty(),
            "Should identify some cloud-based CI vendors"
        );
        assert!(
            !others.is_empty(),
            "Should have non-cloud-based vendors too"
        );
    }
}
