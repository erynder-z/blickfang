import themes from "$lib/theme/themes.json";
import { appConfig } from "$lib/stores";
import type { Unsubscriber } from "svelte/store";

/**
 * ThemeManager class responsible for managing application themes.
 * Handles theme initialization, application, and cleanup.
 */
export class ThemeManager {
  private unsubscribe: Unsubscriber | undefined;
  private readonly themes: Record<string, Record<string, string>>;

  /**
   * Creates a new ThemeManager instance.
   */
  constructor() {
    this.themes = themes;
  }

  /**
   * Sets CSS custom properties for the given theme.
   * If the theme is not found, it will fall back to the default theme.
   * @param {string} theme - The theme to set the colors for.
   */
  private setColors(theme: string) {
    const themeColors = this.themes[theme];
    if (!themeColors) {
      console.warn(`Theme '${theme}' not found. Using default.`);
      this.setColors("default");
      return;
    }
    for (const [key, value] of Object.entries(themeColors)) {
      document.documentElement.style.setProperty(`--color-${key}`, value as string);
    }
  }

  /**
   * Initializes the theme manager by subscribing to appConfig.
   * Whenever the theme is updated in appConfig, the theme manager will update the CSS custom properties to reflect the new theme.
   */
  public init() {
    this.unsubscribe = appConfig.subscribe((config) => {
      if (config && config.theme) {
        this.setColors(config.theme);
      }
    });
  }

  /**
   * Cleans up the theme manager by unsubscribing from the appConfig store.
   * Should be called when the theme manager is no longer needed.
   */
  public destroy() {
    this.unsubscribe?.();
    this.unsubscribe = undefined;
  }
}

/**
 * Singleton instance of ThemeManager for global use.
 */
export const themeManager = new ThemeManager();
