import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/collections_screen_notifier.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/collections/ui/collection_form.dart';
import 'package:alexandria/common/result.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import '../domain/item_picker_test.dart';
import '../infra/collections_memory_repository.dart';

class CollectionFormWrapper extends StatelessWidget {
  final CollectionForm collectionForm;

  CollectionFormWrapper({required this.collectionForm});

  @override
  Widget build(BuildContext context) {
    return TextButton.icon(
      key: Key('testButton'),
      onPressed: () => showDialog(
        context: context,
        builder: (BuildContext context) => Dialog(child: collectionForm),
      ),
      icon: Icon(Icons.add),
      label: Text('Test collection form'),
    );
  }
}

void main() {
  testWidgets('Collection form creates a collection', (tester) async {
    CollectionsRepository repository = CollectionsMemoryRepository();

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: CollectionFormWrapper(
            collectionForm: CollectionForm(
              notifier: CollectionsScreenNotifier(
                collectionsRepository: repository,
              ),
              picker: FileItemPicker(filePicker: TestPicker.with2Files()),
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.byKey(Key('testButton')));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextFormField), 'A collection');
    await tester.tap(find.byKey(const Key('openPicker')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(Key('createCollection')));
    await tester.pumpAndSettle();

    List<Collection> collections =
        ((await repository.all()) as Ok<List<Collection>>).value;
    expect(collections.length, 1);
    expect(collections[0].name, 'A collection');
    expect(collections[0].items.length, 2);
    expect(collections[0].items[0].name, 'A name');
    expect(collections[0].items[1].name, 'A name');
    expect(find.byType(CollectionForm), findsNothing);
  });
}
