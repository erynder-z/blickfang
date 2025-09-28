export function lang(node: HTMLElement, lang: string) {
  document.documentElement.lang = lang;

  return {
    update(newLang: string) {
      document.documentElement.lang = newLang;
    }
  };
}
