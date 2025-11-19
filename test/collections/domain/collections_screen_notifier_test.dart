import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/collections_screen_notifier.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/common/command.dart';
import 'package:alexandria/common/result.dart';
import 'package:flutter_test/flutter_test.dart';

import '../infra/collections_memory_repository.dart';
import 'builders.dart';

void main() {
  test('Collections notifier', () async {
    CollectionsRepository repository = CollectionsMemoryRepository();
    CollectionsScreenNotifier notifier = CollectionsScreenNotifier(
      collectionsRepository: repository,
    );

    Result result = (await notifier.initializeCollection('My collection', [
      ItemFile(path: 'a path', name: 'A file name', size: 2, checksum: '3'),
    ]));

    expect((result as Ok).value, null);
    List<Collection> collections = ((await repository.all()) as Ok).value;
    expect(collections[0].id, isA<String>());
    expect(collections[0].id, hasLength(36));
  });

  test('Update an item of a collection', () async {
    CollectionsRepository repository = CollectionsMemoryRepository();
    await repository.add(
      aCollection()
          .addItem(aCollectionItem().withName('First Item').build())
          .build(),
    );
    var collectionItemToBeUpdated = aCollectionItem()
        .withName('Second Item')
        .build();
    var collectionToBeUpdated = aCollection()
        .addItem(aCollectionItem().withName('New Item').build())
        .addItem(collectionItemToBeUpdated)
        .build();
    await repository.add(collectionToBeUpdated);
    CollectionsScreenNotifier notifier = CollectionsScreenNotifier(
      collectionsRepository: repository,
    );

    ParameterizedCommand<void, UpdateCollectionItemCommand> updateCollection =
        notifier.updateCollectionItem;
    await updateCollection.execute(
      UpdateCollectionItemCommand(
        collectionToBeUpdated: collectionToBeUpdated,
        collectionItemToBeUpdated: collectionItemToBeUpdated,
        collectionItemUpdate: CollectionItemUpdate(
          name: "A new name",
          metadata: {"Author": "An author", "Title": "A title"},
          tags: ["Tag1", "Tag2"],
        ),
      ),
    );
    Result<void>? result = updateCollection.result;

    expect((result as Ok).value, null);
    Collection collection =
        ((await repository.getCollection(collectionToBeUpdated.id)) as Ok)
            .value;
    expect(collection.items[1].name, "A new name");
    expect(collection.items[1].metadata, {
      "Author": "An author",
      "Title": "A title",
    });
    expect(collection.items[1].tags, ["Tag1", "Tag2"]);
  });
}
