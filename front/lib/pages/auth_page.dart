import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/app_route.dart';

class AuthScreen extends StatelessWidget {
  const AuthScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Auth Screen')),
      body: Center(child:ElevatedButton(
        onPressed: () => context.go(Routes.root.route),
        child: const Text('Login'),
      ),),
    );
  }
}
