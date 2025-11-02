import '../common/result.dart';
import 'domain/item_picker.dart';

class Collection {
  final String _name;
  final List<CollectionItem> _items;

  Collection(this._name, this._items);

  String get name => _name;
  List<CollectionItem> get items => _items;
}

typedef Tags = List<String>;

class CollectionItem {
  final ItemFile file;
  final String name;
  final Map<String, String> metadata;
  final Tags tags;

  CollectionItem({
    required this.file,
    required this.name,
    required this.metadata,
    required this.tags,
  });
}

abstract class CollectionsRepository {
  Future<Result<List<Collection>>> all();

  Future<Result<void>> add(Collection collection);
}
