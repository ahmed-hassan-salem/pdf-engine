//! Adapter contract. No parser or renderer is implemented yet.
use document_model::Document;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfError {
    InvalidInput,
    Unsupported,
    ResourceLimit,
}
/// Implementations must enforce limits before processing untrusted PDF data.
pub trait PdfBackend {
    fn inspect(&self, bytes: &[u8]) -> Result<Document, PdfError>;
}
