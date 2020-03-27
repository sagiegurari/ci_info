use ci_info;
use ci_info::types::{CiInfo, Vendor};

#[test]
fn mock_ci() {
    // create the CI info manually
    let mut mock_info = CiInfo::new();
    mock_info.vendor = Some(Vendor::TravisCI);
    mock_info.ci = true;
    mock_info.pr = Some(true);
    mock_info.branch_name = Some("dev_branch".to_string());

    // mock environment
    ci_info::mock_ci(&mock_info);

    let info = ci_info::get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "dev_branch");
}
