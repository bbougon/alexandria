<script lang="ts">
  import type { Snippet } from 'svelte';

  interface DrawerProps {
    open: boolean;
    close: () => void;
    children: Snippet;
  }

  let { close, open, children }: DrawerProps = $props();
  let drawerEl: HTMLDialogElement | null = null;

  $effect(() => {
    if (drawerEl) {
      if (open && !drawerEl.open) drawerEl.showModal();
      if (!open && drawerEl.open) drawerEl.close();
    }
  });

  const requestClose = () => {
    open = false;
    close();
  };
</script>

<el-dialog>
  <dialog
    bind:this={drawerEl}
    id="drawer"
    class="fixed inset-0 size-auto max-h-none max-w-none overflow-hidden bg-transparent not-open:hidden backdrop:bg-transparent"
    onclose={requestClose}
  >
    <el-dialog-backdrop
      class="absolute inset-0 bg-gray-500/75 transition-opacity duration-500 ease-in-out data-closed:opacity-0 dark:bg-gray-900/50"
      onclick={requestClose}
    ></el-dialog-backdrop>

    <div class="absolute inset-0 pl-10 focus:outline-none sm:pl-16">
      <el-dialog-panel
        class="group/dialog-panel relative ml-auto block size-full sm:max-w-sm lg:max-w-2xl transform transition duration-500 ease-in-out data-closed:translate-x-full sm:duration-700"
      >
        <div
          class="absolute top-0 left-0 -ml-8 flex pt-4 pr-2 duration-500 ease-in-out group-data-closed/dialog-panel:opacity-0 sm:-ml-10 sm:pr-4"
        >
          <button
            type="button"
            class="relative rounded-md text-gray-300 hover:text-white focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:text-gray-400 dark:hover:text-white dark:focus-visible:outline-indigo-500"
            onclick={requestClose}
          >
            <span class="absolute -inset-2.5"></span>
            <span class="sr-only">Close panel</span>
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              data-slot="icon"
              aria-hidden="true"
              class="size-6"
            >
              <path
                d="M6 18 18 6M6 6l12 12"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
        </div>

        <!-- Slide-over panel, show/hide based on slide-over state. -->
        <div
          class="relative h-full overflow-y-auto bg-white p-8 dark:bg-gray-800 dark:after:absolute dark:after:inset-y-0 dark:after:left-0 dark:after:w-px dark:after:bg-white/10"
        >
          <div class="space-y-6 pb-16">
            {@render children?.()}
          </div>
        </div>
      </el-dialog-panel>
    </div>
  </dialog>
</el-dialog>
