import '../../common/result.dart';
import '../collections.dart';

class CollectionsFileRepository implements CollectionsRepository {
  @override
  Future<Result<List<Collection>>> all() async {
    return Result.ok([Collection('Video 1'), Collection('Video 2')]);
  }
}