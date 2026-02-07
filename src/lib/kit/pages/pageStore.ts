import { writable } from 'svelte/store';

export type Page =
  | 'CollectionCreationPage'
  | 'CollectionDetailsPage'
  | 'SearchResultPage';

const { set, subscribe, update } = writable<Page>('CollectionCreationPage');
const goTo = (page: Page) => update(() => page);

export const pageStore = {
  initialize: set,
  subscribe,
  goTo,
};
