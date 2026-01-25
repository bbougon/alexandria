<script lang="ts">
  import CollectionForm from './lib/collections/CollectionForm.svelte';
  import Searchbar from './lib/kit/layout/Searchbar.svelte';
  import Sidebar from './lib/kit/layout/Sidebar.svelte';
</script>

<Sidebar />

<div class="lg:pl-72">
  <Searchbar />
  <main class="py-10">
    <CollectionForm />
  </main>
</div>

<style>
</style>
