import { writable } from "svelte/store";
import type { AppConfig } from "$lib/types/app";

export const appConfig = writable<AppConfig>({
  language: "en",
  theme: "default",
  toolbarButtonSize: "large",
  imageNameDisplayMode: "fade",
  edgeIndicatorsVisible: false,
  rememberWindowSize: false,
  shortcuts: {
    openFile: { keys: [], label: "" },
    saveImageAs: { keys: [], label: "" },
    previousImage: { keys: [], label: "" },
    nextImage: { keys: [], label: "" },
    zoomIn: { keys: [], label: "" },
    zoomOut: { keys: [], label: "" },
    toggleExif: { keys: [], label: "" },
    toggleOptions: { keys: [], label: "" },
    zoomModifierUp: { keys: [], label: "" },
    zoomModifierDown: { keys: [], label: "" },
  },
  customShortcuts: {
    openFile: { keys: [], label: "" },
    saveImageAs: { keys: [], label: "" },
    previousImage: { keys: [], label: "" },
    nextImage: { keys: [], label: "" },
    zoomIn: { keys: [], label: "" },
    zoomOut: { keys: [], label: "" },
    toggleExif: { keys: [], label: "" },
    toggleOptions: { keys: [], label: "" },
    zoomModifierUp: { keys: [], label: "" },
    zoomModifierDown: { keys: [], label: "" },
  },
});
