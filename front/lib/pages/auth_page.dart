import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/app_route.dart';
import 'package:swifty_proteins/widgets/sign_in.dart';
import 'package:swifty_proteins/widgets/sign_up.dart';

class AuthScreen extends StatefulWidget {
  const AuthScreen({super.key});
  @override
  _AuthScreenState createState() => _AuthScreenState();
}

class _AuthScreenState extends State<AuthScreen> {
  late Widget signUp;
  final signIn = SignIn();
  Widget _option = SignIn();

  void registrationSuccess() {
    setState(() {
      _option = signIn;
    });
  }

  @override
  void initState() {
    super.initState();
    signUp = SignUp(registrationSuccess: registrationSuccess);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Auth Screen')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            _option,
            //onPressed: () => context.go(Routes.root.route),
            GestureDetector(
              onTap: () {
                setState(() {
                  _option = _option == signUp ? signIn : signUp;
                });
              },
              child: Container(
                padding: const EdgeInsets.symmetric(vertical: 16.0),
                alignment: Alignment.center,
                child: Text(
                  _option == signUp
                      ? 'Already have an account? Sign In'
                      : 'Don\'t have an account? Sign Up',
                  style: const TextStyle(
                    color: Colors.blue,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
            OutlinedButton(
                onPressed: () => context.goNamed(Routes.root.name),
                child: const Text('ByPass Auth Screen'))
          ],
        ),
      ),
    );
  }
}
