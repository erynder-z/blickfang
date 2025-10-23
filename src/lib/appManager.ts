import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { setLocale } from "$lib/i18n";
import { appConfig, imageUrl, imagePath } from "$lib/store";
import type { AppConfig } from "$lib/store";
import { processImage } from "./imageProcessor";

const handleImageSourceEvent = async (event: { payload: string[] }) => {
  const paths = event.payload;
  if (paths.length > 0) {
    const path = paths[0];
    imagePath.set(path);

    invoke("read_image_file", { path }).then(async (res) => {
      const [dataUrl, exifData] = res as [string, string];
      imageUrl.set(dataUrl);
      processImage(dataUrl, path, exifData);
    });
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
