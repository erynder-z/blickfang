import { writable } from "svelte/store";

export const locales: Record<string, Record<string, string>> = {
  en: {
    "language.name": "English",
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "Select an image to view",
    "options.title": "Options",
    "options.button.hotkeys": "Hotkeys",
    "options.button.language": "Language",
    "options.button.theme": "Theme",
    "options.language.heading": "Select Language",
    "options.theme.heading": "Select Theme",
    "exif.title": "Exif Data",
    "exif.no-data": "No Exif Data",
  },
  de: {
    "language.name": "Deutsch",
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "Wählen Sie ein Bild zur Ansicht",
    "options.title": "Optionen",
    "options.button.hotkeys": "Hotkeys",
    "options.button.language": "Sprache",
    "options.button.theme": "Thema",
    "options.language.heading": "Sprache auswhlen",
    "options.theme.heading": "Thema auswählen",
    "exif.title": "Exif-Daten",
    "exif.no-data": "Keine Exif-Daten",
  },
  ja: {
    "language.name": "日本語",
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "表示する画像を選択してください",
    "options.title": "オプション",
    "options.button.hotkeys": "ショートカットキー",
    "options.button.language": "言語",
    "options.button.theme": "テーマ",
    "options.language.heading": "言語を選択してください",
    "options.theme.heading": "テーマを選択",
    "exif.title": "Exifデータ",
    "exif.no-data": "Exifデータがありません",
  },
};

export const locale = writable<string>("en");
export const t = writable(locales.en);

export const setLocale = (newLocale: string) => {
  if (newLocale in locales) {
    locale.set(newLocale);
    t.set(locales[newLocale]);
  }
};
