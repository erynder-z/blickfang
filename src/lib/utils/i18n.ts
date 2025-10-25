import { writable } from 'svelte/store';

const modules = import.meta.glob('../../i18n/**/*.json', { eager: true });

export const locales: Record<string, Record<string, string>> = {};
for (const path in modules) {
    const locale = path.match(/..\/..\/i18n\/(.*?)\//)?.[1];
    if (locale) {
        locales[locale] = (modules[path] as any).default;
    }
}

export const locale = writable<string>('en');
export const t = writable<Record<string, string>>(locales.en);

export const setLocale = (newLocale: string) => {
  if (newLocale in locales) {
    locale.set(newLocale);
    t.set(locales[newLocale]);
  }
};