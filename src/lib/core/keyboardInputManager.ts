import { get } from "svelte/store";
import {
  appConfig,
  isRemapping,
  isZoomModifierDownActive,
  isZoomModifierUpActive,
} from "$lib/stores/appState";
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
const pressedKeys = new Set<string>();

/**
 * Normalizes a key based on the current platform.
 * On macOS, the "Meta" key is normalized to "Alt" to match the standard
 * shortcut naming convention.
 * @param {string} key - The key to be normalized.
 * @returns {string} - The normalized key.
 */
const platformNormalizedKey = (key: string): string => {
  const isMac = navigator.platform.toUpperCase().indexOf("MAC") >= 0;
  if (isMac && key === "Meta") {
    return "Alt";
  }
  return key;
};

/**
 * Updates the zoom modifier active states based on the currently pressed keys.
 * This function is called whenever a key is pressed or released.
 * It checks if any of the keys defined in the zoom modifier up or down shortcuts
 * are currently pressed, and updates the corresponding active states accordingly.
 */
const updateZoomModifiers = () => {
  const { zoomModifierUp, zoomModifierDown } = get(appConfig).shortcuts;

  const modUpPressed = zoomModifierUp.keys.some((key) => pressedKeys.has(key));
  const modDownPressed = zoomModifierDown.keys.some((key) => pressedKeys.has(key));

  const upActive = modUpPressed && !modDownPressed;
  const downActive = modDownPressed && !modUpPressed;

  isZoomModifierUpActive.set(upActive);
  isZoomModifierDownActive.set(downActive);
};

/**
 * Handles a keydown event by checking if the key matches any of the shortcuts.
 * If a match is found, the corresponding action is triggered and the event is
 * prevented from propagating further.
 * If the key matches the active continuous key (i.e. the key that was last
 * pressed to start a continuous action), the function does nothing and returns.
 * If the event is a repeat of a previously pressed key, the function does
 * nothing and returns.
 *
 * @param {KeyboardEvent} event - The keydown event to be handled.
 */
export const handleKeyDown = (event: KeyboardEvent) => {
  if (get(isRemapping)) return;

  const key = platformNormalizedKey(event.key);
  pressedKeys.add(key);
  updateZoomModifiers();

  if (event.repeat || activeContinuousKey) return;

  const shortcuts = get(appConfig).shortcuts;
  for (const actionName in shortcuts) {
    const shortcut = shortcuts[actionName as keyof typeof shortcuts];
    if (shortcut.keys.includes(key)) {
      if (actionName === "zoomIn") {
        startZoomIn();
        activeContinuousKey = key;
      } else if (actionName === "zoomOut") {
        startZoomOut();
        activeContinuousKey = key;
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

  const key = platformNormalizedKey(event.key);
  pressedKeys.delete(key);
  updateZoomModifiers();

  if (key === activeContinuousKey) {
    stopContinuousZoom();
    activeContinuousKey = null;
    event.preventDefault();
  }
};
