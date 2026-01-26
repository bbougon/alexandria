<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  interface Props<T = unknown> extends HTMLInputAttributes {
    option: T;
    checked: boolean;
  }

  let { option, checked, ...props }: Props = $props();
</script>

<label class="flex items-center px-4 py-2 hover:bg-gray-100">
  <input type="checkbox" class="mr-2" bind:checked {...props} />
  {option}
</label>
