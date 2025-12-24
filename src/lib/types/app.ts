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
  rotateClockwise: Shortcut;
  rotateCounterclockwise: Shortcut;
  toggleExif: Shortcut;
  toggleOptions: Shortcut;
  zoomModifierUp: Shortcut;
  zoomModifierDown: Shortcut;
  toggleFullscreen: Shortcut;
  convertToAsciiArt: Shortcut;
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
  linuxDesktopInstallChoice?: "installed" | "skipped" | "not_asked";
}
