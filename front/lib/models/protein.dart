class PageMetadata {
  final int totalPages;
  final int currentPage;
  final int pageSize;
  final int totalItems;
  final int itemsOnPage;

  PageMetadata({
    required this.totalPages,
    required this.currentPage,
    required this.pageSize,
    required this.totalItems,
    required this.itemsOnPage,
  });

  factory PageMetadata.fromJson(Map<String, dynamic> json) {
    return PageMetadata(
      totalPages: json['total_pages'],
      currentPage: json['current_page'],
      pageSize: json['page_size'],
      totalItems: json['total_items'],
      itemsOnPage: json['items_on_page'],
    );
  }
}

class Protein {
  final String code;

  Protein({required this.code});

  factory Protein.fromJson(Map<String, dynamic> json) {
    return Protein(
      code: json['code'],
    );
  }
}

class ProteinPage {
  final PageMetadata metadata;
  final List<Protein> data;

  ProteinPage({
    required this.metadata,
    required this.data,
  });

  factory ProteinPage.fromJson(Map<String, dynamic> json) {
    var dataJsonList = json['data'] as List;
    var dataList = dataJsonList.map((item) => Protein.fromJson(item)).toList();

    return ProteinPage(
      metadata: PageMetadata.fromJson(json['metadata']),
      data: dataList,
    );
  }
}
