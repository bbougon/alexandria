import 'package:alexandria/collections/ui/collection_item_update_form.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('MetadataField adds Metadata', (tester) async {
    MetadataController controller = MetadataController();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: MetadataField(keyName: 'Author', controller: controller),
        ),
      ),
    );

    await tester.enterText(
      find.byKey(Key('metadata-field-value-author')),
      'RATM',
    );

    expect(find.text('RATM'), findsOneWidget);
    expect(controller.metadata['Author'], 'RATM');
  });
}
