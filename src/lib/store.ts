import { writable } from "svelte/store";

export const imageUrl = writable<string | null>(null);
export const imagePath = writable<string | null>(null);
export const imageExif = writable<string | null>(null);
export const zoomLevel = writable<number>(1);
export const edgeIndicators = writable({
  top: false,
  bottom: false,
  left: false,
  right: false,
});
export const indicatorsVisible = writable(false);
export const isSidebarVisible = writable<boolean>(true);