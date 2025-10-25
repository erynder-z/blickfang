// src/lib/themeManager.ts
import themes from "./themes.json";
import { appConfig } from "$lib/stores/appState";

const setColors = (theme: string) => {
  const themeColors = (themes as Record<string, Record<string, string>>)[theme];
  if (!themeColors) {
    console.warn(`Theme '${theme}' not found. Using default.`);
    setColors("default");
    return;
  }
  for (const [key, value] of Object.entries(themeColors)) {
    document.documentElement.style.setProperty(`--color-${key}`, value as string);
  }
};

export const initThemeManager = () => {
  appConfig.subscribe((config) => {
    if (config && config.theme) {
      setColors(config.theme);
    }
  });
};
