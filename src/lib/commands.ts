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

// --- Feedback Management ---

const startFeedback = (actionName: string) => {
  activeActions.update((actions) => [...new Set([...actions, actionName])]);
};

const stopFeedback = (actionName: string) => {
  activeActions.update((actions) => actions.filter((a) => a !== actionName));
};

const singleShotFeedback = (actionName: string) => {
  startFeedback(actionName);
  setTimeout(() => stopFeedback(actionName), 150);
};

// --- Actions ---

// --- File Actions ---
export const openFile = async () => {
  startFeedback("openFile");
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
    stopFeedback("openFile");
  }
};

// --- Image Navigation ---
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
  singleShotFeedback("previousImage");
  changeImage("previous");
};

export const nextImage = () => {
  singleShotFeedback("nextImage");
  changeImage("next");
};

// --- Zoom Actions ---

// For single-step zoom (hotkeys)
export const zoomIn = () => {
  singleShotFeedback("zoomIn");
  zoomLevel.update((level) => level + 0.25);
};

export const zoomOut = () => {
  singleShotFeedback("zoomOut");
  zoomLevel.update((level) => {
    const newLevel = level - 0.25;
    return newLevel < 0.1 ? 0.1 : newLevel;
  });
};

// For continuous zoom (holding button/key)
let zoomInterval: ReturnType<typeof setInterval> | null = null;

export const stopContinuousZoom = () => {
  if (zoomInterval) {
    clearInterval(zoomInterval);
    zoomInterval = null;
  }
  stopFeedback("zoomIn");
  stopFeedback("zoomOut");
};

export const startZoomIn = () => {
  stopContinuousZoom();
  startFeedback("zoomIn");
  zoomLevel.update((level) => level + 0.1);
  zoomInterval = setInterval(() => {
    zoomLevel.update((level) => level + 0.1);
  }, 50);
};

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
const stopWheelZoom = () => {
  stopFeedback("zoomIn");
  stopFeedback("zoomOut");
};

export const triggerWheelZoom = (direction: "in" | "out") => {
  if (wheelZoomTimeout) {
    clearTimeout(wheelZoomTimeout);
  }
  stopFeedback(direction === "in" ? "zoomOut" : "zoomIn");
  startFeedback(direction === "in" ? "zoomIn" : "zoomOut");
  wheelZoomTimeout = setTimeout(stopWheelZoom, 150);
};

// --- UI Toggle Actions ---

export const toggleExif = () => {
  singleShotFeedback("toggleExif");
  isExifSidebarVisible.update((isOpen) => !isOpen);
};

export const toggleOptions = () => {
  singleShotFeedback("toggleOptions");
  isOptionsSidebarVisible.update((isOpen) => !isOpen);
};
