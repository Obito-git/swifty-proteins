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
  final List<Protein> _allProteins = [];
  final List<Protein> _filteredProteins = [];
  bool _isSearching = false;
  final ScrollController _scrollController = ScrollController();
  final ProteinApiService _proteinService = ProteinApiService();
  PageMetadata? _lastPageMetadata;
  PageMetadata? _filteredPageMetadata;
  bool _isLoading = false;
  String _searchValue = '';

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
        _allProteins.addAll(data.data);
      });
    });
  }

  void getFilteredProteins(String filterValue) {
    setState(() {
      _isLoading = true;
    });
    final nextPage = (_filteredPageMetadata != null)
        ? _filteredPageMetadata!.currentPage + 1
        : 1;
    _proteinService
        .getProteinsPage(page: nextPage, filter: filterValue)
        .then((data) {
      setState(() {
        _isSearching = true;
        _isLoading = false;
        _filteredPageMetadata = data.metadata;
        _filteredProteins.addAll(data.data);
      });
    });
  }

  void loadMoreProteins() {
    if (_scrollController.position.pixels ==
        _scrollController.position.maxScrollExtent) {
      if (_isSearching) {
        if (_filteredPageMetadata!.currentPage <
            _filteredPageMetadata!.totalPages) {
          getFilteredProteins(_searchValue);
        }
      } else {
        if (_lastPageMetadata!.currentPage < _lastPageMetadata!.totalPages) {
          getProteins();
        }
      }
    }
  }

  void handleSearch(String value) {
    _searchValue = value;
    _filteredProteins.clear();
    _filteredPageMetadata = null;
    if (value.isEmpty) {
      setState(() {
        _isSearching = false;
      });
    } else {
      setState(() {
        _isSearching = true;
      });
      getFilteredProteins(value);
    }
  }

  @override
  Widget build(BuildContext context) {
    final proteins = _isSearching ? _filteredProteins : _allProteins;
    return Scaffold(
      appBar: AppBar(
        title: const Text('Root Page'),
      ),
      body: SizedBox(
        height: MediaQuery.of(context).size.height,
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: TextField(
                decoration: const InputDecoration(
                  prefixIcon: Icon(Icons.search),
                  hintText: "001",
                  border: OutlineInputBorder(),
                  labelText: 'Search',
                ),
                onChanged: handleSearch,
              ),
            ),
            Expanded(
              child: ListView.builder(
                physics: const AlwaysScrollableScrollPhysics(),
                controller: _scrollController,
                itemBuilder: (context, index) {
                  return Column(
                    children: [
                      ListTile(
                        title: Text(proteins[index].code),
                        onTap: () => context.goNamed(
                          Routes.protein.name,
                          pathParameters: {"code": proteins[index].code},
                        ),
                      ),
                      if (index == proteins.length - 1 && _isLoading)
                        const Padding(
                          padding: EdgeInsets.all(10.0),
                          child: SpinKitThreeBounce(color: Colors.blue),
                        ),
                    ],
                  );
                },
                itemCount: proteins.length,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
