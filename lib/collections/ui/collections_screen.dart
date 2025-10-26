import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:flutter/material.dart';

import '../../common/error_indicator.dart';
import '../../common/result.dart';
import 'collections_body.dart';
import 'collections_screen_notifier.dart';

class CollectionsScreen extends StatefulWidget {
  const CollectionsScreen({super.key, required this.screenNotifier});

  final CollectionsScreenNotifier screenNotifier;

  @override
  State<CollectionsScreen> createState() => _CollectionsScreenState();
}

class _CollectionsScreenState extends State<CollectionsScreen> {
  @override
  void initState() {
    super.initState();
    widget.screenNotifier.loadCollections.addListener(_listener);
  }

  @override
  void dispose() {
    widget.screenNotifier.loadCollections.removeListener(_listener);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.primaryContainer,
      body: Center(
        child: Column(
          children: [
            Expanded(
              child: Column(
                children: [
                  Row(
                    children: [
                      TextButton.icon(
                        style: TextButton.styleFrom(
                          foregroundColor: Theme.of(
                            context,
                          ).colorScheme.onPrimary,
                          backgroundColor: Theme.of(
                            context,
                          ).colorScheme.primary,
                        ),
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
                        icon: Icon(Icons.add),
                        label: Text('New Collection'),
                      ),
                    ],
                  ),
                  Expanded(
                    child: Row(
                      children: [
                        Expanded(
                          child: ListenableBuilder(
                            listenable: Listenable.merge([
                              widget.screenNotifier.loadCollections,
                            ]),
                            builder: (context, child) {
                              if (widget
                                  .screenNotifier
                                  .loadCollections
                                  .running) {
                                return const Center(
                                  child: CircularProgressIndicator(),
                                );
                              }
                              if (widget.screenNotifier.loadCollections.error) {
                                return Center(
                                  child: ErrorIndicator(
                                    title: 'Erreur',
                                    label: 'Erreur',
                                    onPressed: () => {},
                                  ),
                                );
                              }
                              return child!;
                            },
                            child: CollectionsBody(
                              screenNotifier: widget.screenNotifier,
                            ),
                          ),
                        ),
                      ],
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

  void _listener() {
    if (widget.screenNotifier.loadCollections.error) {
      widget.screenNotifier.loadCollections.clearResult();
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Erreur lors duchargement'),
          action: SnackBarAction(
            label: 'Recharge de nouveau',
            onPressed: widget.screenNotifier.loadCollections.execute,
          ),
        ),
      );
    }
  }
}
