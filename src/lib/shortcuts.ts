import { get } from "svelte/store";
import {
  imagePath,
  imageUrl,
  imageExif,
  zoomLevel,
  isExifSidebarVisible,
  isOptionsSidebarVisible,
  appConfig,
} from "$lib/store";
import { invoke } from "@tauri-apps/api/core";

const openFile = async () => {
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

const zoomIn = () => {
  zoomLevel.update((level) => level + 0.25);
};

const zoomOut = () => {
  zoomLevel.update((level) => {
    const newLevel = level - 0.25;
    return newLevel < 0.1 ? 0.1 : newLevel;
  });
};

const toggleExifInfo = () => {
  isExifSidebarVisible.update((isOpen) => !isOpen);
};

const toggleOptions = () => {
  isOptionsSidebarVisible.update((isOpen) => !isOpen);
};

const actions: Record<string, () => void> = {
  openFile: openFile,
  previousImage: () => changeImage("previous"),
  nextImage: () => changeImage("next"),
  zoomIn: zoomIn,
  zoomOut: zoomOut,
  toggleExif: toggleExifInfo,
  toggleOptions: toggleOptions,
};

export const handleKeyDown = (event: KeyboardEvent) => {
  if (event.ctrlKey || event.altKey || event.metaKey) return;

  const shortcuts = get(appConfig).shortcuts;
  for (const actionName in shortcuts) {
    const shortcut = shortcuts[actionName as keyof typeof shortcuts];
    if (shortcut.keys.includes(event.key)) {
      const action = actions[actionName];
      if (action) {
        action();
        event.preventDefault();
        return;
      }
    }
  }
};
