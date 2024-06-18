import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:swifty_proteins/exception/bad_request_exception.dart';
import 'package:swifty_proteins/models/user_credentials.dart';

class UserApiService {
  static const String baseUrl = 'http://10.0.2.2:8000';

  Future<String?> signIn(UserCredentials credentials) async {
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

      final jsonBody = jsonDecode(response.body);
      if (response.statusCode == 200) {
        return jsonBody['token'];
      } else if (response.statusCode == 400) {
        throw BadRequestException(jsonBody['message']);
      } else {
        print("ERRRRRRRRRRRRRR ${response.statusCode}, ${response.body}");
        throw Exception('Something went wrong!');
      }
    } on BadRequestException {
      rethrow;
    } catch (e) {
      throw Exception('Something went wrong!');
    }
  }

  Future<void> signUp(UserCredentials credentials) async {
    String apiUrl = '$baseUrl/signup';

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

      if (response.statusCode != 201) {
        final jsonBody = jsonDecode(response.body);
        throw BadRequestException(jsonBody['message']);
      }
    } catch (e) {
      throw Exception('Something went wrong!');
    }
  }
}
