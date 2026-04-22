import 'package:flutter/material.dart';
import 'package:nato_app/src/rust/api.dart';
import 'package:nato_app/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    final entries = convertNato(input: 'hello');
    return MaterialApp(
      title: 'NATO Phonetic Alphabet',
      home: Scaffold(
        appBar: AppBar(title: const Text('NATO Phonetic Alphabet')),
        body: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Rust bridge verified — "hello":',
                style: TextStyle(fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 8),
              ...entries.map(
                (e) => Text(
                  '${e.character.toUpperCase()} – ${e.word ?? "(no equivalent)"}',
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
