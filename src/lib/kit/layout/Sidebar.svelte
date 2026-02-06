<script>
  import DesktopSidebar from './DesktopSidebar.svelte';
  import MobileSidebar from './MobileSidebar.svelte';
  import { collectionsStore } from '../../collections/collections.store';
</script>

{#snippet showCollections()}
  {#each $collectionsStore as collection}
    <li>
      <button
        class="block cursor-pointer rounded-md py-2 pr-2 pl-9 text-sm/6 text-gray-700 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-white/5"
        >{collection.title}</button
      >
    </li>
  {/each}
{/snippet}

{#snippet showLogo()}
  <img
    src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600"
    alt="Your Company"
    class="h-8 w-auto dark:hidden"
  />
  <img
    src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=500"
    alt="Your Company"
    class="h-8 w-auto not-dark:hidden"
  />
{/snippet}

{#snippet showNav()}
  <ul role="list" class="flex flex-1 flex-col gap-y-7">
    <li>
      <ul role="list" class="-mx-2 space-y-1">
        <li>
          <!-- Current: "bg-gray-50 dark:bg-white/5 text-indigo-600 dark:text-white", Default: "text-gray-700 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-white/5" -->
          <a
            href="/"
            class="group flex gap-x-3 rounded-md bg-gray-50 p-2 text-sm/6 font-semibold text-indigo-600 dark:bg-white/5 dark:text-white"
          >
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              data-slot="icon"
              aria-hidden="true"
              class="size-6 shrink-0 text-indigo-600 dark:text-white"
            >
              <path
                d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
            Dashboard
          </a>
        </li>
      </ul>
    </li>
    <li>
      <button
        type="button"
        command="--toggle"
        commandfor="sub-menu-1"
        class="flex w-full items-center gap-x-3 rounded-md p-2 text-left text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-white/5"
      >
        <svg
          viewBox="0 0 20 20"
          fill="currentColor"
          data-slot="icon"
          aria-hidden="true"
          class="size-5 shrink-0 not-in-aria-expanded:text-gray-400 in-aria-expanded:rotate-90 in-aria-expanded:text-gray-500 dark:in-aria-expanded:text-gray-400"
        >
          <path
            d="M8.22 5.22a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 0 1 0 1.06l-4.25 4.25a.75.75 0 0 1-1.06-1.06L11.94 10 8.22 6.28a.75.75 0 0 1 0-1.06Z"
            clip-rule="evenodd"
            fill-rule="evenodd"
          />
        </svg>
        Collections
      </button>
      <el-disclosure id="sub-menu-1" hidden class="contents">
        <ul class="mt-1 px-2">
          {@render showCollections()}
        </ul>
      </el-disclosure>
    </li>
    <li class="mt-auto">
      <a
        href="/"
        class="group -mx-2 flex gap-x-3 rounded-md p-2 text-sm/6 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-300 dark:hover:bg-white/5 dark:hover:text-white"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          data-slot="icon"
          aria-hidden="true"
          class="size-6 shrink-0 text-gray-400 group-hover:text-indigo-600 dark:group-hover:text-white"
        >
          <path
            d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
        Settings
      </a>
    </li>
  </ul>
{/snippet}

<DesktopSidebar logo={showLogo} nav={showNav} />
<MobileSidebar logo={showLogo} nav={showNav} />
