import { get } from "svelte/store";
import {
  imagePath,
  imageUrl,
  imageExif,
  zoomLevel,
  isExifSidebarVisible,
  isOptionsSidebarVisible,
  activeActions,
} from "$lib/store";
import { invoke } from "@tauri-apps/api/core";

const triggerFeedback = (actionName: string) => {
  activeActions.update((actions) => [...actions, actionName]);
  setTimeout(() => {
    activeActions.update((actions) => actions.filter((a) => a !== actionName));
  }, 200);
};

export const openFile = async () => {
  activeActions.update((actions) => [...actions, "openFile"]);
  try {
    const [newImageData, newImageExif, newImagePath] =
      await invoke<[string, string, string]>("open_and_read_file");
    if (newImageData) {
      imageUrl.set(newImageData);
      imageExif.set(newImageExif);
      imagePath.set(newImagePath);
    }
  } catch (error) {
    console.error("Failed to open and read file:", error);
  } finally {
    activeActions.update((actions) => actions.filter((a) => a !== "openFile"));
  }
};

const changeImage = async (direction: "next" | "previous") => {
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
    imageExif.set(newImageExif);
  } catch (error) {
    console.error("Failed to change image:", error);
  }
};

export const previousImage = () => {
  triggerFeedback("previousImage");
  changeImage("previous");
};

export const nextImage = () => {
  triggerFeedback("nextImage");
  changeImage("next");
};

export const zoomIn = () => {
  triggerFeedback("zoomIn");
  zoomLevel.update((level) => level + 0.25);
};

export const zoomOut = () => {
  triggerFeedback("zoomOut");
  zoomLevel.update((level) => {
    const newLevel = level - 0.25;
    return newLevel < 0.1 ? 0.1 : newLevel;
  });
};

export const toggleExif = () => {
  triggerFeedback("toggleExif");
  isExifSidebarVisible.update((isOpen) => !isOpen);
};

export const toggleOptions = () => {
  triggerFeedback("toggleOptions");
  isOptionsSidebarVisible.update((isOpen) => !isOpen);
};

let zoomInterval: ReturnType<typeof setInterval> | null = null;

export const stopContinuousZoom = () => {
  if (zoomInterval) {
    clearInterval(zoomInterval);
    zoomInterval = null;
  }
  activeActions.update((actions) => actions.filter((a) => a !== "zoomIn" && a !== "zoomOut"));
};

export const startZoomIn = () => {
  stopContinuousZoom();
  activeActions.update((actions) => [...actions, "zoomIn"]);
  zoomLevel.update((level) => level + 0.1);
  zoomInterval = setInterval(() => {
    zoomLevel.update((level) => level + 0.1);
  }, 50);
};

export const startZoomOut = () => {
  stopContinuousZoom();
  activeActions.update((actions) => [...actions, "zoomOut"]);
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
