import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/common/command.dart';
import 'package:flutter/material.dart';

import '../../common/result.dart';

class CollectionsScreenNotifier extends ChangeNotifier {
  CollectionsScreenNotifier({
    required CollectionsRepository collectionsRepository,
  }) : _collectionsRepository = collectionsRepository {
    loadCollections = SimpleCommand(_loadCollections);
  }

  final CollectionsRepository _collectionsRepository;
  List<Collection>? _collections;
  late final SimpleCommand<void> loadCollections;
  List<Collection>? get collections => _collections;

  Future<Result<void>> _loadCollections() async {
    final result = await _collectionsRepository.all();
    switch (result) {
      case Ok<List<Collection>>():
        _collections = result.value;
        notifyListeners();
      case Error<List<Collection>>():
        notifyListeners();
    }
    return result;
  }
}
