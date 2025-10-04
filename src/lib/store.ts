import { writable } from "svelte/store";

export interface Shortcut {
  keys: string[];
  label: string;
}

export interface Shortcuts {
  openFile: Shortcut;
  previousImage: Shortcut;
  nextImage: Shortcut;
  zoomIn: Shortcut;
  zoomOut: Shortcut;
  toggleExif: Shortcut;
  toggleOptions: Shortcut;
}

export interface AppConfig {
  language: string;
  theme: string;
  shortcuts: Shortcuts;
}

export const imageUrl = writable<string | null>(null);
export const imagePath = writable<string | null>(null);
export const imageExif = writable<string | null>(null);
export const zoomLevel = writable<number>(1);
export const edgeIndicators = writable({
  top: false,
  bottom: false,
  left: false,
  right: false,
});
export const indicatorsVisible = writable(false);
export const isExifSidebarVisible = writable<boolean>(false);
export const isOptionsSidebarVisible = writable<boolean>(false);
export const isLanguageMenuVisible = writable<boolean>(false);
export const isThemeMenuVisible = writable<boolean>(false);
export const isHotkeysMenuVisible = writable<boolean>(false);
export const appConfig = writable<AppConfig>({
  language: "en",
  theme: "default",
  shortcuts: {
    openFile: { keys: ["o"], label: "O" },
    previousImage: { keys: ["Arrow Left"], label: "←" },
    nextImage: { keys: ["Arrow Right"], label: "→" },
    zoomIn: { keys: ["+", "=", "Arrow Up"], label: "+ / ↑" },
    zoomOut: { keys: ["-", "_", "Arrow Down"], label: "- / ↓" },
    toggleExif: { keys: ["i"], label: "I" },
    toggleOptions: { keys: ["m"], label: "M" },
  },
});
