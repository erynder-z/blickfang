import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { setLocale } from "$lib/utils/i18n";
import { imageUrl, imagePath, appConfig, type AppConfig } from "$lib/stores/appState";
import { processImage } from "../utils/imageProcessor";

const handleImageSourceEvent = async (event: { payload: string[] }) => {
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

const handleConfigUpdatedEvent = (event: { payload: AppConfig }) => {
  appConfig.set(event.payload);
  setLocale(event.payload.language);
};

const registerEventListeners = async () => {
  const unlistenImageSource = await listen<string[]>("image-source", handleImageSourceEvent);

  await invoke("frontend_is_ready");

  const unlistenConfig = await listen<AppConfig>("config-updated", handleConfigUpdatedEvent);

  return { unlistenImageSource, unlistenConfig };
};

const showMainWindow = () => {
  const timeout = 150;
  setTimeout(() => {
    invoke("show_window");
  }, timeout);
};

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
