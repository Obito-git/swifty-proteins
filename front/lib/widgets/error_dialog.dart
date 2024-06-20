import 'package:flutter/material.dart';

void showErrorDialog(String message, BuildContext context,
    {Function? afterPopCallback}) {
  if (!context.mounted) return;
  showDialog(
    context: context,
    builder: (context) {
      return AlertDialog(
        title: const Text('Error'),
        content: Text(message),
        actions: <Widget>[
          TextButton(
            child: const Text('OK'),
            onPressed: () {
              Navigator.of(context).pop();
              if (afterPopCallback != null) {
                afterPopCallback();
              }
            },
          ),
        ],
      );
    },
  );
}
