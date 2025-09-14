import { writable } from "svelte/store";

export const imageUrl = writable<string | null>(null);
export const imagePath = writable<string | null>(null);
export const zoomLevel = writable<number>(1);
