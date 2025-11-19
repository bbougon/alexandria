import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/common/command.dart';
import 'package:flutter/material.dart';

import '../../common/result.dart';

class UpdateCollectionItemCommand {
  Collection collectionToBeUpdated;
  CollectionItem collectionItemToBeUpdated;
  CollectionItemUpdate collectionItemUpdate;

  UpdateCollectionItemCommand({
    required this.collectionToBeUpdated,
    required this.collectionItemToBeUpdated,
    required this.collectionItemUpdate,
  });
}

class CollectionsScreenNotifier extends ChangeNotifier {
  CollectionsScreenNotifier({
    required CollectionsRepository collectionsRepository,
  }) : _collectionsRepository = collectionsRepository {
    loadCollections = SimpleCommand(_loadCollections);
    addItem = ParameterizedCommand(_addItem);
    updateCollectionItem = ParameterizedCommand(_updateCollectionItem);
  }

  final CollectionsRepository _collectionsRepository;
  List<Collection>? _collections;
  late final SimpleCommand<void> loadCollections;
  late final ParameterizedCommand<void, CollectionItem> addItem;
  late final ParameterizedCommand<void, UpdateCollectionItemCommand>
  updateCollectionItem;
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

  Future<Collection> getCollection(String collectionId) async {
    var result = await _collectionsRepository.getCollection(collectionId);
    switch (result) {
      case Ok<Collection>():
        return result.value;
      case Error<Collection>():
        throw UnimplementedError("Something wrong");
    }
  }

  Future<Result<void>> _updateCollectionItem(
    UpdateCollectionItemCommand command,
  ) async {
    CollectionItem foundItem = command.collectionToBeUpdated.items.firstWhere(
      (item) => item.file.path == command.collectionItemToBeUpdated.file.path,
    );
    command.collectionToBeUpdated.items[command.collectionToBeUpdated.items
        .indexOf(foundItem)] = foundItem.newItemWith(
      command.collectionItemUpdate,
    );
    return Result.ok(null);
  }
}

class CollectionItemUpdate {
  String name;
  Map<String, String> metadata;
  Tags tags;

  CollectionItemUpdate({
    required this.name,
    required this.metadata,
    required this.tags,
  });
}
