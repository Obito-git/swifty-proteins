import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/app_route.dart';

class RootScreen extends StatelessWidget {
  const RootScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Root Screen'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            ElevatedButton(
              //TODO: refactor and implement actual logic
              onPressed: () => context.goNamed(Routes.protein.name,
                  pathParameters: {"code": "123"}),
              child: const Text('Go to Protein Screen'),
            ),
          ],
        ),
      ),
    );
  }
}
