import 'package:alexandria/collections/domain/collections_screen_notifier.dart';
import 'package:uuid/uuid.dart';

import '../common/result.dart';
import 'domain/item_picker.dart';

class Collection {
  final String _name;
  final List<CollectionItem> _items;
  late final String _id;

  Collection(this._name, this._items) {
    _id = Uuid().v4();
  }

  String get id => _id;
  String get name => _name;
  List<CollectionItem> get items => _items;
}

typedef Tags = List<String>;
typedef Metadata = Map<String, String>;

class CollectionItem {
  final ItemFile file;
  final String name;
  final Metadata metadata;
  final Tags tags;

  CollectionItem({
    required this.file,
    required this.name,
    required this.metadata,
    required this.tags,
  });

  CollectionItem newItemWith(CollectionItemUpdate collectionItemUpdate) {
    return CollectionItem(
      file: file,
      metadata: collectionItemUpdate.metadata,
      tags: collectionItemUpdate.tags,
      name: collectionItemUpdate.name,
    );
  }
}

abstract class CollectionsRepository {
  Future<Result<List<Collection>>> all();

  Future<Result<void>> add(Collection collection);

  Future<Result<Collection>> getCollection(String collectionId);
}
