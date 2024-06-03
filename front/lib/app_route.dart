import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/pages/auth_page.dart';
import 'package:swifty_proteins/pages/home_page.dart';
import 'package:swifty_proteins/pages/protein_page.dart';

enum Routes {
  root("/"),
  auth("/auth"),
  protein("/protein"),
  ;

  final String route;

  const Routes(this.route);
}

class AppRoute extends StatelessWidget {
  final GoRouter _router = GoRouter(
    routes: <RouteBase>[
      GoRoute(
        path: Routes.root.route,
        builder: (context, state) => const RootScreen(),
        routes: [
          GoRoute(
            path: Routes.protein.route,
            builder: (context, state) => const ProteinScreen(),
          )
        ],
      ),
      GoRoute(
        path: Routes.auth.route,
        builder: (context, state) => const AuthScreen(),
      ),
    ],
    initialLocation: Routes.auth.route,
  );

  AppRoute({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      routerDelegate: _router.routerDelegate,
      routeInformationParser: _router.routeInformationParser,
      routeInformationProvider: _router.routeInformationProvider,
    );
  }
}
