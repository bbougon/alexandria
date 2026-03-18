import { getContext, setContext } from 'svelte';

export type PopoverState = {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
  toggle: () => void;
  id: string;
  anchor: HTMLElement | null;
};

const POPOVER_CONTEXT = Symbol('POPOVER_CONTEXT');

export function setPopoverContext(state: PopoverState) {
  setContext(POPOVER_CONTEXT, state);
}

export function getPopoverContext(): PopoverState {
  return getContext(POPOVER_CONTEXT);
}
