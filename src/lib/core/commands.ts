import { get } from "svelte/store";
import type { ImageMetadata } from "$lib/types/image";
import {
  imagePath,
  imageUrl,
  zoomLevel,
  isInfoSidebarVisible,
  isOptionsMenuVisible,
  activeActions,
  isFullscreenActive,
  isLanguageMenuVisible,
  isThemeMenuVisible,
  isHotkeysMenuVisible,
  isToolbarMenuVisible,
  isImageNameDisplayMenuVisible,
  isEdgeIndicatorMenuVisible,
  isAppWindowMenuVisible,
  isSaveAsMenuVisible,
  isRefittingOnResize,
  imageFormat,
  imageResolution,
  imageAspectRatio,
  imageColorDepth,
  imageExif,
  isZoomModifierUpActive,
  isZoomModifierDownActive,
  imageFileSize,
  rotation,
  isConvertedToAscii,
  isGridOverlayVisible,
  isZenModeActive,
  appConfig,
} from "$lib/stores";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { showNotification } from "$lib/utils/toastNotifications";

/**
 * Updates the image stores with the new metadata.
 * If the image has a valid width and height, it will also update the image resolution and aspect ratio.
 * If the image does not have a valid width and height, it will reset the image resolution and aspect ratio to null.
 * @param {ImageMetadata} metadata - the new image metadata to update the stores with
 */
export const updateImageStores = (metadata: ImageMetadata) => {
  imageExif.set(metadata.exif_data);
  imageFormat.set(metadata.format);
  imageColorDepth.set(metadata.color_depth);
  imageFileSize.set(metadata.file_size);

  if (metadata.width > 0 && metadata.height > 0) {
    imageResolution.set({ width: metadata.width, height: metadata.height });
    imageAspectRatio.set(metadata.aspect_ratio);
  } else {
    imageResolution.set(null);
    imageAspectRatio.set(null);
  }
};

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
 * Opens a file dialog to select an image file, and then reads the selected file.
 * Updates the image stores with the new image data, and resets the zoom level to 1.
 * Starts and stops feedback for the "openFile" action during the operation.
 * @returns {Promise<void>}
 */
export const openFile = async (): Promise<void> => {
  startFeedback("openFile");
  try {
    const result = await invoke<[ImageMetadata, string, string[]] | null>("open_and_read_file");

    if (result) {
      const [metadata, path, _] = result;
      imageUrl.set(metadata.image_data);
      imagePath.set(path);
      updateImageStores(metadata);
      zoomLevel.set(1);
      rotation.set(0);
      isConvertedToAscii.set(false);
    }
  } catch (error) {
    console.error("Failed to open and read file:", error);
  } finally {
    stopFeedback("openFile");
  }
};

/**
 * Changes the current image to the next or previous image in the directory.
 * The function reads the new image file and updates the image stores with the new image data.
 * The zoom level is reset to 1.
 * If the function is called while there is no current image, it will return immediately without doing anything.
 * @param {string} direction - the direction of the image change, either "next" or "previous"
 * @returns {Promise<void>}
 */
