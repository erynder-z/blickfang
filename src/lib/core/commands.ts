import { get } from "svelte/store";
import {
  imagePath,
  imageUrl,
  zoomLevel,
  isExifSidebarVisible,
  isOptionsMenuVisible,
  activeActions,
} from "$lib/stores/appState";
import { invoke } from "@tauri-apps/api/core";
import { processImage } from "../utils/imageProcessor";

/**
 * Starts feedback for an action, marking it as active in the app state.
 * The feedback will be stopped when stopFeedback is called with the same action name.
 * @param {string} actionName - the name of the action to start feedback for
 */
const startFeedback = (actionName: string) => {
  activeActions.update((actions) => [...new Set([...actions, actionName])]);
};

/**
 * Stops feedback for an action, marking it as inactive in the app state.
 * This should be called after an action has finished, to prevent the feedback from
 * being displayed indefinitely.
 * @param {string} actionName - the name of the action to stop feedback for
 */
const stopFeedback = (actionName: string) => {
  activeActions.update((actions) => actions.filter((a) => a !== actionName));
};

/**
 * Starts and stops feedback for an action, after a short delay.
 * This can be used to display a brief loading indicator or other feedback
 * when an action is started, and then hide it when the action is finished.
 * Intended for actions that are completed in a short amount of time.
 * @param {string} actionName - the name of the action to start and stop feedback for
 */
const singleShotFeedback = (actionName: string) => {
  startFeedback(actionName);
  setTimeout(() => stopFeedback(actionName), 150);
};

// --- Actions ---

/**
 * Opens a file dialog to select an image file, reads the file and sets the image data,
 * path, and EXIF data in the app state.
 * The zoom level is also reset to 1.
 * If the file cannot be opened or read, an error is logged to the console.
 * A loading indicator is shown while the file is being opened and read.
 * @returns {Promise<void>}
 */
export const openFile = async (): Promise<void> => {
  startFeedback("openFile");
  try {
    const [newImageData, newImageExif, newImagePath, _] =
      await invoke<[string, string, string, string[]]>("open_and_read_file");
    if (newImageData) {
      imageUrl.set(newImageData);
      imagePath.set(newImagePath);
      processImage(newImageData, newImagePath, newImageExif);
      zoomLevel.set(1);
    }
  } catch (error) {
    console.error("Failed to open and read file:", error);
  } finally {
    stopFeedback("openFile");
  }
};

/**
 * Changes the image to the previous or next one in the directory.
 * Reads the new image file and sets the image data, path, and EXIF data in the app state.
 * The zoom level is also reset to 1.
 * If the file cannot be opened or read, an error is logged to the console.
 * A loading indicator is shown while the file is being opened and read.
 * @param {string} direction - the direction to change the image, either "next" or "previous"
 * @returns {Promise<void>}
 */
const changeImage = async (direction: "next" | "previous"): Promise<void> => {
  const currentPath = get(imagePath);
  if (!currentPath) return;

  try {
    const [newImageData, newImagePath, newImageExif] = await invoke<[string, string, string]>(
      "change_image",
      {
        currentPath,
        direction,
      }
    );
    imageUrl.set(newImageData);
    imagePath.set(newImagePath);
    processImage(newImageData, newImagePath, newImageExif);
    zoomLevel.set(1);
  } catch (error) {
    console.error("Failed to change image:", error);
  }
};

/**
 * Handles all associated actions for changing the image to the previous one in the directory.
 */
export const previousImage = () => {
  singleShotFeedback("previousImage");
  changeImage("previous");
};

/**
 * Handles all associated actions for changing the image to the next one in the directory.
 */
export const nextImage = () => {
  singleShotFeedback("nextImage");
  changeImage("next");
};

// --- Zoom Actions ---

/**
 * Handles all associated actions for zooming in the image.
 * Zooms in the image by increasing the zoom level by 0.25.
 */
