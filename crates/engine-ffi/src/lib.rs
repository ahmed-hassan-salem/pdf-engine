//! C ABI v1: scalars only; no pointers or allocations cross this boundary.
use document_model::Document;
use feature_engines::plan_extraction;
#[no_mangle]
pub extern "C" fn sde_abi_version() -> u32 {
    1
}
/// Status: 0 success, 1 empty document, 2 invalid range.
#[no_mangle]
pub extern "C" fn sde_validate_extraction(page_count: u32, start: u32, end: u32) -> u32 {
    let Ok(document) = Document::new(page_count) else {
        return 1;
    };
    match plan_extraction(&document, start, end) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abi_status_contract() {
        assert_eq!(sde_abi_version(), 1);
        assert_eq!(sde_validate_extraction(0, 1, 1), 1);
        assert_eq!(sde_validate_extraction(3, 1, 4), 2);
        assert_eq!(sde_validate_extraction(3, 1, 3), 0);
    }
}
