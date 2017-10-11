use super::*;

#[test]
fn ci_info_new() {
    let info = CiInfo::new();

    assert!(info.vendor.is_none());
    assert!(!info.ci);
}
