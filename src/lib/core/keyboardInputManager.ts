import { get } from "svelte/store";
import {
  appConfig,
  imageUrl,
  isRemapping,
  isZoomModifierDownActive,
  isZoomModifierUpActive,
} from "$lib/stores";

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
  toggleSaveAsMenu,
} from "./commands";

// Actions that fire only once (triggered by a single key press)
const singleShotActions: Record<string, () => void> = {
  openFile,
  previousImage,
  nextImage,
  toggleExif,
  toggleOptions,
  toggleFullscreen,
  saveImageAs: toggleSaveAsMenu,
};

// Actions that are continuous (toggled by holding a key)
const continuousActions: Record<string, () => void> = {
  zoomIn: startZoomIn,
  zoomOut: startZoomOut,
};

const imageDependentActions = new Set([
  "previousImage",
  "nextImage",
  "toggleExif",
  "zoomIn",
  "zoomOut",
  "saveImageAs",
]);

let activeContinuousKey: string | null = null;
const pressedKeys = new Set<string>();

/**
 * Normalize a key name based on the platform.
 * On macOS, the "Meta" key is equivalent to the "Alt" key.
 * @param {string} key - The key name to normalize.
 * @returns {string}
 */
const platformNormalizedKey = (key: string): string => {
  const isMac = navigator.platform.toUpperCase().includes("MAC");
  return isMac && key === "Meta" ? "Alt" : key;
};

/**
 * Updates the zoom modifier state based on the currently pressed keys.
 * The zoom modifier state is used to determine whether the zoom in or zoom out
 * action should be performed when the user presses a zoom key.
 */
const updateZoomModifiers = () => {
  const { zoomModifierUp, zoomModifierDown } = get(appConfig).shortcuts;

  const upPressed = zoomModifierUp.keys.some((k) => pressedKeys.has(k));
  const downPressed = zoomModifierDown.keys.some((k) => pressedKeys.has(k));

  isZoomModifierUpActive.set(upPressed && !downPressed);
  isZoomModifierDownActive.set(downPressed && !upPressed);
};

/**
 * Handles a keydown event. If the key is part of a shortcut, it will execute
 * the corresponding action. If the key is part of a continuous action (zoom), it
 * will start the action. If the key is part of a single-shot action (e.g. open
 * file), it will execute the action once.
 * @param {KeyboardEvent} event - The keydown event to handle.
 */
export const handleKeyDown = (event: KeyboardEvent) => {
  if (get(isRemapping)) return;

  const key = platformNormalizedKey(event.key);
  pressedKeys.add(key);
  updateZoomModifiers();

  if (event.repeat || activeContinuousKey) return;

  const shortcuts = get(appConfig).shortcuts;
  const hasImage = !!get(imageUrl);

  for (const actionName in shortcuts) {
    const shortcut = shortcuts[actionName as keyof typeof shortcuts];
    if (!shortcut.keys.includes(key)) continue;

    if (!hasImage && imageDependentActions.has(actionName)) continue;

    // Continuous actions (zoom)
    if (actionName in continuousActions) {
      continuousActions[actionName]();
      activeContinuousKey = key;
      event.preventDefault();
      return;
    }

    // Single-shot actions
    const action = singleShotActions[actionName];
    if (action) {
      action();
      event.preventDefault();
    }
    return;
  }
};

/**
 * Handles a keyup event. If the key is part of a continuous action (zoom), it
 * will stop the action. Also updates the zoom modifier state.
 * @param {KeyboardEvent} event - The keyup event to handle.
 */
export const handleKeyUp = (event: KeyboardEvent) => {
  if (get(isRemapping)) return;

  const key = platformNormalizedKey(event.key);
  pressedKeys.delete(key);
  updateZoomModifiers();

  if (key === activeContinuousKey) {
    stopContinuousZoom();
    activeContinuousKey = null;
    event.preventDefault();
  }
};
