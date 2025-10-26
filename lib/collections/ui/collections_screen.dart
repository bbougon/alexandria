import 'package:alexandria/collections/ui/add_collection_screen.dart';
import 'package:flutter/material.dart';

import '../../common/error_indicator.dart';
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
                      builder: (BuildContext context) => Dialog.fullscreen(
                        child: Scaffold(
                          backgroundColor: Theme.of(
                            context,
                          ).colorScheme.primaryContainer,
                          floatingActionButton: ElevatedButton.icon(
                            label: Text('Close'),
                            onPressed: () => Navigator.pop(context),
                            icon: Icon(Icons.close),
                          ),
                          body: SafeArea(child: AddCollectionScreen()),
                        ),
                      ),
                    );
                  },
                  icon: Icon(Icons.add),
                  label: Text('New Collection'),
                ),
              ],
            ),
            Expanded(
              child: Column(
                children: [
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