export const zoomIn = () => {
  singleShotFeedback("zoomIn");
  zoomLevel.update((level) => level + 0.25);
};

/**
 * Handles all associated actions for zooming out of the image.
 * Zooms out of the image by decreasing the zoom level by 0.25.
 */
export const zoomOut = () => {
  singleShotFeedback("zoomOut");
  zoomLevel.update((level) => {
    const newLevel = level - 0.25;
    return newLevel < 0.1 ? 0.1 : newLevel;
  });
};

// For continuous zoom (holding button/key)
let zoomInterval: ReturnType<typeof setInterval> | null = null;

/**
 * Stops continuous zooming of the image.
 * Clears the zoom interval and stops the "zoomIn" and "zoomOut" feedback.
 */
export const stopContinuousZoom = () => {
  if (zoomInterval) {
    clearInterval(zoomInterval);
    zoomInterval = null;
  }
  stopFeedback("zoomIn");
  stopFeedback("zoomOut");
};

/**
 * Starts continuous zooming in of the image.
 * Zooms in the image by increasing the zoom level by 0.1 every 50ms.
 * Stops any existing continuous zooming and starts feedback for the "zoomIn" action.
 */
export const startZoomIn = () => {
  stopContinuousZoom();
  startFeedback("zoomIn");
  zoomLevel.update((level) => level + 0.1);
  zoomInterval = setInterval(() => {
    zoomLevel.update((level) => level + 0.1);
  }, 50);
};

/**
 * Starts continuous zooming out of the image.
 * Zooms out of the image by decreasing the zoom level by 0.1 every 50ms.
 * Stops any existing continuous zooming and starts feedback for the "zoomOut" action.
 */
export const startZoomOut = () => {
  stopContinuousZoom();
  startFeedback("zoomOut");
  zoomLevel.update((level) => {
    const newLevel = level - 0.1;
    return newLevel < 0.1 ? 0.1 : newLevel;
  });
  zoomInterval = setInterval(() => {
    zoomLevel.update((level) => {
      const newLevel = level - 0.1;
      return newLevel < 0.1 ? 0.1 : newLevel;
    });
  }, 50);
};

// For wheel zoom
let wheelZoomTimeout: ReturnType<typeof setTimeout> | null = null;

/**
 * Handles all associated actions for stopping the wheel zoom.
 * Stops the "zoomIn" and "zoomOut" feedback.
 */
const stopWheelZoom = () => {
  stopFeedback("zoomIn");
  stopFeedback("zoomOut");
};

/**
 * Triggers the wheel zoom action.
 * If the wheel zoom is already in progress, it stops the current action and starts a new one.
 * Stops the feedback for the opposite action, starts the feedback for the given action, and then stops the feedback after 150ms.
 * @param {string} direction - the direction of the wheel zoom, either "in" or "out"
 */
export const triggerWheelZoom = (direction: "in" | "out") => {
  if (wheelZoomTimeout) {
    clearTimeout(wheelZoomTimeout);
  }
  stopFeedback(direction === "in" ? "zoomOut" : "zoomIn");
  startFeedback(direction === "in" ? "zoomIn" : "zoomOut");
  wheelZoomTimeout = setTimeout(stopWheelZoom, 150);
};

// --- UI Toggle Actions ---

/**
 * Toggles the visibility of the EXIF sidebar.
 * Starts feedback for the "toggleExif" action, and then updates the isExifSidebarVisible store.
 */
export const toggleExif = () => {
  singleShotFeedback("toggleExif");
  isExifSidebarVisible.update((isOpen) => !isOpen);
};

/**
 * Toggles the visibility of the options sidebar.
 * Starts feedback for the "toggleOptions" action, and then updates the isOptionsMenuVisible store.
 */
export const toggleOptions = () => {
  singleShotFeedback("toggleOptions");
  isOptionsMenuVisible.update((isOpen) => !isOpen);
};
