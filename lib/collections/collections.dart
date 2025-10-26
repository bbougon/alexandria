import '../common/result.dart';

class Collection {
  final String _name;

  Collection(this._name);

  String get name => _name;
}

abstract class CollectionsRepository  {
  Future<Result<List<Collection>>> all();
}
