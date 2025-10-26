import 'dart:io';

import 'package:alexandria/video/video_player.dart';
import 'package:flutter/material.dart';

import '../../common/result.dart';
import '../domain/item_picker.dart';

class AddCollectionScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _AddCollectionScreenState();
}

class _AddCollectionScreenState extends State<AddCollectionScreen> {
  List<File> _files = [];

  void _handleFiles(List<ItemFile?> files) {
    List<File> filesToPlay = [];
    for (var file in files) {
      if (file != null) {
        filesToPlay.add(File(file.path));
      }
    }
    setState(() {
      _files = filesToPlay;
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
                final file = _files[index];
                return Padding(
                  padding: const EdgeInsets.symmetric(
                    vertical: 6.0,
                    horizontal: 8,
                  ),
                  child: Row(children: [VideoPlayerWidget(videoFile: file)]),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
