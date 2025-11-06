import 'package:flutter/material.dart';

import '../../common/result.dart';
import '../domain/collections_screen_notifier.dart';
import '../domain/item_picker.dart';

class CollectionForm extends StatefulWidget {
  final CollectionsScreenNotifier notifier;
  late final FileItemPicker _picker;

  CollectionForm({super.key, required this.notifier, FileItemPicker? picker}) {
    _picker = picker ??= FileItemPicker();
  }

  @override
  State<StatefulWidget> createState() => _CollectionFormState();
}

class _CollectionFormState extends State<CollectionForm> {
  TextEditingController _collectionNameController = TextEditingController();

  List<ItemFile> _files = [];

  void _handleFiles(List<ItemFile?> files) {
    setState(() {
      if (files.isNotEmpty) {
        _files = files.nonNulls.toList();
      }
    });
  }

  Future<void> _createCollection() async {
    await widget.notifier.initializeCollection(
      _collectionNameController.text,
      _files,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Row(
          children: [
            Text(
              "Create a collection".toUpperCase(),
              style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
            ),
          ],
        ),
        SizedBox(height: 15),
        TextFormField(
          controller: _collectionNameController,
          decoration: InputDecoration(
            border: const UnderlineInputBorder(),
            labelText: 'Collection name',
          ),
        ),
        SizedBox(height: 15),
        Row(
          children: [
            TextButton.icon(
              key: Key('openPicker'),
              onPressed: () async {
                var pickedFiles = await widget._picker.pickFiles();
                switch (pickedFiles) {
                  case Ok<List<ItemFile?>>():
                    _handleFiles(pickedFiles.value);
                  case Error<List<ItemFile?>>():
                    throw UnimplementedError();
                }
              },
              label: Text('...browse'),
            ),
          ],
        ),
        SizedBox(height: 15),
        Row(
          children: [
            TextButton(
              key: Key('createCollection'),
              onPressed: () async {
                await _createCollection();
                if (context.mounted) {
                  Navigator.pop(context);
                }
              },
              child: Text('Create'),
            ),
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: Text('Cancel'),
            ),
          ],
        ),
      ],
    );
  }
}
