<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  interface Props extends HTMLInputAttributes {
    value: string;
    remove: () => void;
  }

  type ColourName = 'purple' | 'yellow' | 'green' | 'blue' | 'indigo' | 'pink';
  type ColourClasses = { span: string[]; svg: string[]; button: string[] };
  type ColourEntry = Record<ColourName, ColourClasses>;

  const colours: ColourEntry = {
    purple: {
      span: [
        'bg-purple-100',
        'text-purple-700',
        'dark:bg-purple-400/10',
        'dark:text-purple-400',
      ],
      svg: [
        'stroke-violet-700/50',
        'group-hover:stroke-violet-700/75',
        'dark:stroke-violet-400',
        'dark:group-hover:stroke-violet-300',
      ],
      button: ['bg-purple-600/20', 'dark:bg-purple-400/20'],
    },
    yellow: {
      span: [
        'bg-yellow-100',
        'text-yellow-800',
        'dark:bg-yellow-400/10',
        'dark:text-yellow-500',
      ],
      svg: [
        'stroke-yellow-800/50',
        'group-hover:stroke-yellow-800/75',
        'dark:stroke-yellow-400',
        'dark:group-hover:stroke-yellow-300',
      ],
      button: ['bg-yellow-600/20', 'dark:bg-yellow-400/20'],
    },
    green: {
      span: [
        'bg-green-100',
        'text-green-700',
        'dark:bg-green-400/10',
        'dark:text-green-400',
      ],
      svg: [
        'stroke-green-800/50',
        'group-hover:stroke-green-800/75',
        'dark:stroke-green-400',
        'dark:group-hover:stroke-green-300',
      ],
      button: ['bg-green-600/20', 'dark:bg-green-400/20'],
    },
    blue: {
      span: [
        'bg-blue-100',
        'text-blue-700',
        'dark:bg-blue-400/10',
        'dark:text-blue-400',
      ],
      svg: [
        'stroke-blue-800/50',
        'group-hover:stroke-blue-800/75',
        'dark:stroke-blue-400',
        'dark:group-hover:stroke-blue-300',
      ],
      button: ['bg-blue-600/20', 'dark:bg-blue-400/20'],
    },
    indigo: {
      span: [
        'bg-indigo-100',
        'text-indigo-700',
        'dark:bg-indigo-400/10',
        'dark:text-indigo-400',
      ],
      svg: [
        'stroke-indigo-700/50',
        'group-hover:stroke-indigo-700/75',
        'dark:stroke-indigo-400',
        'dark:group-hover:stroke-indigo-300',
      ],
      button: ['bg-indigo-600/20', 'dark:bg-indigo-400/20'],
    },
    pink: {
      span: [
        'bg-pink-100',
        'text-pink-700',
        'dark:bg-pink-400/10',
        'dark:text-pink-400',
      ],
      svg: [
        'stroke-pink-800/50',
        'group-hover:stroke-pink-800/75',
        'dark:stroke-pink-400',
        'dark:group-hover:stroke-pink-300',
      ],
      button: ['bg-pink-600/20', 'dark:bg-pink-400/20'],
    },
  };

  const pickRandomColour = (colours: ColourEntry) => {
    const allColours = Object.values(colours);
    return allColours[Math.floor(Math.random() * allColours.length)];
  };

  let { value, remove }: Props = $props();
  const activeColour = pickRandomColour(colours);
</script>

<span
  class={`${activeColour.span.join(' ')} inline-flex items-center gap-x-0.5 rounded-md  px-2 py-1 text-xs font-medium`}
>
  {value}
  <button
    type="button"
    class={`${activeColour.button.join(' ')} group relative -mr-1 size-3.5 rounded-xs`}
    onclick={() => remove()}
  >
    <span class="sr-only">Remove</span>
    <svg viewBox="0 0 14 14" class={`${activeColour.svg.join(' ')} size-3.5`}>
      <path d="M4 4l6 6m0-6l-6 6" />
    </svg>
    <span class="absolute -inset-1"></span>
  </button>
</span>
