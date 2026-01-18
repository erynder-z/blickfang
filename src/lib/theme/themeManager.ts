import themes from "$lib/theme/themes.json";
import { appConfig } from "$lib/stores";

/**
 * Sets CSS custom properties for the given theme.
 * If the theme is not found, it will fall back to the default theme.
 * @param {string} theme - The theme to set the colors for.
 */
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

/**
 * Initializes the theme manager by subscribing to appConfig.
 * Whenever the theme is updated in appConfig, the theme manager will update the CSS custom properties to reflect the new theme.
 */
export const initThemeManager = () => {
  appConfig.subscribe((config) => {
    if (config && config.theme) setColors(config.theme);
  });
};
