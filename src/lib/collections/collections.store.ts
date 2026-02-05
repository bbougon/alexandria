import { writable } from 'svelte/store';
import type { Collection } from './collection';

const { subscribe, set } = writable<Collection[]>();

export const collectionsStore = { subscribe, initialize: set };
