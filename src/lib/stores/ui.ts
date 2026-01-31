import type { ImageTransform } from "$lib/types/viewport";
import { createTooltipStore } from "$lib/utils/createTooltipStore";
import { writable } from "svelte/store";

export const isInfoSidebarVisible = writable(false);
export const isOptionsMenuVisible = writable(false);
export const isLanguageMenuVisible = writable(false);
export const isThemeMenuVisible = writable(false);
export const isHotkeysMenuVisible = writable(false);
export const isToolbarMenuVisible = writable(false);
export const isImageNameDisplayMenuVisible = writable(false);
export const isEdgeIndicatorMenuVisible = writable(false);
export const isAppWindowMenuVisible = writable(false);
export const isSaveAsMenuVisible = writable(false);
export const isAboutMenuVisible = writable(false);
export const isAsciiCharsMenuVisible = writable(false);
export const isGridOverlayMenuVisible = writable(false);
export const isGridOverlayVisible = writable(false);
export const isZenModeActive = writable(false);

export const isFullscreenActive = writable(false);
export const isRefittingOnResize = writable(false);

export const edgeIndicators = writable({
  top: false,
  bottom: false,
  left: false,
  right: false,
});

export const indicatorsVisible = writable(false);
export const zoomLevel = writable(1);
export const isZoomModifierUpActive = writable(false);
export const isZoomModifierDownActive = writable(false);
export const tooltip = createTooltipStore();

export const notification = writable<{ message: string; visible: boolean }>({
  message: "",
  visible: false,
});

export const imageTransform = writable<ImageTransform>({
  offsetX: 0,
  offsetY: 0,
  scale: 1,
  rotation: 0,
  width: 0,
  height: 0,
  naturalWidth: 0,
  naturalHeight: 0,
  renderedWidth: 0,
  renderedHeight: 0,
  baseScale: 1,
});
