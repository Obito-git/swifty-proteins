import 'package:flutter/material.dart';
import 'package:model_viewer_plus/model_viewer_plus.dart';

class ProteinScreen extends StatelessWidget {
  final String code;
  const ProteinScreen({super.key, required this.code});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Protein Screen of $code')),
      body: const Center(
        child: ModelViewer(
          src: 'http://10.0.2.2:8000/proteins/666',
          alt: "A 3D model of Matilda",
          ar: true,
          autoRotate: true,
          cameraControls: true,
        ),
      ),
    );
  }
}
