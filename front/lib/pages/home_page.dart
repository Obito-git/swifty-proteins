import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:go_router/go_router.dart';
import 'package:swifty_proteins/app_route.dart';
import 'package:swifty_proteins/models/protein.dart';
import 'package:swifty_proteins/services/protein_api_service.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final List<Protein> _proteins = [];
  final ScrollController _scrollController = ScrollController();
  final ProteinApiService _proteinService = ProteinApiService();
  PageMetadata? _lastPageMetadata;
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    getProteins();

    _scrollController.addListener(loadMoreProteins);
  }

  void getProteins() {
    setState(() {
      _isLoading = true;
    });
    final nextPage =
        (_lastPageMetadata != null) ? _lastPageMetadata!.currentPage + 1 : 1;
    _proteinService.getProteinsPage(page: nextPage).then((data) {
      setState(() {
        _isLoading = false;
        _lastPageMetadata = data.metadata;
        _proteins.addAll(data.data);
      });
    });
  }

  void loadMoreProteins() {
    if (_scrollController.position.pixels ==
        _scrollController.position.maxScrollExtent) {
      if (_lastPageMetadata!.currentPage < _lastPageMetadata!.totalPages) {
        getProteins();
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: const Text('Root Page'),
        ),
        body: SizedBox(
          height: MediaQuery.sizeOf(context).height,
          child: ListView.builder(
            physics: const AlwaysScrollableScrollPhysics(),
            shrinkWrap: false,
            controller: _scrollController,
            itemBuilder: (context, index) {
              return Column(children: [
                ListTile(
                  title: Text(_proteins[index].code),
                  onTap: () => context.goNamed(Routes.protein.name,
                      pathParameters: {"code": _proteins[index].code}),
                ),
                if (index == _proteins.length - 1 && _isLoading)
                  const Padding(
                      padding: EdgeInsets.all(10.0),
                      child: SpinKitThreeBounce(color: Colors.blue)),
              ]);
            },
            itemCount: _proteins.length,
          ),
        ));
  }
}

/*
Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            ElevatedButton(
              onPressed: () => context.goNamed(Routes.protein.name,
                  pathParameters: {"code": "123"}),
              child: const Text('Go to Protein Page'),
            ),
          ],
        ),
      ),
*/