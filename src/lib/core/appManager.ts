import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { setLocale } from "$lib/utils/i18n";
import { imageUrl, imagePath, appConfig, type AppConfig } from "$lib/stores/appState";
import { processImage } from "../utils/imageProcessor";

/**
 * Handles an event from the image source event listener.
 * The event payload should contain an array of strings representing the paths to the images.
 * If the array is not empty, the first path will be used to read the image from the file system,
 * and the image data will be set to the image url and image path stores.
 * The image will then be processed and its exif data will be set to the image exif store.
 * If there is an error reading the image from the file system, an error will be logged to the console.
 * @param {string[]} event.payload - An array of strings representing the paths to the images.
 * @returns {Promise<void>}
 */
const handleImageSourceEvent = async (event: { payload: string[] }): Promise<void> => {
  const paths = event.payload;
  if (paths.length > 0) {
    const path = paths[0];

    try {
      const result = await invoke("read_image_from_path", { path });
      const [dataUrl, exifData, newPathStr, _directoryFiles] = result as [
        string,
        string,
        string,
        string[],
      ];
      imageUrl.set(dataUrl);
      imagePath.set(newPathStr);
      processImage(dataUrl, newPathStr, exifData);
    } catch (error) {
      console.error("Failed to read image from path:", error);
    }
  }
};

/**
 * Handles an event from the config-updated event listener.
 * The event payload should contain an AppConfig object representing the updated app configuration.
 * The app configuration will be updated with the new values, and the locale will be set to the language specified in the app configuration.
 * @param {object} event - An object containing the event payload.
 * @param {AppConfig} event.payload - An AppConfig object representing the updated app configuration.
 */
const handleConfigUpdatedEvent = (event: { payload: AppConfig }) => {
  appConfig.set(event.payload);
  setLocale(event.payload.language);
};

/**
 * Registers event listeners for the "image-source" and "config-updated" events.
 * The "image-source" event is triggered when the image source event listener is triggered,
 * and the event payload should contain an array of strings representing the paths to the images.
 * The "config-updated" event is triggered when the app configuration is updated,
 * and the event payload should contain an AppConfig object representing the updated app configuration.
 * The function returns an object containing two functions, unlistenImageSource and unlistenConfig,
 * which can be used to unregister the event listeners.
 * @returns {Promise<{ unlistenImageSource: () => void, unlistenConfig: () => void }>}
 */
const registerEventListeners = async (): Promise<{
  unlistenImageSource: () => void;
  unlistenConfig: () => void;
}> => {
  const unlistenImageSource = await listen<string[]>("image-source", handleImageSourceEvent);

  await invoke("frontend_is_ready");

  const unlistenConfig = await listen<AppConfig>("config-updated", handleConfigUpdatedEvent);

  return { unlistenImageSource, unlistenConfig };
};

/**
 * Invokes the event that will show the main window of the app.
 * The window will be shown after a short delay (150ms) to allow the frontend to finish initializing.
 */
const showMainWindow = () => {
  const timeout = 150;
  setTimeout(() => {
    invoke("show_window");
  }, timeout);
};

/**
 * Initializes the app by registering event listeners for the "image-source" and "config-updated" events.
 * The event listeners are registered asynchronously, and the main window of the app is shown after a short delay (150ms) to allow the frontend to finish initializing.
 * The function returns a function that can is used to unregister the event listeners.
 * @returns {() => void} - A function that is used to unregister the event listeners.
 */
export const initializeApp = () => {
  let unlistenImageSource: () => void;
  let unlistenConfig: () => void;

  (async () => {
    const unlisteners = await registerEventListeners();
    unlistenImageSource = unlisteners.unlistenImageSource;
    unlistenConfig = unlisteners.unlistenConfig;
  })();

  showMainWindow();

  return () => {
    unlistenImageSource?.();
    unlistenConfig?.();
  };
};
