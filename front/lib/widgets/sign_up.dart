import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/app_route.dart';
import 'package:swifty_proteins/exception/bad_request_exception.dart';
import 'package:swifty_proteins/models/user_credentials.dart';
import 'package:swifty_proteins/services/user_api_service.dart';
import 'package:swifty_proteins/widgets/error_dialog.dart';

class SignUp extends StatefulWidget {
  final Function() registrationSuccess;

  const SignUp({super.key, required this.registrationSuccess});

  @override
  _SignUpState createState() => _SignUpState();
}

class _SignUpState extends State<SignUp> {
  final UserApiService _apiService = UserApiService();
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _confirmPasswordController =
      TextEditingController();
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  bool _isObscure = true;
  bool _isConfirmObscure = true;

  final RegExp alnumRegex = RegExp(r'^[a-zA-Z0-9_]+$');

  void _toggleObscure() {
    setState(() {
      _isObscure = !_isObscure;
    });
  }

  void _toggleConfirmObscure() {
    setState(() {
      _isConfirmObscure = !_isConfirmObscure;
    });
  }

  void _submitForm() {
    if (_formKey.currentState!.validate()) {
      String username = _usernameController.text.trim();
      String password = _passwordController.text.trim();

      try {
        _apiService
            .signUp(UserCredentials(username: username, password: password));

        _usernameController.clear();
        _passwordController.clear();
        _confirmPasswordController.clear();
        if (!context.mounted) return;
        widget.registrationSuccess();
      } on BadRequestException catch (e) {
        showErrorDialog(e.toString(), context);
      } catch (e) {
        showErrorDialog("Something went wrong!", context);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Form(
        key: _formKey,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            TextFormField(
              controller: _usernameController,
              decoration: const InputDecoration(
                labelText: 'Username',
                border: OutlineInputBorder(),
              ),
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Username is required';
                } else if (value.length < 6) {
                  return 'Username must be at least 6 characters';
                } else if (value.length > 20) {
                  return 'Username must not exceed 20 characters';
                } else if (!alnumRegex.hasMatch(value)) {
                  return 'Username must contain only letters, numbers';
                }

                return null;
              },
            ),
            const SizedBox(height: 12.0),
            TextFormField(
              controller: _passwordController,
              decoration: InputDecoration(
                labelText: 'Password',
                border: const OutlineInputBorder(),
                suffixIcon: IconButton(
                  icon: Icon(
                      _isObscure ? Icons.visibility : Icons.visibility_off),
                  onPressed: _toggleObscure,
                ),
              ),
              obscureText: _isObscure,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Password is required';
                } else if (value.length < 6) {
                  return 'Password must be at least 6 characters';
                } else if (value.length > 20) {
                  return 'Password must not exceed 20 characters';
                } else if (!alnumRegex.hasMatch(value)) {
                  return 'Password must contain only letters, numbers';
                }

                return null;
              },
            ),
            const SizedBox(height: 12.0),
            TextFormField(
              controller: _confirmPasswordController,
              decoration: InputDecoration(
                labelText: 'Confirm Password',
                border: const OutlineInputBorder(),
                suffixIcon: IconButton(
                  icon: Icon(_isConfirmObscure
                      ? Icons.visibility
                      : Icons.visibility_off),
                  onPressed: _toggleConfirmObscure,
                ),
              ),
              obscureText: _isConfirmObscure,
              validator: (value) {
                if (value!.isEmpty) {
                  return 'Confirm Password is required';
                }
                if (value != _passwordController.text) {
                  return 'Passwords do not match';
                }
                return null;
              },
            ),
            const SizedBox(height: 24.0),
            ElevatedButton(
              onPressed: _submitForm,
              child: const Text('Sign Up'),
            ),
          ],
        ),
      ),
    );
  }
}
