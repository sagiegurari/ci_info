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
