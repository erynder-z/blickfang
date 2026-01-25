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
  rotateCounterclockwise,
  rotateClockwise,
  convertToAsciiArt,
  toggleGridOverlay,
  toggleZenMode,
} from "$lib/core/commands";

export class KeyboardInputManager {
  // Actions that fire only once (triggered by a single key press)
  private singleShotActions: Record<string, () => void> = {
    openFile,
    previousImage,
    nextImage,
    rotateClockwise,
    rotateCounterclockwise,
    toggleExif,
    toggleOptions,
    toggleFullscreen,
    saveImageAs: toggleSaveAsMenu,
    convertToAsciiArt,
    toggleGridOverlay,
    toggleZenMode,
  };

  // Actions that are continuous (toggled by holding a key)
  private continuousActions: Record<string, () => void> = {
    zoomIn: startZoomIn,
    zoomOut: startZoomOut,
  };

  private imageDependentActions = new Set([
    "previousImage",
    "nextImage",
    "toggleExif",
    "zoomIn",
    "zoomOut",
    "rotateClockwise",
    "rotateCounterclockwise",
    "saveImageAs",
    "convertToAsciiArt",
    "toggleGridOverlay",
    "toggleZenMode",
  ]);

  private activeContinuousKey: string | null = null;
  private pressedKeys = new Set<string>();

  constructor() {
    this.handleKeyDown = this.handleKeyDown.bind(this);
    this.handleKeyUp = this.handleKeyUp.bind(this);
  }

  /**
   * Normalizes a key to account for differences in key names between Windows and
   * macOS. For example, the "Meta" key on a Mac is equivalent to the "Alt" key on
   * Windows.
   * @param {string} key - The key to normalize.
   * @returns {string} - The normalized key.
   */
  private platformNormalizedKey(key: string): string {
    const isMac = /Mac/i.test(navigator.userAgent);
    return isMac && key === "Meta" ? "Alt" : key;
  }

  /**
   * Updates the zoom modifier state based on the currently pressed keys.
   * The zoom modifier state is used to determine whether the zoom in or zoom out
   * action should be performed when the user presses a zoom key.
   */
  private updateZoomModifiers(): void {
    const { zoomModifierUp, zoomModifierDown } = get(appConfig).shortcuts;

    const upPressed = zoomModifierUp.keys.some((k) => this.pressedKeys.has(k));
    const downPressed = zoomModifierDown.keys.some((k) => this.pressedKeys.has(k));

    isZoomModifierUpActive.set(upPressed && !downPressed);
    isZoomModifierDownActive.set(downPressed && !upPressed);
  }

  /**
   * Handles a keydown event. If the key is part of a shortcut, it will execute
   * the corresponding action. If the key is part of a continuous action (zoom), it
   * will start the action. If the key is part of a single-shot action (e.g. open
   * file), it will execute the action once.
   * @param {KeyboardEvent} event - The keydown event to handle.
   */
  public handleKeyDown(event: KeyboardEvent): void {
    if (get(isRemapping)) return;

    const key = this.platformNormalizedKey(event.key);
    this.pressedKeys.add(key);
    this.updateZoomModifiers();

    if (event.repeat || this.activeContinuousKey) return;

    const shortcuts = get(appConfig).shortcuts;
    const hasImage = !!get(imageUrl);

    for (const actionName in shortcuts) {
      const shortcut = shortcuts[actionName as keyof typeof shortcuts];
      if (!shortcut.keys.includes(key)) continue;

      if (!hasImage && this.imageDependentActions.has(actionName)) continue;

      // Continuous actions (zoom)
      if (actionName in this.continuousActions) {
        this.continuousActions[actionName]();
        this.activeContinuousKey = key;
        event.preventDefault();
        return;
      }

      // Single-shot actions
      const action = this.singleShotActions[actionName];
      if (action) {
        try {
          const result = action();
          const promiseResult = result as Promise<void> | void;

          if (promiseResult && typeof promiseResult.then === "function") {
            promiseResult.catch(console.error);
          }
        } catch (error) {
          console.error("Error executing action:", error);
        }
        event.preventDefault();
      }
      return;
    }
  }

  /**
   * Handles a keyup event. If the key is part of a continuous action (zoom), it
   * will stop the action. Also updates the zoom modifier state.
   * @param {KeyboardEvent} event - The keyup event to handle.
   */
  public handleKeyUp(event: KeyboardEvent): void {
    if (get(isRemapping)) return;

    const key = this.platformNormalizedKey(event.key);
    this.pressedKeys.delete(key);
    this.updateZoomModifiers();

    if (key === this.activeContinuousKey) {
      stopContinuousZoom();
      this.activeContinuousKey = null;
      event.preventDefault();
    }
  }
}

export const keyboardInputManager = new KeyboardInputManager();
