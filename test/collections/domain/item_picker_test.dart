import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/common/result.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('FileItemPicker should return files', () async {
    var filesResult = await FileItemPicker(
      filePicker: TestPicker.with2Files(),
    ).pickFiles();

    expect((filesResult as Ok<List<ItemFile?>>).value.length, 2);
  });

  test('FileItemPicker should return an empty list', () async {
    var filesResult = await FileItemPicker(
      filePicker: TestPicker.withNoFile(),
    ).pickFiles();

    expect((filesResult as Ok<List<ItemFile?>>).value.length, 0);
  });

  test('FileItemPicker should return a result with expected fields', () async {
    var filesResult = await FileItemPicker(
      filePicker: TestPicker(
        filePickerResult: FilePickerResult([
          TestPickerResultBuilder()
              .withName('file name')
              .withPath('a path')
              .withSize(1)
              .withBytes(Uint8List(1))
              .build()
              .result(),
        ]),
      ),
    ).pickFiles();

    expect((filesResult as Ok<List<ItemFile?>>).value[0]?.size, 1);
    expect((filesResult).value[0]?.name, "file name");
    expect((filesResult).value[0]?.path, "a path");
    expect((filesResult).value[0]?.bytes, Uint8List(1));
    expect(
      filesResult.value[0]?.checksum,
      '6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d',
    );
    expect((filesResult).value[0]?.file, isNotNull);
  });

  test(
    'FileItemPicker should return a result without bytes and checksum',
    () async {
      var filesResult = await FileItemPicker(
        filePicker: TestPicker(
          filePickerResult: FilePickerResult([
            TestPickerResultBuilder()
                .withName('file name')
                .withPath('a path')
                .withSize(1)
                .build()
                .result(),
          ]),
        ),
      ).pickFiles();

      expect((filesResult as Ok<List<ItemFile?>>).value[0]?.size, 1);
      expect((filesResult).value[0]?.bytes, null);
      expect(filesResult.value[0]?.checksum, null);
    },
  );

  test(
    'FileItemPicker should return the checksum from a read stream',
    () async {
      var filesResult = await FileItemPicker(
        filePicker: TestPicker(
          filePickerResult: FilePickerResult([
            TestPickerResultBuilder()
                .withName('file name')
                .withPath('a path')
                .withSize(1)
                .withReadStreamFor(Uint8List(1))
                .build()
                .result(),
          ]),
        ),
      ).pickFiles();

      expect((filesResult as Ok<List<ItemFile?>>).value[0]?.size, 1);
      expect((filesResult).value[0]?.name, "file name");
      expect((filesResult).value[0]?.path, "a path");
      expect(
        filesResult.value[0]?.checksum,
        '6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d',
      );
    },
  );
}

class TestPickerResultBuilder {
  String? _name;
  String? _path;
  int? _size;
  Uint8List? _bytes;
  Stream<List<int>>? _readStream;

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
    _readStream = Stream.value(List<int>.from(bytes));
    return this;
  }

  TestPickerResultBuilder withReadStreamFor(Uint8List bytes) {
    _readStream = Stream.value(List<int>.from(bytes));
    return this;
  }

  TestPickerResult build() {
    return TestPickerResult(
      name: _name ?? 'A name',
      size: _size ?? 3,
      path: _path,
      bytes: _bytes,
      readStream: _readStream,
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
    Stream<List<int>>? readStream,
  }) : _name = name,
       _size = size,
       _path = path,
       _bytes = bytes,
       _readStream = readStream;

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
