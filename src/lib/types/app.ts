export type Shortcut = {
  keys: string[];
  label: string;
};

export type Shortcuts = {
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
  toggleGridOverlay: Shortcut;
};

export type AppConfig = {
  language: string;
  theme: string;
  shortcuts: Shortcuts;
  customShortcuts: Shortcuts;
  toolbarButtonSize: "large" | "small" | "hide";
  imageNameDisplayMode: "show" | "hide" | "fade";
  edgeIndicatorsVisible: boolean;
  rememberWindowSize: boolean;
  linuxDesktopInstallChoice?: "installed" | "skipped" | "not_asked";
  asciiChars:
    | "blocks"
    | "stripes"
    | "minimal"
    | "classic"
    | "extended"
    | "dense_gradient"
    | "line_art"
    | "high_detail";
  asciiBackgroundColor?: string;
  asciiAutoBackground?: boolean;
  gridOverlayMode: "golden-ratio" | "rule-of-thirds" | "grid";
  gridColor?: string;
  gridLineStrength?: number;
};
