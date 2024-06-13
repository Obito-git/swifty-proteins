import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:swifty_proteins/models/user_credentials.dart';

class UserApiService {
  static const String baseUrl = 'http://10.0.2.2:8000';

  Future<void> loginUser(UserCredentials credentials) async {
    String apiUrl = '$baseUrl/signin';

    try {
      final response = await http.post(
        Uri.parse(apiUrl),
        headers: <String, String>{
          'Content-Type': 'application/json; charset=UTF-8',
        },
        body: jsonEncode(<String, String>{
          'username': credentials.username,
          'password': credentials.password,
        }),
      );

      if (response.statusCode == 200) {
        print('Login successful!!!!');
        print('response body ${response.body}');
      } else {
        print('status code ${response.statusCode}');
        print('response body ${response.body}');
      }
    } catch (e) {
      print('$e');
    }
  }
}
