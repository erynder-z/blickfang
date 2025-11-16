import { writable, derived } from "svelte/store";

const modules = import.meta.glob("../../i18n/**/*.json", { eager: true });

export const locales: Record<string, Record<string, string>> = {};
for (const path in modules) {
  const locale = path.match(/..\/..\/i18n\/(.*?)\//)?.[1];
  if (locale) {
    locales[locale] = (modules[path] as any).default;
  }
}

export const locale = writable<string>("en");
export const t = derived(locale, ($locale) => locales[$locale]);

/**
 * Sets the current locale and updates the translations.
 * @param {string} newLocale - The new locale to set.
 */
export const setLocale = (newLocale: string) => {
  if (newLocale in locales) {
    locale.set(newLocale);
  }
};
