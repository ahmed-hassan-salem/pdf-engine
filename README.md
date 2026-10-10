# Smart Document Engine
Rust engine foundation for local document processing, with Dart/Flutter integration.
Repository: pdf-engine. Product name: Smart Document Engine.

**Status:** architecture and executable page-selection validation only.
No PDF parsing, rendering, extraction, OCR or mobile library packaging yet.

- [Specification](docs/specification.md)
- [Architecture](docs/architecture.md)
- [Development / Codespaces](docs/development.md)
- [Learn Rust with the project (Arabic)](docs/learning/01-document-model.ar.md)

Start: cargo test --workspace --offline
Then build engine-ffi and run the real Dart/native smoke test per development.md.
