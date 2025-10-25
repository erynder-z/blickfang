import { get } from "svelte/store";
import { appConfig, isRemapping } from "$lib/stores/appState";
import {
  openFile,
  previousImage,
  nextImage,
  toggleExif,
  toggleOptions,
  startZoomIn,
  startZoomOut,
  stopContinuousZoom,
} from "./commands";

const singleShotActions: Record<string, () => void> = {
  openFile,
  previousImage,
  nextImage,
  toggleExif,
  toggleOptions,
};

let activeContinuousKey: string | null = null;

export const handleKeyDown = (event: KeyboardEvent) => {
  if (get(isRemapping)) return;
  if (event.repeat || activeContinuousKey) return;

  const shortcuts = get(appConfig).shortcuts;
  for (const actionName in shortcuts) {
    const shortcut = shortcuts[actionName as keyof typeof shortcuts];
    if (shortcut.keys.includes(event.key)) {
      if (actionName === "zoomIn") {
        startZoomIn();
        activeContinuousKey = event.key;
      } else if (actionName === "zoomOut") {
        startZoomOut();
        activeContinuousKey = event.key;
      } else {
        const action = singleShotActions[actionName];
        if (action) action();
      }
      event.preventDefault();
      return;
    }
  }
};

export const handleKeyUp = (event: KeyboardEvent) => {
  if (get(isRemapping)) return;
  if (event.key === activeContinuousKey) {
    stopContinuousZoom();
    activeContinuousKey = null;
    event.preventDefault();
  }
};
