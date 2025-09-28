import { writable } from "svelte/store";

export const locales: Record<string, Record<string, string>> = {
  en: {
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "Select an image to view",
    "image.name.no-data": "No image selected!",
    "options.title": "Options",
    "options.hotkeys": "Hotkeys",
    "options.language": "Language",
    "exif.title": "Exif Data",
    "exif.no-data": "No Exif Data",
  },
  de: {
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "Wählen Sie ein Bild zur Ansicht",
    "image.name.no-data": "Kein Bild ausgewählt!",
    "options.title": "Optionen",
    "options.hotkeys": "Hotkeys",
    "options.language": "Sprache",
    "exif.title": "Exif-Daten",
    "exif.no-data": "Keine Exif-Daten",
  },
  ja: {
    "app.title": "Simple Image Viewer",
    "image-view.placeholder": "表示する画像を選択してください",
    "image.name.no-data": "画像が選択されていません！",
    "options.title": "オプション",
    "options.hotkeys": "ショートカットキー",
    "options.language": "言語",
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
