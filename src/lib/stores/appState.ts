import { writable } from "svelte/store";
import type { AppConfig } from "$lib/types/app";
import type { AiDetectionResult } from "$lib/types/image";

export const imageUrl = writable<string | null>(null);
export const imagePath = writable<string | null>(null);
export const imageExif = writable<string | null>(null);
export const imageFormat = writable<string | null>(null);
export const imageResolution = writable<{ width: number; height: number } | null>(null);
export const imageAspectRatio = writable<string | null>(null);
export const imageColorDepth = writable<number | null>(null);
export const zoomLevel = writable<number>(1);
export const edgeIndicators = writable({
  top: false,
  bottom: false,
  left: false,
  right: false,
});
export const indicatorsVisible = writable(false);
export const isInfoSidebarVisible = writable<boolean>(false);
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

export const aiDetectionResult = writable<AiDetectionResult | null>(null);

