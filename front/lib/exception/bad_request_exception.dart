class BadRequestException implements Exception {
  final Map<String, dynamic> errors;

  BadRequestException(this.errors);

  @override
  String toString() {
    return '$errors';
  }
}