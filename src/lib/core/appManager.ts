import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { setLocale } from "$lib/utils/i18n";
import {
  imageUrl,
  imagePath,
  appConfig,
  type AppConfig,
  aiDetectionResult,
  type AiDetectionResult,
} from "$lib/stores/appState";
import { type ImageMetadata, updateImageStores } from "./commands";

/**
 * Runs AI detection on the given image path and updates the aiDetectionResult store.
 * If the path is null, it sets the aiDetectionResult store to null.
 * @param {string | null} path - The path to the image file
 * @returns {Promise<void>}
 */
const runAiDetection = async (path: string | null): Promise<void> => {
  if (!path) {
    aiDetectionResult.set(null);
    return;
  }

  try {
    const result = await invoke<AiDetectionResult>("detect_ai_image", { path });
    aiDetectionResult.set(result);
  } catch (error) {
    console.error("Failed to detect AI image:", error);
    aiDetectionResult.set(null);
  }
};

/**
 * Handles the "image-source" event.
 * This event is triggered when the image source event listener is triggered,
 * and the event payload should contain an array of strings representing the paths to the images.
 * The function reads the first image from the paths and updates the image stores.
 * @param {Object} event - The event object containing the paths to the images.
 * @returns {Promise<void>}
 */
const handleImageSourceEvent = async (event: { payload: string[] }): Promise<void> => {
  const paths = event.payload;
  if (paths.length > 0) {
    const path = paths[0];

    try {
      const [metadata, newPathStr, _directoryFiles] = await invoke<
        [ImageMetadata, string, string[]]
      >("read_image_from_path", { path });
      imageUrl.set(metadata.image_data);
      imagePath.set(newPathStr);
      updateImageStores(metadata);
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
  let unsubscribeImagePath: () => void;

  (async () => {
    const unlisteners = await registerEventListeners();
    unlistenImageSource = unlisteners.unlistenImageSource;
    unlistenConfig = unlisteners.unlistenConfig;
  })();

  unsubscribeImagePath = imagePath.subscribe(runAiDetection);

  showMainWindow();

  return () => {
    unlistenImageSource?.();
    unlistenConfig?.();
    unsubscribeImagePath?.();
  };
};
