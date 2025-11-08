import 'dart:typed_data';

import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:faker/faker.dart';
import 'package:file_picker/file_picker.dart';

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

class TestPickerResultBuilder implements Builder<TestPickerResult> {
  String? _name;
  String? _path;
  int? _size;
  Uint8List? _bytes;

  TestPickerResultBuilder withName(String name) {
    _name = name;
    return this;
  }

  TestPickerResultBuilder withPath(String path) {
    _path = path;
    return this;
  }

  TestPickerResultBuilder withSize(int size) {
    _size = size;
    return this;
  }

  TestPickerResultBuilder withBytes(Uint8List bytes) {
    _bytes = bytes;
    return this;
  }

  @override
  TestPickerResult build() {
    return TestPickerResult(
      name: _name ?? 'A name',
      size: _size ?? 3,
      path: _path,
      bytes: _bytes,
    );
  }
}

class TestPickerResult {
  String _name = 'A name';
  int _size = 1;
  String? _path;
  Uint8List? _bytes;
  Stream<List<int>>? _readStream;

  TestPickerResult({
    required String name,
    required int size,
    String? path,
    Uint8List? bytes,
  }) : _name = name,
       _size = size,
       _path = path,
       _bytes = bytes;

  static PlatformFile aResult() {
    return TestPickerResult(name: 'A name', size: 1).result();
  }

  PlatformFile result() {
    return PlatformFile(
      name: _name,
      size: _size,
      bytes: _bytes,
      path: _path,
      readStream: _readStream,
    );
  }
}

class TestPicker extends FilePicker {
  final FilePickerResult? _filePickerResult;

  TestPicker({FilePickerResult? filePickerResult})
    : _filePickerResult = filePickerResult;

  @override
  Future<FilePickerResult?> pickFiles({
    String? dialogTitle,
    String? initialDirectory,
    FileType type = FileType.any,
    List<String>? allowedExtensions,
    Function(FilePickerStatus)? onFileLoading,
    @Deprecated(
      'allowCompression is deprecated and has no effect. Use compressionQuality instead.',
    )
    bool allowCompression = false,
    int compressionQuality = 0,
    bool allowMultiple = false,
    bool withData = false,
    bool withReadStream = false,
    bool lockParentWindow = false,
    bool readSequential = false,
  }) async {
    return _filePickerResult;
  }

  static FilePicker? with2Files() {
    return TestPicker(
      filePickerResult: FilePickerResult([
        TestPickerResult.aResult(),
        TestPickerResult.aResult(),
      ]),
    );
  }

  static FilePicker? withNoFile() {
    return TestPicker();
  }
}

ItemFileBuilder anItemFile() => ItemFileBuilder();

CollectionBuilder aCollection() => CollectionBuilder();

CollectionItemBuilder aCollectionItem() => CollectionItemBuilder();
