import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/collections/ui/collections_screen_notifier.dart';
import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import 'package:provider/provider.dart';

import 'collections/infra/collections_file_repository.dart';
import 'collections/ui/collections_screen.dart';
import 'home/page.dart';

void main() {
  Logger.root.level = Level.ALL;
  Logger.root.onRecord.listen((record) {
    print(
      '${record.level.name}: ${record.loggerName} ${record.time}: ${record.message}',
    );
  });
  // debugPaintSizeEnabled = true;
  runApp(
    MultiProvider(
      providers: [
        Provider(
          create: (context) =>
              CollectionsFileRepository() as CollectionsRepository,
        ),
      ],
      child: const AlexandriaApp(),
    ),
  );
}

class AlexandriaApp extends StatelessWidget {
  const AlexandriaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => AlexandriaAppState(),
      child: MaterialApp(
        title: 'Namer App',
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.blueAccent),
        ),
        home: AlexandriaHomePage(),
      ),
    );
  }
}

class AlexandriaAppState extends ChangeNotifier {}

class AlexandriaHomePage extends StatefulWidget {
  @override
  State<AlexandriaHomePage> createState() => _AlexandriaHomePageState();
}

class _AlexandriaHomePageState extends State<AlexandriaHomePage> {
  var selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    Widget page;
    switch (selectedIndex) {
      case 0:
        page = HomePage();
      case 1:
        var collectionsViewModel = CollectionsScreenNotifier(
          collectionsRepository: context.read(),
        );
        page = CollectionsScreen(screenNotifier: collectionsViewModel);
        collectionsViewModel.loadCollections.execute();
      default:
        throw UnimplementedError('no widget for $selectedIndex');
    }

    return LayoutBuilder(
      builder: (context, constraints) {
        return Scaffold(
          body:
              (BoxConstraints constraints, BuildContext context, Widget page) {
                return Row(
                  children: [
                    SafeArea(
                      child: NavigationRail(
                        extended: constraints.maxWidth >= 600,
                        destinations: [
                          NavigationRailDestination(
                            icon: Icon(Icons.home),
                            label: Text('Home'),
                          ),
                          NavigationRailDestination(
                            icon: Icon(Icons.folder),
                            label: Text('Collections'),
                          ),
                        ],
                        selectedIndex: selectedIndex,
                        onDestinationSelected: (value) {
                          setState(() {
                            selectedIndex = value;
                          });
                        },
                      ),
                    ),
                    Expanded(
                      child: Container(
                        color: Theme.of(context).colorScheme.primaryContainer,
                        child: page,
                      ),
                    ),
                  ],
                );
              }(constraints, context, page),
        );
      },
    );
  }
}
