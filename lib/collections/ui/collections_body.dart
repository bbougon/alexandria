import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/ui/collections_screen_notifier.dart';
import 'package:flutter/material.dart';

class CollectionsBody extends StatelessWidget {
  CollectionsBody({super.key, required this.screenNotifier});

  final CollectionsScreenNotifier screenNotifier;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: screenNotifier,
      builder: (context, _) {
        final collections = screenNotifier.collections;
        if (collections == null) return const SizedBox();

        return CustomScrollView(
          slivers: [
            SliverList(
              delegate: SliverChildBuilderDelegate((context, index) {
                final collection = collections[index];
                return _Collection(collection: collection);
              }, childCount: collections.length),
            ),
            SliverToBoxAdapter(child: SizedBox(height: 100, width: 200)),
          ],
        );
      },
    );
  }
}

class _Collection extends StatelessWidget {
  const _Collection({required this.collection});

  final Collection collection;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.only(top: 16, left: 16, right: 16),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisAlignment: MainAxisAlignment.start,
              children: [
                Text(
                  collection.name.toUpperCase(),
                  style: Theme.of(context).textTheme.labelSmall,
                ),
                Text(
                  collection.name,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: Theme.of(context).textTheme.bodyMedium,
                ),
                Divider(height: 8, thickness: 2.0),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
