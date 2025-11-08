import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/domain/collections_screen_notifier.dart';
import 'package:alexandria/collections/ui/collections_screen.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart' hide Tags;

import '../domain/builders.dart';
import '../infra/collections_memory_repository.dart';

void main() {
  late CollectionsRepository repository;

  setUp(() async {
    repository = CollectionsMemoryRepository();
    await repository.add(
      aCollection()
          .withName('My collection')
          .addItem(aCollectionItem().withName('An item').build())
          .build(),
    );
    await repository.add(
      aCollection()
          .withName('Another collection')
          .addItem(aCollectionItem().withName('Another Item').build())
          .build(),
    );
  });

  testWidgets('Display collections', (tester) async {
    var screenNotifier = CollectionsScreenNotifier(
      collectionsRepository: repository,
    );
    screenNotifier.loadCollections.execute();

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: CollectionsScreen(screenNotifier: screenNotifier)),
      ),
    );

    expect(find.text('My collection'), findsOneWidget);
    expect(find.text('Another collection'), findsOneWidget);
  });

  testWidgets('Display items on tap', (tester) async {
    var screenNotifier = CollectionsScreenNotifier(
      collectionsRepository: repository,
    );
    screenNotifier.loadCollections.execute();

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: CollectionsScreen(screenNotifier: screenNotifier)),
      ),
    );

    await tester.tap(find.text('My collection'));
    await tester.pumpAndSettle();
    expect(find.text('An item'), findsOneWidget);
    await tester.tap(find.text('Another collection'));
    await tester.pumpAndSettle();
    expect(find.text('Another Item'), findsOneWidget);
  });
}
