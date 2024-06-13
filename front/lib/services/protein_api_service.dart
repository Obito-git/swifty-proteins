import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:swifty_proteins/models/protein.dart';

class ProteinApiService {
  static const String baseUrl = 'http://10.0.2.2:8000';

  Future<ProteinPage> getProteinsPage({int page = 1, String? filter}) async {
    final queryParameters = {
      'page': page.toString(),
      if (filter != null) 'filter': filter,
    };
    final uri = Uri.parse(baseUrl)
        .replace(path: '/proteins', queryParameters: queryParameters);

    final response = await http.get(uri);

    if (response.statusCode == 200) {
      final json = jsonDecode(response.body) as Map<String, dynamic>;
      return ProteinPage.fromJson(json);
    } else {
      throw Exception('Failed to load proteins');
    }
  }
}
