import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:file_picker/file_picker.dart';

import '../../common/result.dart';

abstract class ItemPicker {}

abstract class Picker<T> {
  Future<List<ItemFile?>> pickFiles();
}

class PickerFilePicker extends Picker<FilePickerResult> {
  final FilePicker _filePicker;

  PickerFilePicker({required FilePicker filePicker}) : _filePicker = filePicker;

  @override
  Future<List<ItemFile?>> pickFiles() async {
    FilePickerResult? pickFiles = await _filePicker.pickFiles(
      allowMultiple: true,
    );
    var list = pickFiles?.files.map((file) async {
      var bytes = file.bytes;
      String? digest = await retrieveChecksum(file);
      return ItemFile(
        path: file.path ?? '',
        bytes: bytes,
        name: file.name,
        size: file.size,
        checksum: digest,
      );
    }).toList();
    if (list != null) {
      return await Future.wait(list);
    }
    return [];
  }

  Future<String?> retrieveChecksum(PlatformFile file) async {
    Uint8List? bytes = file.bytes;
    String? digest = bytes != null
        ? sha256.convert(bytes.toList()).toString()
        : null;
    return digest;
  }
}

class ItemFile {
  final String path;
  final String name;
  final int size;
  final Uint8List? bytes;
  final String? checksum;
  final File file;

  ItemFile({
    required this.path,
    required this.name,
    required this.size,
    this.bytes,
    required this.checksum,
  }) : file = File(path);

  @override
  String toString() {
    return 'Item : $name in $path with checksum $checksum and bytes $bytes';
  }
}

class FileItemPicker extends ItemPicker {
  final PickerFilePicker _filePicker;

  FileItemPicker({FilePicker? filePicker})
    : _filePicker = filePicker == null
          ? PickerFilePicker(filePicker: FilePicker.platform)
          : PickerFilePicker(filePicker: filePicker);

  Future<Result<List<ItemFile?>>> pickFiles() async {
    List<ItemFile?> result = await _filePicker.pickFiles();
    return Result.ok(result);
  }
}
