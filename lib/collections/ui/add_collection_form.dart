import 'dart:collection';

import 'package:alexandria/collections/ui/text_field_tags_controller.dart';
import 'package:flutter/material.dart';

import '../../common/colors.dart';
import '../../video/video_player.dart';
import '../domain/item_picker.dart';
import 'text_field_tags.dart';

class AddCollectionForm extends StatefulWidget {
  final ItemFile file;
  late final TextEditingController _collectionNameController;

  AddCollectionForm({super.key, required this.file}) {
    _collectionNameController = TextEditingController(text: file.name);
  }

  @override
  State<StatefulWidget> createState() => _AddCollectionFormState();
}

class _AddCollectionFormState extends State<AddCollectionForm> {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6.0, horizontal: 8),
      child: SizedBox(
        height: 200,
        child: Row(
          spacing: 16,
          children: [
            Flexible(
              child: SizedBox(
                width: 300,
                child: VideoPlayerWidget(videoFile: widget.file.file),
              ),
            ),
            SizedBox(
              width: 300,
              child: Column(
                children: [
                  CollectionName(
                    textEditingController: widget._collectionNameController,
                  ),
                ],
              ),
            ),
            SizedBox(
              width: 300,
              child: Column(
                children: [
                  Tags(),
                  MetadataField(),
                  Expanded(
                    child: Align(
                      alignment: Alignment.bottomRight,
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.end,
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          ElevatedButton.icon(
                            onPressed: () {
                              showDialog(
                                context: context,
                                builder: (BuildContext context) => AlertDialog(
                                  title: Text(
                                    widget._collectionNameController.text,
                                  ),
                                ),
                              );
                            },
                            label: Text('Submit'),
                            icon: Icon(Icons.send),
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class CollectionName extends StatefulWidget {
  final TextEditingController textEditingController;

  CollectionName({super.key, required this.textEditingController});

  @override
  State<StatefulWidget> createState() => _CollectionNameState();
}

class _CollectionNameState extends State<CollectionName> {
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
                      controller: widget.textEditingController,
                      style: TextStyle(fontSize: 14),
                      decoration: InputDecoration(
                        border: const UnderlineInputBorder(),
                        labelText: 'Name',
                      ),
                    ),
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

class Tags extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _TagsState();
}

class _TagsState extends State<Tags> {
  late double _distanceToField;
  final _stringTagController = StringTagController();

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _distanceToField = MediaQuery.of(context).size.width;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TextFieldTags(
          textfieldTagsController: _stringTagController,
          textSeparators: const [' ', ','],
          inputFieldBuilder: (context, inputFieldValues) => TextField(
            controller: inputFieldValues.textEditingController,
            focusNode: inputFieldValues.focusNode,
            maxLines: 3,
            decoration: InputDecoration(
              isDense: true,
              border: const OutlineInputBorder(
                borderSide: BorderSide(color: grey, width: 2.0),
              ),
              focusedBorder: const OutlineInputBorder(
                borderSide: BorderSide(color: grey, width: 2.0),
              ),
              hintText: inputFieldValues.tags.isNotEmpty ? '' : "Enter tag...",
              errorText: inputFieldValues.error,
              prefixIconConstraints: BoxConstraints(
                maxWidth: _distanceToField * 0.8,
              ),
              prefixIcon: inputFieldValues.tags.isNotEmpty
                  ? SingleChildScrollView(
                      controller: inputFieldValues.tagScrollController,
                      scrollDirection: Axis.vertical,
                      child: Padding(
                        padding: const EdgeInsets.only(
                          top: 4,
                          bottom: 4,
                          left: 4,
                        ),
                        child: Wrap(
                          runSpacing: 2.0,
                          spacing: 2.0,
                          children: inputFieldValues.tags.map((String tag) {
                            return Container(
                              decoration: const BoxDecoration(
                                borderRadius: BorderRadius.all(
                                  Radius.circular(20.0),
                                ),
                                color: grey,
                              ),
                              margin: const EdgeInsets.symmetric(
                                horizontal: 3.0,
                              ),
                              padding: const EdgeInsets.symmetric(
                                horizontal: 5.0,
                                vertical: 3.0,
                              ),
                              child: Row(
                                mainAxisAlignment: MainAxisAlignment.start,
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  InkWell(
                                    child: Text(
                                      '#$tag',
                                      style: const TextStyle(
                                        color: Colors.white,
                                      ),
                                    ),
                                  ),
                                  const SizedBox(width: 2.0),
                                  InkWell(
                                    child: const Icon(
                                      Icons.cancel,
                                      size: 14.0,
                                      color: Color.fromARGB(255, 233, 233, 233),
                                    ),
                                    onTap: () {
                                      inputFieldValues.onTagRemoved(tag);
                                    },
                                  ),
                                ],
                              ),
                            );
                          }).toList(),
                        ),
                      ),
                    )
                  : null,
            ),
            onChanged: inputFieldValues.onTagChanged,
            onSubmitted: inputFieldValues.onTagSubmitted,
          ),
        ),
      ],
    );
  }
}

class MetadataField extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _MetadataFieldState();
}

class _MetadataFieldState extends State<MetadataField> {
  @override
  Widget build(BuildContext context) {
    return Row(
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
