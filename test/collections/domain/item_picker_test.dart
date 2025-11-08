import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/common/result.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';

import 'builders.dart';

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
}
