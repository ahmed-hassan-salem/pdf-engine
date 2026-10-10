//! Backend-independent document metadata and validated page selections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    page_count: u32,
}
impl Document {
    pub fn new(page_count: u32) -> Result<Self, DocumentError> {
        if page_count == 0 {
            return Err(DocumentError::EmptyDocument);
        }
        Ok(Self { page_count })
    }
    pub fn page_count(&self) -> u32 {
        self.page_count
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentError {
    EmptyDocument,
    InvalidPageRange,
}
/// Inclusive, one-based page selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRange {
    start: u32,
    end: u32,
}
impl PageRange {
    pub fn new(document: &Document, start: u32, end: u32) -> Result<Self, DocumentError> {
        if start == 0 || end < start || end > document.page_count() {
            return Err(DocumentError::InvalidPageRange);
        }
        Ok(Self { start, end })
    }
    pub fn start(&self) -> u32 {
        self.start
    }
    pub fn end(&self) -> u32 {
        self.end
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_boundaries() {
        let doc = Document::new(3).unwrap();
        assert!(PageRange::new(&doc, 1, 3).is_ok());
        assert!(PageRange::new(&doc, 3, 3).is_ok());
        for (start, end) in [(0, 1), (2, 1), (1, 4), (u32::MAX, u32::MAX)] {
            assert_eq!(
                PageRange::new(&doc, start, end),
                Err(DocumentError::InvalidPageRange)
            );
        }
        assert_eq!(Document::new(0), Err(DocumentError::EmptyDocument));
    }
}
