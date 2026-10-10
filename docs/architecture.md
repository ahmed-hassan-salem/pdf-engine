# Architecture
Dependency arrows mean "depends on".

Flutter app -> Dart binding -> engine-ffi -> feature-engines -> document-model
pdf-core -> document-model
document-intelligence -> document-model

The model has no backend, UI or intelligence dependencies.
PDF Core defines the backend boundary; a selected adapter will implement it.
Document Intelligence defines local analysis separately from PDF mechanics.
Feature Engines orchestrate use cases; initially only extraction planning exists.
Later features can depend on PDF Core and Intelligence without reversing layers.
FFI is an adapter, not the home of business logic.

## Boundary decision
Start with a small C ABI and dart:ffi. ABI v1 has scalars only and owns no shared
memory. Exported validation cannot panic for any u32 inputs.
Before exposing file buffers: define handles, lifetime/free rules, typed errors,
panic containment, versioning, resource limits and cancellation.
Heavy processing must run outside the Flutter UI isolate.

## Backend decision remains open
Do not build a full PDF parser merely to avoid a library. Compare existing
backends against requirements and licensing before implementation.
Traits permit substitution; they are contracts, not working PDF support.

## Platforms
Host dynamic-library smoke tests come first.
Android needs cross-compilation and library packaging per ABI.
iOS needs macOS/Xcode, static linkage and symbol retention.
Codespaces does not replace device validation or iOS builds.

## References
- https://dart.dev/interop/c-interop
- https://doc.rust-lang.org/book/
- https://docs.github.com/en/codespaces/setting-up-your-project-for-codespaces/adding-a-dev-container-configuration/introduction-to-dev-containers
