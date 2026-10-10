import 'dart:ffi';

enum ExtractionStatus { valid, emptyDocument, invalidRange }

/// Supply the built native library explicitly. Mobile bundling is a later milestone.
final class SmartDocumentEngine {
  SmartDocumentEngine(DynamicLibrary library) {
    final version = library.lookupFunction<Uint32 Function(), int Function()>(
      'sde_abi_version',
    )();
    if (version != 1) throw StateError('Unsupported engine ABI: $version');
    _validate = library
        .lookupFunction<
          Uint32 Function(Uint32, Uint32, Uint32),
          int Function(int, int, int)
        >('sde_validate_extraction');
  }
  late final int Function(int, int, int) _validate;
  ExtractionStatus validateExtraction(int pageCount, int start, int end) {
    for (final value in [pageCount, start, end]) {
      if (value < 0 || value > 0xffffffff) {
        throw RangeError.range(value, 0, 0xffffffff);
      }
    }
    final status = _validate(pageCount, start, end);
    if (status >= ExtractionStatus.values.length) {
      throw StateError('Unknown engine status: $status');
    }
    return ExtractionStatus.values[status];
  }
}
