import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/common/command.dart';
import 'package:flutter/material.dart';

import '../../common/result.dart';

class CollectionsScreenNotifier extends ChangeNotifier {
  CollectionsScreenNotifier({
    required CollectionsRepository collectionsRepository,
  }) : _collectionsRepository = collectionsRepository {
    loadCollections = SimpleCommand(_loadCollections);
    addItem = ParameterizedCommand(_addItem);
  }

  final CollectionsRepository _collectionsRepository;
  List<Collection>? _collections;
  late final SimpleCommand<void> loadCollections;
  late final ParameterizedCommand<void, CollectionItem> addItem;
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

  Future<Result<void>> _addItem(CollectionItem item) async {
    final result = _collectionsRepository.add(
      Collection('New collection', [item]),
    );
    notifyListeners();
    return result;
  }

  Future<Result<void>> initializeCollection(
    String collectionName,
    List<ItemFile> files,
  ) async {
    List<CollectionItem> items = files
        .map(
          (f) => CollectionItem(
            file: f,
            name: f.name,
            metadata: <String, String>{},
            tags: [],
          ),
        )
        .toList();
    notifyListeners();
    return _collectionsRepository.add(Collection(collectionName, items));
  }
}
