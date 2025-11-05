import { writable } from "svelte/store";

export interface Shortcut {
  keys: string[];
  label: string;
}

export interface Shortcuts {
  openFile: Shortcut;
  saveImageAs: Shortcut;
  previousImage: Shortcut;
  nextImage: Shortcut;
  zoomIn: Shortcut;
  zoomOut: Shortcut;
  toggleExif: Shortcut;
  toggleOptions: Shortcut;
  zoomModifierUp: Shortcut;
  zoomModifierDown: Shortcut;
}

export interface AppConfig {
  language: string;
  theme: string;
  shortcuts: Shortcuts;
  customShortcuts: Shortcuts;
  toolbarButtonSize: "large" | "small" | "hide";
  imageNameDisplayMode: "show" | "hide" | "fade";
  edgeIndicatorsVisible: boolean;
  rememberWindowSize: boolean;
}

export const imageUrl = writable<string | null>(null);
export const imagePath = writable<string | null>(null);
export const imageExif = writable<string | null>(null);
export const imageFormat = writable<string | null>(null);
export const imageResolution = writable<{ width: number; height: number } | null>(null);
export const imageAspectRatio = writable<string | null>(null);
export const zoomLevel = writable<number>(1);
export const edgeIndicators = writable({
  top: false,
  bottom: false,
  left: false,
  right: false,
});
export const indicatorsVisible = writable(false);
export const isExifSidebarVisible = writable<boolean>(false);
export const isOptionsMenuVisible = writable<boolean>(false);
export const isLanguageMenuVisible = writable<boolean>(false);
export const isThemeMenuVisible = writable<boolean>(false);
export const isHotkeysMenuVisible = writable<boolean>(false);
export const isToolbarMenuVisible = writable<boolean>(false);
export const isImageNameDisplayMenuVisible = writable<boolean>(false);
export const isEdgeIndicatorMenuVisible = writable<boolean>(false);
export const isAppWindowMenuVisible = writable<boolean>(false);
export const isSaveAsMenuVisible = writable<boolean>(false);
export const isRemapping = writable<boolean>(false);
export const activeActions = writable<string[]>([]);
export const isFullscreenActive = writable<boolean>(false);
export const isRefittingOnResize = writable<boolean>(false);
export const isZoomModifierUpActive = writable<boolean>(false);
export const isZoomModifierDownActive = writable<boolean>(false);
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
