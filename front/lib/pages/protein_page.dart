import 'package:flutter/material.dart';

class ProteinScreen extends StatelessWidget {
  final String code;
  const ProteinScreen({super.key, required this.code});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Protein Screen')),
      body: Center(child: Text('Protein Page of code: $code')),
    );
  }
}
