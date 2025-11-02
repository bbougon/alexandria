import '../../common/result.dart';
import '../collections.dart';

class CollectionsFileRepository implements CollectionsRepository {
  final List<Collection> _collections = [];
  @override
  Future<Result<List<Collection>>> all() async {
    return Result.ok(_collections);
  }

  @override
  Future<Result<void>> add(Collection collection) async {
    _collections.add(collection);
    return Result.ok(null);
  }
}
