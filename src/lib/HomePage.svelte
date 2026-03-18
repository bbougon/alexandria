<script lang="ts">
  import Card from './components/Card.svelte';
  import { Folder, Video } from '@lucide/svelte';
  import { collectionsStore } from './collections/collections.store';
</script>

<div>
  <div class="mb-8">
    <h1 class="text-3xl font-semibold mb-2">Video Library</h1>
    <p class="text-muted-foreground">Manage your video collections and metadata</p>
  </div>

  <div class="grid grid-cols-2 gap-4 mb-8">
    <Card class="p-6 bg-gradient-to-br from-blue-50 to-indigo-50">
      <div class="flex items-center gap-4">
        <div class="p-3 bg-blue-500 rounded-lg">
          <Folder class="w-6 h-6 text-white" />
        </div>
        <div>
          <p class="text-2xl font-semibold">{$collectionsStore.length}</p>
          <p class="text-sm text-muted-foreground">Collections</p>
        </div>
      </div>
    </Card>

    <Card class="p-6 bg-gradient-to-br from-purple-50 to-pink-50">
      <div class="flex items-center gap-4">
        <div class="p-3 bg-purple-500 rounded-lg">
          <Video class="w-6 h-6 text-white" />
        </div>
        <div>
          <p class="text-2xl font-semibold">10</p>
          <p class="text-sm text-muted-foreground">Total Videos</p>
        </div>
      </div>
    </Card>
  </div>

  <!--{collections.length === 0 ? (-->
  <!--  <Card class="p-12 text-center bg-white">-->
  <!--    <Folder class="w-16 h-16 mx-auto mb-4 text-gray-300" />-->
  <!--    <h3 class="text-xl font-medium mb-2">No collections yet</h3>-->
  <!--    <p class="text-muted-foreground mb-6">-->
  <!--      Create your first collection to start organizing your videos-->
  <!--    </p>-->
  <!--    <Link to="/new">-->
  <!--      <Button size="lg">-->
  <!--        <Plus class="w-4 h-4 mr-2" />-->
  <!--        Create Collection-->
  <!--      </Button>-->
  <!--    </Link>-->
  <!--  </Card>-->
  <!--) : (-->
  <!--  <div>-->
  <!--    <h2 class="text-xl font-semibold mb-4">Recent Collections</h2>-->
  <!--    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">-->
  <!--      {collections.map(collection => (-->
  <!--        <Link key={collection.id} to={`/collection/${collection.id}`}>-->
  <!--          <Card class="p-6 bg-white hover:shadow-lg transition-all hover:scale-[1.02]">-->
  <!--            <div class="flex items-start gap-4">-->
  <!--              <div class="p-3 bg-blue-100 rounded-lg">-->
  <!--                <Folder class="w-6 h-6 text-blue-600" />-->
  <!--              </div>-->
  <!--              <div class="flex-1 min-w-0">-->
  <!--                <h3 class="font-semibold truncate mb-1">-->
  <!--                  {collection.name}-->
  <!--                </h3>-->
  <!--                <p class="text-sm text-muted-foreground">-->
  <!--                  {collection.videos.length} video{collection.videos.length !== 1 ? 's' : ''}-->
  <!--                </p>-->
  <!--                <p class="text-xs text-muted-foreground mt-2">-->
  <!--                  {new Date(collection.createdAt).toLocaleDateString()}-->
  <!--                </p>-->
  <!--              </div>-->
  <!--            </div>-->
  <!--          </Card>-->
  <!--        </Link>-->
  <!--      ))}-->
  <!--    </div>-->
  <!--  </div>-->
  <!--)}-->
</div>
