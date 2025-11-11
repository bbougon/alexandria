import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/ui/text_field_tags_controller.dart';
import 'package:flutter/material.dart';

import '../../common/colors.dart';
import '../../video/video_player.dart';
import '../domain/item_picker.dart';
import 'text_field_tags.dart';

class CollectionItemUpdateForm extends StatefulWidget {
  final ItemFile file;
  late final TextEditingController _collectionNameController;
  late final MetadataController _metadataController;
  late final StringTagController<String> _tagsController;
  final ValueChanged<CollectionItem> onChanged;

  CollectionItemUpdateForm({
    super.key,
    required this.file,
    required this.onChanged,
  }) {
    _collectionNameController = TextEditingController(text: file.name);
    _metadataController = MetadataController();
    _tagsController = StringTagController();
  }

  @override
  State<StatefulWidget> createState() => _CollectionItemUpdateFormState();
}

const List<String> metadataKeys = <String>['Author', 'Style', 'Title'];

class _CollectionItemUpdateFormState extends State<CollectionItemUpdateForm> {
  late CollectionItem _collectionItem;

  void _onChanged() {
    setState(() {
      _collectionItem = CollectionItem(
        file: widget.file,
        name: widget._collectionNameController.text,
        metadata: widget._metadataController.metadata,
        tags: widget._tagsController.getTags ?? [],
      );
    });
    widget.onChanged(_collectionItem);
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6.0, horizontal: 8),
      child: SizedBox(
        child: Column(
          spacing: 16,
          children: [
            Row(
              children: [
                Flexible(
                  flex: 10,
                  child: Center(
                    child: SizedBox(
                      height: 400,
                      width: 600,
                      child: VideoPlayerWidget(videoFile: widget.file.file),
                    ),
                  ),
                ),
              ],
            ),
            Row(
              children: [
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
              ],
            ),
            Row(
              spacing: 20,
              mainAxisAlignment: MainAxisAlignment.start,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Flexible(
                  flex: 3,
                  fit: FlexFit.loose,
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      ...metadataKeys.map(
                        (key) => MetadataField(
                          keyName: key,
                          controller: widget._metadataController,
                        ),
                      ),
                    ],
                  ),
                ),
                Flexible(
                  flex: 7,
                  child: Tags(controller: widget._tagsController),
                ),
              ],
            ),
            Expanded(
              child: Align(
                alignment: Alignment.bottomRight,
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    ElevatedButton.icon(
                      onPressed: () => _onChanged(),
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
            children: [
              TextFormField(
                controller: widget.textEditingController,
                style: TextStyle(fontSize: 14),
                decoration: InputDecoration(
                  border: const UnderlineInputBorder(),
                  labelText: 'Name',
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class Tags extends StatefulWidget {
  final StringTagController controller;

  Tags({super.key, required this.controller});

  @override
  State<StatefulWidget> createState() => _TagsState();
}

class _TagsState extends State<Tags> {
  late double _distanceToField;

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
          textfieldTagsController: widget.controller,
          textSeparators: const [' ', ','],
          inputFieldBuilder: (context, inputFieldValues) => TextField(
            controller: inputFieldValues.textEditingController,
            focusNode: inputFieldValues.focusNode,
            maxLines: 5,
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

class Metadata {
  final String key;
  final String value;

  const Metadata(this.key, this.value);
}

class MetadataController extends ChangeNotifier {
  Map<String, String> metadata = {};

  void addMetadata(Metadata metadata) {
    this.metadata[metadata.key] = metadata.value;
    notifyListeners();
  }
}

class MetadataField extends StatefulWidget {
  final MetadataController controller;
  final String keyName;

  MetadataField({super.key, required this.controller, required this.keyName});

  @override
  State<StatefulWidget> createState() => _MetadataFieldState();
}

class _MetadataFieldState extends State<MetadataField> {
  TextEditingController _metadataValueController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: [
        TextFormField(
          key: Key('metadata-field-value-${widget.keyName.toLowerCase()}'),
          controller: _metadataValueController,
          decoration: InputDecoration(
            border: const UnderlineInputBorder(),
            labelText: widget.keyName,
          ),
          onChanged: (value) => widget.controller.addMetadata(
            Metadata(widget.keyName, _metadataValueController.text),
          ),
        ),
      ],
    );
  }
}
