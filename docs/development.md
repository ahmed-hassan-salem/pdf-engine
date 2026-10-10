# Development
## Rust
Install Rust 1.95.0 (rust-toolchain.toml selects it).
cargo fmt --all
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo build -p engine-ffi --offline

## Native binding smoke test
With Dart installed, from repository root:
Windows:
dart packages/smart_document_engine/tool/smoke_test.dart target/debug/engine_ffi.dll
Linux:
dart packages/smart_document_engine/tool/smoke_test.dart "$PWD/target/debug/libengine_ffi.so"
macOS: use target/debug/libengine_ffi.dylib (sign it if required).

## Codespaces
Create a Codespace from the foundation branch to load .devcontainer.
The first setup requires internet to download the image/toolchain.
Rust is preconfigured; setup.sh installs Flutter 3.47.4 (including Dart),
then runs Rust tests, the native binding smoke test and Flutter analysis.
The first Codespace setup has not been exercised locally on Windows.
CI validates Linux bindings and Flutter independently.
An offline runtime is required; initial SDK/dependency installation is separate.
Commit and push before switching machines; uncommitted Codespace edits are not
automatically available in another checkout.

## Flutter example
Source only: platform runners and mobile native packaging are not generated.
For a host desktop demo with the platform toolchain installed:
cd examples/flutter_demo
flutter create --platforms=windows,linux,macos .
flutter pub get
flutter run -d windows --dart-define=SDE_LIBRARY_PATH=<absolute-path-to-engine_ffi.dll>
Use linux/macOS and their native library paths on the corresponding host.
Android/iOS integration is a later milestone; do not claim device support yet.

No license grant is made by making this repository public. Choose a license
explicitly after the backend license review.

Local validation used the installed stable Rust 1.95.0 via cargo +stable because
the host rustup configuration requested unavailable optional components when
resolving the numbered toolchain. No global toolchain settings were changed.
