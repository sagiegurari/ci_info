#[cfg(feature = "iter")]
use strum::IntoEnumIterator;
use ci_info::types::Vendor;

fn main() {
    #[cfg(not(feature = "iter"))]
    {
        eprintln!("Error: This example requires the 'iter' feature to be enabled.");
        eprintln!("Run with: cargo run --example iterate_vendors --features iter");
        std::process::exit(1);
    }

    #[cfg(feature = "iter")]
    {
        // List all supported CI vendors
        println!("Supported CI vendors:");
        for vendor in Vendor::iter() {
            println!("  {:?}", vendor);
        }

        // Count total vendors
        let count = Vendor::iter().count();
        println!("\nTotal: {} vendors", count);

        // Check if specific vendor is supported
        let has_github = Vendor::iter().any(|v| matches!(v, Vendor::GitHubActions));
        println!("GitHub Actions supported: {}", has_github);

        // Filter for vendors containing "Git"
        let git_vendors: Vec<_> = Vendor::iter()
            .filter(|v| format!("{:?}", v).contains("Git"))
            .collect();
        println!("\nVendors with 'Git' in name: {:?}", git_vendors);
    }
}
