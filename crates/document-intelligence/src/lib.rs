//! Local analysis contract; OCR/models are not bundled in this milestone.
use document_model::Document;
#[derive(Debug, PartialEq, Eq)]
pub enum AnalysisError {
    Unsupported,
    ResourceLimit,
}
pub trait DocumentAnalyzer {
    type Output;
    fn analyze(&self, document: &Document) -> Result<Self::Output, AnalysisError>;
}
