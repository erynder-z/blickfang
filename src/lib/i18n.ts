import { writable } from "svelte/store";

export const locales: Record<string, Record<string, string>> = {
  en: {
    "language.name": "English",
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "Select an image to view",
    "image.name.no-data": "No image selected!",
    "options.title": "Options",
    "options.button.hotkeys": "Hotkeys",
    "options.button.language": "Language",
    "options.language.heading": "Select Language",
    "exif.title": "Exif Data",
    "exif.no-data": "No Exif Data",
  },
  de: {
    "language.name": "Deutsch",
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "Wählen Sie ein Bild zur Ansicht",
    "image.name.no-data": "Kein Bild ausgewählt!",
    "options.title": "Optionen",
    "options.button.hotkeys": "Hotkeys",
    "options.button.language": "Sprache",
    "options.language.heading": "Sprache auswhlen",
    "exif.title": "Exif-Daten",
    "exif.no-data": "Keine Exif-Daten",
  },
  ja: {
    "language.name": "日本語",
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "表示する画像を選択してください",
    "image.name.no-data": "画像が選択されていません！",
    "options.title": "オプション",
    "options.button.hotkeys": "ショートカットキー",
    "options.button.language": "言語",
    "options.language.heading": "言語を選択してください",
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
