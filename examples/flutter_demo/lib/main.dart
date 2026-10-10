import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:smart_document_engine/smart_document_engine.dart';

void main() => runApp(const MaterialApp(home: EngineDemo()));

class EngineDemo extends StatelessWidget {
  const EngineDemo({super.key});
  @override
  Widget build(BuildContext context) {
    const path = String.fromEnvironment('SDE_LIBRARY_PATH');
    String message = 'Provide SDE_LIBRARY_PATH to load the native engine.';
    if (path.isNotEmpty) {
      try {
        final engine = SmartDocumentEngine(DynamicLibrary.open(path));
        message = 'Pages 1–3 of 3: ${engine.validateExtraction(3, 1, 3).name}';
      } catch (error) {
        message = 'Engine could not load: $error';
      }
    }
    return Scaffold(
      appBar: AppBar(title: const Text('Smart Document Engine')),
      body: Center(child: Text(message)),
    );
  }
}
