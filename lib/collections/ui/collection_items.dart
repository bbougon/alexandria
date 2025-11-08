import 'package:flutter/material.dart';

import '../collections.dart';

class CollectionScreen extends StatefulWidget {
  final Collection? collection;

  CollectionScreen({super.key, required this.collection});

  @override
  State<StatefulWidget> createState() => _CollectionScreenState();
}

class _CollectionScreenState extends State<CollectionScreen> {
  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: ListenableBuilder(
        listenable: Listenable.merge([]),
        builder: (context, child) {
          return child!;
        },
        child: _CollectionBody(collection: widget.collection),
      ),
    );
  }
}

class _CollectionBody extends StatefulWidget {
  final Collection? collection;

  _CollectionBody({super.key, required this.collection});

  @override
  State<StatefulWidget> createState() => _CollectionBodyState();
}

class _CollectionBodyState extends State<_CollectionBody> {
  @override
  Widget build(BuildContext context) {
    return CustomScrollView(
      slivers: [
        SliverList(
          delegate: SliverChildBuilderDelegate((context, index) {
            final item = widget.collection?.items[index];
            return GestureDetector(
              onTap: () {
                showDialog(
                  context: context,
                  builder: (BuildContext context) =>
                      AlertDialog(content: Text(item?.name ?? 'rien')),
                );
              },
              child: _Item(item: item),
            );
          }, childCount: widget.collection?.items.length ?? 0),
        ),
        SliverToBoxAdapter(child: SizedBox(height: 100, width: 200)),
      ],
    );
  }
}

class _Item extends StatelessWidget {
  const _Item({required this.item});

  final CollectionItem? item;

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
                  item?.name ?? '',
                  style: Theme.of(context).textTheme.labelSmall,
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
