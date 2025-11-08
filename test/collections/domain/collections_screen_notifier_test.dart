import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/collections_screen_notifier.dart';
import 'package:alexandria/collections/domain/item_picker.dart';
import 'package:alexandria/common/result.dart';
import 'package:flutter_test/flutter_test.dart';

import '../infra/collections_memory_repository.dart';

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
}
