//! Validated extraction planning. This does not write a PDF.
use document_model::{Document, DocumentError, PageRange};
#[derive(Debug, PartialEq, Eq)]
pub struct ExtractionPlan {
    pub range: PageRange,
}
pub fn plan_extraction(
    document: &Document,
    start: u32,
    end: u32,
) -> Result<ExtractionPlan, DocumentError> {
    Ok(ExtractionPlan {
        range: PageRange::new(document, start, end)?,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_out_of_document_pages() {
        let doc = Document::new(5).unwrap();
        assert!(plan_extraction(&doc, 2, 4).is_ok());
        assert_eq!(
            plan_extraction(&doc, 2, 6),
            Err(DocumentError::InvalidPageRange)
        );
    }
}
