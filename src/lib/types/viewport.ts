import type { Writable } from "svelte/store";

export type ViewportOptions = {
  zoomLevelStore: Writable<number>;
  imageUrlStore: Writable<string | null>;
  onImageDrawn?: () => void;
};
