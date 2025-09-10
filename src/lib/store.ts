import { writable } from 'svelte/store';

export const imageUrl = writable<string | null>(null);
