import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/pages/auth_page.dart';
import 'package:swifty_proteins/pages/home_page.dart';
import 'package:swifty_proteins/pages/protein_page.dart';

enum Routes {
  //TODO: refactor structure
  root(route: "/", name: "root"),
  auth(route: "/auth", name: "auth"),
  protein(route: "protein/:code", name: "protein"),
  ;

  final String route;
  final String name;

  const Routes({required this.route, required this.name});
}

class AppRoute extends StatelessWidget {
  final GoRouter _router = GoRouter(
    routes: <RouteBase>[
      GoRoute(
        path: Routes.root.route,
        name: Routes.root.name,
        builder: (context, state) => const RootScreen(),
        routes: [
          GoRoute(
            path: Routes.protein.route,
            name: Routes.protein.name,
            builder: (context, state) =>
                //TODO: add null check for code
                ProteinScreen(code: state.pathParameters['code']!),
          )
        ],
      ),
      GoRoute(
        path: Routes.auth.route,
        name: Routes.auth.name,
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
