import 'package:alexandria/collections/ui/collection_form.dart';
import 'package:alexandria/collections/ui/collection_items.dart';
import 'package:flutter/material.dart';

import '../../common/error_indicator.dart';
import '../collections.dart';
import '../domain/collections_screen_notifier.dart';
import 'collection_item_update_form.dart';
import 'collections_body.dart';

class CollectionsScreen extends StatefulWidget {
  const CollectionsScreen({super.key, required this.screenNotifier});

  final CollectionsScreenNotifier screenNotifier;

  @override
  State<CollectionsScreen> createState() => _CollectionsScreenState();
}

class _CollectionsScreenState extends State<CollectionsScreen> {
  Collection? _selectedCollection;
  CollectionItem? _selectedItem;

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

  void _onSelectedItem(CollectionItem item) {
    setState(() {
      _selectedItem = item;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.primaryContainer,
      body: Center(
        child: Column(
          children: [
            Row(
              children: [
                TextButton.icon(
                  style: TextButton.styleFrom(
                    foregroundColor: Theme.of(context).colorScheme.onPrimary,
                    backgroundColor: Theme.of(context).colorScheme.primary,
                  ),
                  onPressed: () {
                    showDialog(
                      context: context,
                      builder: (BuildContext context) => Dialog(
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.all(Radius.circular(16)),
                        ),
                        constraints: BoxConstraints(
                          minWidth: 350,
                          minHeight: 250,
                          maxWidth: 350,
                          maxHeight: 250,
                        ),
                        child: Padding(
                          padding: EdgeInsets.all(12.0),
                          child: CollectionForm(
                            notifier: widget.screenNotifier,
                          ),
                        ),
                      ),
                    );
                  },
                  icon: Icon(Icons.add),
                  label: Text('New Collection'),
                ),
              ],
            ),
            Flexible(
              child: Row(
                children: [
                  Expanded(
                    flex: 1,
                    child: Column(
                      children: [
                        Expanded(
                          child: Row(
                            children: [
                              Expanded(
                                child: ListenableBuilder(
                                  listenable: Listenable.merge([
                                    widget.screenNotifier.loadCollections,
                                    widget.screenNotifier.addItem,
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
                                    if (widget
                                        .screenNotifier
                                        .loadCollections
                                        .error) {
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
                                    onSelectedCollection:
                                        (String collectionId) =>
                                            _onSelectedCollection(collectionId),
                                  ),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                  Expanded(
                    flex: 9,
                    child: Row(
                      children: [
                        Expanded(
                          flex: 2,
                          child: Column(
                            children: [
                              CollectionScreen(
                                collection: _selectedCollection,
                                onSelectedItem: _onSelectedItem,
                              ),
                            ],
                          ),
                        ),
                        Expanded(flex: 8, child: displayCollectionItemForm()),
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

  Widget displayCollectionItemForm() {
    final selectedItem = _selectedItem;
    if (selectedItem != null) {
      var selectedItemIndex = _selectedCollection?.items.indexOf(selectedItem);
      return CollectionItemUpdateForm(
        key: Key(
          'collection-item-update-form-${_selectedCollection?.id}-$selectedItemIndex',
        ),
        item: selectedItem,
        onChanged: (CollectionItem collectionItem) =>
            widget.screenNotifier.addItem.execute(collectionItem),
      );
    }
    return Column();
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

  void _onSelectedCollection(String collectionId) async {
    Collection collection = await widget.screenNotifier.getCollection(
      collectionId,
    );
    setState(() {
      _selectedCollection = collection;
    });
  }
}
