import 'package:flutter/material.dart';

import '../../common/result.dart';
import '../domain/item_picker.dart';

class AddCollectionScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _AddCollectionScreenState();
}

class _AddCollectionScreenState extends State<AddCollectionScreen> {
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
                      for (var file in pickedFiles.value) {
                        print(file);
                      }
                    case Error<List<ItemFile?>>():
                      throw UnimplementedError();
                  }
                },
                label: Text('...browse'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
