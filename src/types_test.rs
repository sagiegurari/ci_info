use super::*;

#[test]
fn ci_info_new() {
    let ci_info = CiInfo::new();

    assert!(ci_info.vendor.is_none());
    assert!(!ci_info.ci);
}
