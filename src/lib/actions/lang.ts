/**
 * Sets the lang attribute of the html element to the given language.
 * Returns an object with an update method, which can be used to update the lang attribute.
 * @param {HTMLElement} node - The node to set the lang attribute on.
 * @param {string} lang - The language to set.
 * @returns {Object} - An object with an update method.
 */
export function lang(node: HTMLElement, lang: string): object {
  document.documentElement.lang = lang;

  return {
    update(newLang: string) {
      document.documentElement.lang = newLang;
    },
  };
}
