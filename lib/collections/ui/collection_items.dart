import 'package:flutter/material.dart';

import '../collections.dart';

class CollectionScreen extends StatefulWidget {
  final Collection? collection;
  final ValueChanged<CollectionItem> onSelectedItem;

  CollectionScreen({
    super.key,
    required this.collection,
    required this.onSelectedItem,
  });

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
        child: _CollectionBody(
          collection: widget.collection,
          onSelectedItem: widget.onSelectedItem,
        ),
      ),
    );
  }
}

class _CollectionBody extends StatefulWidget {
  final Collection? collection;
  final ValueChanged<CollectionItem> onSelectedItem;

  _CollectionBody({required this.collection, required this.onSelectedItem});

  @override
  State<StatefulWidget> createState() => _CollectionBodyState();
}

class _CollectionBodyState extends State<_CollectionBody> {
  CollectionItem? _selectedItem;

  @override
  Widget build(BuildContext context) {
    return CustomScrollView(
      slivers: [
        SliverList(
          delegate: SliverChildBuilderDelegate((context, index) {
            final item = widget.collection?.items[index];
            return GestureDetector(
              onTap: () {
                widget.onSelectedItem(item!);
                setState(() {
                  _selectedItem = item;
                });
              },
              child: Container(
                color: item == _selectedItem
                    ? Theme.of(context).colorScheme.inversePrimary
                    : Theme.of(context).colorScheme.primaryContainer,
                child: _Item(item: item),
              ),
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
