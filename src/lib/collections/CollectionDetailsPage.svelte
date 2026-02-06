<script lang="ts">
  import { selectedCollection } from './collection.store';
  import Badge from '../components/Badge.svelte';
</script>

<div class="px-4 sm:px-6 lg:px-8">
  {#if $selectedCollection.collection}
    <div class="border-b border-gray-200 pb-5 dark:border-white/10">
      <div class="mt-2 -ml-2 flex flex-wrap items-baseline">
        <h1 class="text-6xl font-bold tracking-tight text-gray-900 sm:text-4xl">
          {$selectedCollection.collection.title}
        </h1>
        <p class="mt-1 ml-2 truncate text-sm text-gray-500 dark:text-gray-400">
          Number of videos {$selectedCollection.collection.videos.length}
        </p>
      </div>
      <div class="max-w-2xl px-4 pt-16 pb-24 sm:px-6 lg:max-w-7xl lg:px-8">
        <div
          class="mt-12 lg:grid lg:grid-cols-12 lg:items-start lg:gap-x-12 xl:gap-x-16"
        >
          <section aria-labelledby="collection-heading" class="lg:col-span-7">
            <h2 id="collection-heading" class="sr-only">Videos in collection</h2>

            <ul
              role="list"
              class="divide-y divide-gray-200 border-t border-b border-gray-200"
            >
              {#each $selectedCollection.collection.videos as video}
                <li class="flex py-6 sm:py-10">
                  <div class="shrink-0">
                    <img
                      src={video.thumbnail}
                      alt={video.name}
                      class="size-24 aspect-10/7 rounded-md object-cover sm:size-48"
                    />
                  </div>

                  <div class="ml-4 flex flex-1 flex-col justify-between sm:ml-6">
                    <div
                      class="relative pr-9 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:pr-0"
                    >
                      <div>
                        <div class="flex justify-between">
                          <h3 class="text-sm">
                            <a
                              href="#"
                              class="font-medium text-gray-700 hover:text-gray-800"
                              >{video.name}</a
                            >
                          </h3>
                        </div>
                        <p class="mt-1 text-xl font-medium text-gray-900">
                          {video.artist}
                        </p>
                        <p class="mt-1 text-lg font-medium text-gray-900">
                          {video.song}
                        </p>
                        <div class="mt-1 flex text-sm">
                          {#each video.style as style}
                            <p
                              class="style mr-4 border-r border-gray-200 pr-4 text-gray-500"
                            >
                              {style}
                            </p>
                          {/each}
                        </div>
                      </div>
                    </div>

                    <div class="flex">
                      <div class="mt-2 mb-4 flex gap-2 items-center flex-wrap">
                        {#each video.tags as tag}
                          <Badge value={tag} removable={false} />
                        {/each}
                      </div>
                    </div>
                  </div>
                </li>
              {/each}
            </ul>
          </section>
          <!--          <section aria-labelledby="summary-heading" class="mt-16 rounded-lg bg-gray-50 px-4 py-6 sm:p-6 lg:col-span-5 lg:mt-0 lg:p-8">-->
          <!--                      <h2 id="summary-heading" class="text-lg font-medium text-gray-900">Order summary</h2>-->
          <!--                      <dl class="mt-6 space-y-4">-->
          <!--                        <div class="flex items-center justify-between">-->
          <!--                          <dt class="text-sm text-gray-600">Subtotal</dt>-->
          <!--                          <dd class="text-sm font-medium text-gray-900">$99.00</dd>-->
          <!--                        </div>-->
          <!--                        <div class="flex items-center justify-between border-t border-gray-200 pt-4">-->
          <!--                          <dt class="flex items-center text-sm text-gray-600">-->
          <!--                            <span>Shipping estimate</span>-->
          <!--                            <a href="#" class="ml-2 shrink-0 text-gray-400 hover:text-gray-500">-->
          <!--                              <span class="sr-only">Learn more about how shipping is calculated</span>-->
          <!--                              <svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5">-->
          <!--                                <path d="M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0ZM8.94 6.94a.75.75 0 1 1-1.061-1.061 3 3 0 1 1 2.871 5.026v.345a.75.75 0 0 1-1.5 0v-.5c0-.72.57-1.172 1.081-1.287A1.5 1.5 0 1 0 8.94 6.94ZM10 15a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd" fill-rule="evenodd" />-->
          <!--                              </svg>-->
          <!--                            </a>-->
          <!--                          </dt>-->
          <!--                          <dd class="text-sm font-medium text-gray-900">$5.00</dd>-->
          <!--                        </div>-->
          <!--                        <div class="flex items-center justify-between border-t border-gray-200 pt-4">-->
          <!--                          <dt class="flex text-sm text-gray-600">-->
          <!--                            <span>Tax estimate</span>-->
          <!--                            <a href="#" class="ml-2 shrink-0 text-gray-400 hover:text-gray-500">-->
          <!--                              <span class="sr-only">Learn more about how tax is calculated</span>-->
          <!--                              <svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5">-->
          <!--                                <path d="M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0ZM8.94 6.94a.75.75 0 1 1-1.061-1.061 3 3 0 1 1 2.871 5.026v.345a.75.75 0 0 1-1.5 0v-.5c0-.72.57-1.172 1.081-1.287A1.5 1.5 0 1 0 8.94 6.94ZM10 15a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd" fill-rule="evenodd" />-->
          <!--                              </svg>-->
          <!--                            </a>-->
          <!--                          </dt>-->
          <!--                          <dd class="text-sm font-medium text-gray-900">$8.32</dd>-->
          <!--                        </div>-->
          <!--                        <div class="flex items-center justify-between border-t border-gray-200 pt-4">-->
          <!--                          <dt class="text-base font-medium text-gray-900">Order total</dt>-->
          <!--                          <dd class="text-base font-medium text-gray-900">$112.32</dd>-->
          <!--                        </div>-->
          <!--                      </dl>-->

          <!--                      <div class="mt-6">-->
          <!--                        <button type="submit" class="w-full rounded-md border border-transparent bg-indigo-600 px-4 py-3 text-base font-medium text-white shadow-xs hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-50 focus:outline-hidden">Checkout</button>-->
          <!--                      </div>-->
          <!--                    </section>-->
        </div>
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  .style {
    white-space: nowrap;
  }
</style>
