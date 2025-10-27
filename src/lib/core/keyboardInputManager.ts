import { get } from "svelte/store";
import { appConfig, isRemapping } from "$lib/stores/appState";
import {
  openFile,
  previousImage,
  nextImage,
  toggleExif,
  toggleOptions,
  toggleFullscreen,
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
  toggleFullscreen,
};

let activeContinuousKey: string | null = null;

/**
 * Handles a keydown event by checking if the key matches any of the shortcuts
 * defined in the app configuration. If a match is found, the corresponding action is
 * performed. If the action is a continuous zoom action, the action is performed
 * repeatedly until the key is released.
 *
 * @param {KeyboardEvent} event - The keydown event to be handled.
 */
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

/**
 * Handles a keyup event by checking if the key matches the active continuous key.
 * If a match is found, the continuous zoom action is stopped and the active
 * continuous key is reset to null.
 *
 * @param {KeyboardEvent} event - The keyup event to be handled.
 */
export const handleKeyUp = (event: KeyboardEvent) => {
  if (get(isRemapping)) return;
  if (event.key === activeContinuousKey) {
    stopContinuousZoom();
    activeContinuousKey = null;
    event.preventDefault();
  }
};
