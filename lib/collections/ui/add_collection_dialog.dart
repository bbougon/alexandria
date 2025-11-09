import 'dart:collection';

import 'package:alexandria/collections/domain/collections_screen_notifier.dart';
import 'package:flutter/material.dart';

import '../../common/result.dart';
import '../collections.dart';
import '../domain/item_picker.dart';
import 'collection_item_update_form.dart';

class AddCollectionDialog extends StatefulWidget {
  final CollectionsScreenNotifier screenNotifier;

  AddCollectionDialog({super.key, required this.screenNotifier});
  @override
  State<StatefulWidget> createState() => _AddCollectionDialogState();
}

class _AddCollectionDialogState extends State<AddCollectionDialog> {
  List<ItemFile> _files = [];

  void _handleFiles(List<ItemFile?> files) {
    setState(() {
      if (files.isNotEmpty) {
        _files = files.nonNulls.toList();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        children: [
          Row(
            children: [
              TextButton.icon(
                onPressed: () async {
                  var pickedFiles = await FileItemPicker().pickFiles();
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
          Expanded(
            child: ListView.builder(
              padding: EdgeInsets.symmetric(vertical: 8),
              itemCount: _files.length,
              itemBuilder: (context, index) {
                final ItemFile file = _files[index];
                return CollectionItemUpdateForm(
                  file: file,
                  onChanged: (CollectionItem collectionItem) =>
                      widget.screenNotifier.addItem.execute(collectionItem),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
