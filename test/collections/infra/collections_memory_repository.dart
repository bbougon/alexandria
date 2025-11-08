import 'package:alexandria/collections/collections.dart';
import 'package:alexandria/common/result.dart';

class CollectionsMemoryRepository implements CollectionsRepository {
  List<Collection> _collections = [];

  @override
  Future<Result<void>> add(Collection collection) async {
    _collections.add(collection);
    return Result.ok(null);
  }

  @override
  Future<Result<List<Collection>>> all() async {
    return Result.ok(_collections);
  }

  @override
  Future<Result<Collection>> getCollection(String collectionId) async {
    try {
      Collection foundCollection = _collections.singleWhere(
        (collection) => collection.id == collectionId,
      );
      return Result.ok(foundCollection);
    } on Exception catch (exception) {
      return Result.error(exception);
    }
  }
}
