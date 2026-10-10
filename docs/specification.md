# Smart Document Engine — foundation specification

Goal: reusable local engine for PDF Manager, with Rust taught through implementation.
GitHub is the canonical source; Codespaces is preferred for development.
Runtime document processing must not require a server or upload document content.

## Current acceptance criteria
- Rust workspace builds and tests without third-party crates or network access.
- One-based inclusive page ranges reject zero, reversed and out-of-document selections.
- C ABI exposes version and validation statuses; Dart verifies ABI and integer bounds.
- A Flutter source example uses the real binding, never a simulated PDF result.

## Future scope
Inspect/render PDFs; merge, split, reorder, rotate; text extraction and search;
images, annotations, forms, passwords, OCR and document analysis.
This is a roadmap, not a claim of support. Exact PDF Manager feature inventory
must be agreed before promising complete coverage.

## Quality requirements
Document content stays local. Enforce input size, decoded image, page-count,
memory and processing-time limits in every future backend.
Use temporary output and atomic replacement; never overwrite inputs by default.
Add cancellation and background execution before long-running mobile operations.
Test malformed/encrypted files, Arabic/RTL text, large files and real device memory.
OCR/model downloads must be opt-in; packaged models remain usable offline.
Do not commit user documents, credentials or signing keys.

## Milestones
1. Foundation and page-selection planning (this change).
2. Backend evaluation: capabilities, licensing, mobile size, safety, Arabic fidelity.
3. Real inspect and extract flow with fixture PDFs and backend integration tests.
4. Android native packaging and real Flutter device test, then iOS on macOS.
5. Expand features and local intelligence with measured performance.

No backend, OCR library or license for public reuse has been selected yet.
