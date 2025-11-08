import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:faker/faker.dart';

abstract class Builder<T> {
  T build();
}

class CollectionBuilder implements Builder<Collection> {
  late String _name;
  List<CollectionItem> _items = [];

  CollectionBuilder withName(String name) {
    _name = name;
    return this;
  }

  CollectionBuilder addItem(CollectionItem collectionItem) {
    _items.add(collectionItem);
    return this;
  }

  @override
  Collection build() {
    return Collection(_name, _items);
  }
}

class ItemFileBuilder implements Builder<ItemFile> {
  String _path = faker.lorem.word();
  String _name = faker.lorem.word();
  int _size = faker.randomGenerator.integer(10000);
  String? _checksum = faker.randomGenerator.string(32);

  @override
  ItemFile build() {
    return ItemFile(path: _path, name: _name, size: _size, checksum: _checksum);
  }
}

class CollectionItemBuilder implements Builder<CollectionItem> {
  late String _name;
  late ItemFile _file = anItemFile().build();
  Map<String, String> _metadata = {};
  Tags _tags = [];

  @override
  CollectionItem build() {
    return CollectionItem(
      file: _file,
      name: _name,
      metadata: _metadata,
      tags: _tags,
    );
  }

  CollectionItemBuilder withName(String name) {
    _name = name;
    return this;
  }
}

ItemFileBuilder anItemFile() => ItemFileBuilder();

CollectionBuilder aCollection() => CollectionBuilder();

CollectionItemBuilder aCollectionItem() => CollectionItemBuilder();
