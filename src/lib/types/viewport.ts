import type { Writable } from "svelte/store";

export type ViewportOptions = {
  zoomLevelStore: Writable<number>;
  imageUrlStore: Writable<string | null>;
  onImageDrawn?: () => void;
};

export type ImageTransform = {
  offsetX: number;
  offsetY: number;
  scale: number;
  rotation: number;
  width: number;
  height: number;
  naturalWidth: number;
  naturalHeight: number;
  renderedWidth: number;
  renderedHeight: number;
  baseScale: number;
};
