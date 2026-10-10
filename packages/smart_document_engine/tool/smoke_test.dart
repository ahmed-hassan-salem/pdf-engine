import 'dart:ffi';

import '../lib/smart_document_engine.dart';

void main(List<String> args) {
  if (args.length != 1) throw ArgumentError('Provide native library path');
  final engine = SmartDocumentEngine(DynamicLibrary.open(args.single));
  void expectStatus(int count, int start, int end, ExtractionStatus expected) {
    final actual = engine.validateExtraction(count, start, end);
    if (actual != expected)
      throw StateError('$count/$start/$end: $actual != $expected');
  }

  expectStatus(3, 1, 3, ExtractionStatus.valid);
  expectStatus(3, 3, 3, ExtractionStatus.valid);
  expectStatus(0, 1, 1, ExtractionStatus.emptyDocument);
  expectStatus(3, 0, 1, ExtractionStatus.invalidRange);
  expectStatus(3, 2, 1, ExtractionStatus.invalidRange);
  expectStatus(3, 1, 4, ExtractionStatus.invalidRange);
  expectStatus(3, 0xffffffff, 0xffffffff, ExtractionStatus.invalidRange);
  for (final invalid in [-1, 0x100000000]) {
    try {
      engine.validateExtraction(invalid, 1, 1);
      throw StateError('Expected RangeError');
    } on RangeError {
      /* Expected: no truncation across FFI. */
    }
  }
  print('Rust/Dart FFI smoke test passed');
}
