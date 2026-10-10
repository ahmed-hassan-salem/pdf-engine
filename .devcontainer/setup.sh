#!/usr/bin/env bash
set -euo pipefail
sde_flutter_dir="/home/vscode/.local/share/sde-flutter"
if [ ! -d "$sde_flutter_dir/.git" ]; then
  git clone --depth 1 --branch 3.47.4 https://github.com/flutter/flutter.git "$sde_flutter_dir"
fi
export PATH="$sde_flutter_dir/bin:$PATH"
flutter --version
flutter precache --linux
cargo test --workspace --offline
cargo build -p engine-ffi --offline
dart analyze packages/smart_document_engine
dart packages/smart_document_engine/tool/smoke_test.dart "$PWD/target/debug/libengine_ffi.so"
(cd examples/flutter_demo && flutter pub get && flutter analyze)