const changeImage = async (direction: "next" | "previous"): Promise<void> => {
  const currentPath = get(imagePath);
  if (!currentPath) return;

  try {
    const [metadata, newPath] = await invoke<[ImageMetadata, string]>("change_image", {
      currentPath,
      direction,
    });
    imageUrl.set(metadata.image_data);
    imagePath.set(newPath);
    updateImageStores(metadata);
    zoomLevel.set(1);
    rotation.set(0);
    isConvertedToAscii.set(false);
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

/**
 * Rotates the image 90 degrees to the right.
 */
export const rotateClockwise = () => {
  singleShotFeedback("rotateClockwise");
  rotation.update((r) => (r + 90) % 360);
};

/**
 * Rotates the image 90 degrees to the left.
 */
export const rotateCounterclockwise = () => {
  singleShotFeedback("rotateCounterclockwise");
  rotation.update((r) => (r - 90 + 360) % 360);
};

/**
 * Saves the current image to a new file with a different format.
 * Uses a unified approach that works for both normal and ASCII-converted images.
 * @param {string} format - The new format to save the image as (e.g., "png", "jpg").
 * @param {number | undefined} quality - The quality of the saved image (0-100).
 * @returns {Promise<void>}
 */
export const saveImageAs = async (format: string, quality: number | undefined): Promise<void> => {
  const currentPath = get(imagePath);
  if (!currentPath) return;

  startFeedback("saveImageAs");
  try {
    const imageData = get(imageUrl);
    if (imageData) {
      const base64Data = imageData.split(",")[1];
      if (base64Data) {
        const isAscii = get(isConvertedToAscii);
        const sourceName = isAscii ? "ascii_art" : currentPath;

        await invoke<string | null>("save_base64_image_as", {
          base64data: base64Data,
          sourceName,
          format,
          quality,
          rotation: get(rotation),
        });
      }
    }
  } catch (error) {
    console.error("Failed to save image:", error);
  } finally {
    stopFeedback("saveImageAs");
  }
};

// --- Zoom Actions ---

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
 * Returns the step size for continuous zooming based on the current state of the zoom modifier keys.
 * If the zoom modifier up key is active, the step size is 0.3.
 * If the zoom modifier down key is active, the step size is 0.02.
 * Otherwise, the step size is 0.1.
 * @returns {number} - the step size for continuous zooming
 */
const getZoomStep = (): number => {
  if (get(isZoomModifierUpActive)) {
    return 0.3;
  }
  if (get(isZoomModifierDownActive)) {
    return 0.02;
  }
  return 0.1;
};

/**
 * Starts continuous zooming in of the image.
 * Zooms in the image by increasing the zoom level by 0.1 every 50ms.
 * Stops any existing continuous zooming and starts feedback for the "zoomIn" action.
 */
export const startZoomIn = () => {
  stopContinuousZoom();
  startFeedback("zoomIn");
  const step = getZoomStep();
  zoomLevel.update((level) => level + step);
  zoomInterval = setInterval(() => {
    zoomLevel.update((level) => level + step);
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

  const step = getZoomStep();

  zoomLevel.update((level) => {
    const newLevel = level - step;
    return newLevel < 0.1 ? 0.1 : newLevel;
  });

  zoomInterval = setInterval(() => {
    zoomLevel.update((level) => {
      const newLevel = level - step;
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

// --- Fullscreen Action ---

/**
 * Toggles the fullscreen mode of the application window.
 * If the application window is not in fullscreen mode, it will be set to fullscreen mode, and vice versa.
 * Starts feedback for the "toggleFullscreen" action, and then updates the isFullscreenActive store with the new state.
 * Also exits zen mode if fullscreen is toggled manually.
 * @returns {Promise<void>}
 */
export const toggleFullscreen = async (): Promise<void> => {
  singleShotFeedback("toggleFullscreen");

  const fullscreen = !get(isFullscreenActive);

  if (fullscreen) closeAllOpenMenus();

  if (get(isZenModeActive)) isZenModeActive.set(false);

  isRefittingOnResize.set(true);
  await getCurrentWindow().setFullscreen(fullscreen);
  setTimeout(() => {
    isRefittingOnResize.set(false);
  }, 100);

  isFullscreenActive.set(fullscreen);
};

// --- UI Toggle Actions ---

/**
 * Toggles the visibility of the EXIF sidebar.
 * Starts feedback for the "toggleExif" action, and then updates the isInfoSidebarVisible store.
 */
export const toggleExif = () => {
  singleShotFeedback("toggleExif");
  isInfoSidebarVisible.update((isOpen) => !isOpen);
};

/**
 * Toggles the visibility of the options sidebar.
 */
export const toggleOptions = () => {
  isOptionsMenuVisible.update((isOpen) => !isOpen);
};

/**
 * Toggles the visibility of the "Save As" menu.
 * If no image is open, this function does nothing.
 * Starts feedback for the "saveImageAs" action, and then updates the isSaveAsMenuVisible store.
 */
export const toggleSaveAsMenu = () => {
  const isImageOpen = get(imageUrl);
  if (!isImageOpen) return;

  singleShotFeedback("saveImageAs");
  isSaveAsMenuVisible.update((isOpen) => !isOpen);
};

/**
 * Converts the currently open image to ASCII art and updates the imageUrl store with the result.
 * If no image is open, this function does nothing and logs a warning to the console.
 * @returns {Promise<void>}
 */
export const convertToAsciiArt = async (): Promise<void> => {
  const path = get(imagePath);

  if (!path) {
    console.warn("No image path available for conversion");
    return;
  }

  singleShotFeedback("convertToAscii");

  try {
    const asciiImageData = (await invoke("convert_image_to_ascii_art", { path })) as string;

    imageUrl.set(asciiImageData);
    isConvertedToAscii.set(true);
  } catch (error) {
    console.error("Failed to convert image to ASCII art:", error);
  }
};

/**
 * Toggles the visibility of the grid overlay.
 * Starts feedback for the "toggleGridOverlay" action, and then updates the isGridOverlayVisible store.
 */
export const toggleGridOverlay = () => {
  singleShotFeedback("toggleGridOverlay");
  isGridOverlayVisible.update((isVisible) => !isVisible);
};

/**
 * Handles toggling the fullscreen mode of the application window.
 * Sets the isRefittingOnResize store to true, and then sets the fullscreen mode of the current window to the specified value.
 * Waits for the window resize to complete by delaying for 100ms.
 * Finally, sets the isRefittingOnResize store to false and updates the isFullscreenActive store with the new value.
 * @param {boolean} shouldBeFullscreen - Whether the application window should be in fullscreen mode.
 * @returns {Promise<void>}
 */
async function handleFullscreenChange(shouldBeFullscreen: boolean): Promise<void> {
  isRefittingOnResize.set(true);
  if (shouldBeFullscreen) {
    await getCurrentWindow().setFullscreen(true);
  } else {
    await getCurrentWindow().setFullscreen(false);
  }

  // Delay to allow window resize to complete
  await new Promise((resolve) => setTimeout(resolve, 100));
  isRefittingOnResize.set(false);
  isFullscreenActive.set(shouldBeFullscreen);
}

/**
 * Toggles the zen mode of the application window.
 * When entering zen mode, automatically enables fullscreen if not already active.
 * When exiting zen mode, automatically disables fullscreen if it was enabled by zen mode.
 * Shows a notification when entering zen mode with the shortcut to exit.
 * @returns {Promise<void>}
 */
export const toggleZenMode = async (): Promise<void> => {
  const hasImage = !!get(imageUrl);
  if (!hasImage) return;

  singleShotFeedback("toggleZenMode");

  const currentZenMode = get(isZenModeActive);
  const newZenMode = !currentZenMode;
  const isCurrentlyFullscreen = get(isFullscreenActive);

  if (newZenMode && !isCurrentlyFullscreen) {
    await handleFullscreenChange(true);
  } else if (!newZenMode && isCurrentlyFullscreen && currentZenMode) {
    await handleFullscreenChange(false);
  }

  isZenModeActive.set(newZenMode);

  if (newZenMode) {
    const zenModeShortcut = get(appConfig).shortcuts.toggleZenMode;
    showNotification(
      `Zen mode activated. Press ${zenModeShortcut.label.toLocaleUpperCase()} to exit.`
    );
  }
};

// --- Utility Functions ---

/**
 * Gets the application version from the backend.
 * @returns {Promise<string>} The application version
 */
export const getAppVersion = async (): Promise<string> => {
  return await invoke("get_app_version");
};

/**
 * Closes all open menus in the application.
 * This function is used to reset the application state when certain actions are taken.
 * For example, when the user toggles the fullscreen mode, all open menus will be closed.
 */
const closeAllOpenMenus = () => {
  isInfoSidebarVisible.set(false);
  isOptionsMenuVisible.set(false);
  isLanguageMenuVisible.set(false);
  isThemeMenuVisible.set(false);
  isHotkeysMenuVisible.set(false);
  isToolbarMenuVisible.set(false);
  isImageNameDisplayMenuVisible.set(false);
  isEdgeIndicatorMenuVisible.set(false);
  isAppWindowMenuVisible.set(false);
  isZenModeActive.set(false);
};
