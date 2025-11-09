import 'package:alexandria/collections/ui/collection_item_update_form.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('MetadataField is initialized', (tester) async {
    MetadataController controller = MetadataController();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: MetadataField(controller: controller)),
      ),
    );

    expect(find.text('Author'), findsNWidgets(2));
    var dropDownMenu = find.byKey(Key('metadata-field-key'));
    var widget2 = dropDownMenu.evaluate().single.widget as DropdownMenu;
    expect(widget2.initialSelection, 'Author');
    expect(find.byType(TextFormField), findsOneWidget);
    var metadataFieldValue = find.byKey(Key('metadata-field-value'));
    expect(
      (metadataFieldValue.evaluate().single.widget as TextFormField)
          .initialValue,
      isEmpty,
    );
  });

  testWidgets('MetadataField adds Metadata', (tester) async {
    MetadataController controller = MetadataController();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: MetadataField(controller: controller)),
      ),
    );

    await tester.enterText(find.byType(TextFormField), 'RATM');

    expect(find.text('RATM'), findsOneWidget);
    expect(controller.metadata['Author'], 'RATM');
  });
}
