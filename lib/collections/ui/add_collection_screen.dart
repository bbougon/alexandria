import 'dart:collection';

import 'package:alexandria/video/video_player.dart';
import 'package:flutter/material.dart';

import '../../common/result.dart';
import '../domain/item_picker.dart';

class AddCollectionScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _AddCollectionScreenState();
}

class _AddCollectionScreenState extends State<AddCollectionScreen> {
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
                return Padding(
                  padding: const EdgeInsets.symmetric(
                    vertical: 6.0,
                    horizontal: 8,
                  ),
                  child: SizedBox(
                    height: 200,
                    child: Row(
                      spacing: 16,
                      children: [
                        Flexible(
                          child: SizedBox(
                            width: 300,
                            child: VideoPlayerWidget(videoFile: file.file),
                          ),
                        ),
                        Expanded(
                          child: Column(children: [CollectionForm(file: file)]),
                        ),
                        Expanded(child: Column(children: [MetadataRow()])),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

class CollectionForm extends StatefulWidget {
  final ItemFile file;

  CollectionForm({super.key, required this.file});

  @override
  State<StatefulWidget> createState() => _CollectionFormState();
}

class _CollectionFormState extends State<CollectionForm> {
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Flexible(
          child: Column(
            spacing: 1,
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: [
              Wrap(
                spacing: 10,
                children: [
                  Padding(
                    padding: const EdgeInsets.symmetric(
                      vertical: 10.0,
                      horizontal: 8,
                    ),
                    child: TextFormField(
                      style: TextStyle(fontSize: 14),
                      initialValue: widget.file.name,
                      decoration: InputDecoration(
                        border: const UnderlineInputBorder(),
                        labelText: 'Name',
                      ),
                    ),
                  ),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: [
                      MetadataDropDownMenu(),
                      Spacer(flex: 1),
                      Flexible(
                        flex: 2,
                        child: TextFormField(
                          decoration: InputDecoration(
                            border: const UnderlineInputBorder(),
                            labelText: 'Value',
                          ),
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class MetadataRow extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _MetadataRowState();
}

class _MetadataRowState extends State<MetadataRow> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TextField(
          maxLines: 5,
          decoration: InputDecoration(
            border: OutlineInputBorder(),
            labelText: 'Tags',
          ),
        ),
      ],
    );
  }
}

class MetadataDropDownMenu extends StatefulWidget {
  const MetadataDropDownMenu({super.key});

  @override
  State<MetadataDropDownMenu> createState() => _MetadataDropDownMenuState();
}

typedef MetadataEntry = DropdownMenuEntry<String>;
const List<String> list = <String>['Author', 'Style'];

class _MetadataDropDownMenuState extends State<MetadataDropDownMenu> {
  static final List<MetadataEntry> menuEntries =
      UnmodifiableListView<MetadataEntry>(
        list.map<MetadataEntry>(
          (String name) => MetadataEntry(value: name, label: name),
        ),
      );
  String dropdownValue = list.first;

  @override
  Widget build(BuildContext context) {
    return IconTheme(
      data: const IconThemeData(size: 8),
      child: DropdownMenu<String>(
        initialSelection: list.first,
        onSelected: (value) => setState(() => dropdownValue = value!),
        dropdownMenuEntries: menuEntries,
        textStyle: const TextStyle(fontSize: 14),
        inputDecorationTheme: const InputDecorationTheme(
          isDense: true,
          suffixIconConstraints: BoxConstraints(minWidth: 8, minHeight: 8),
          border: OutlineInputBorder(),
        ),
      ),
    );
  }
}
