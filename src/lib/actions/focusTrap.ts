/**
 * Creates a focus trap for the given node.
 * Restricts keyboard focus to a specific area of the page.
 * It will focus the first focusable element when the user presses the "Tab" key without the shift key,
 * and it will focus the last focusable element when the user presses the "Tab" key with the shift key.
 * @param {HTMLElement} node The node for which to create the focus trap.
 * @returns {Object} An object with a single property, "destroy", which is a function that can be used to destroy the focus trap.
 */
export function focusTrap(node: HTMLElement): object {
  const focusableElements = node.querySelectorAll<HTMLElement>(
    'a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), details, [tabindex]:not([tabindex="-1"])'
  );
  const firstElement = focusableElements[0];
  const lastElement = focusableElements[focusableElements.length - 1];

  /**
   * Handles the keydown event for the focus trap.
   * If the key is "Tab" and the shift key is pressed, it will focus the last focusable element.
   * If the key is "Tab" and the shift key is not pressed, it will focus the first focusable element.
   * @param {KeyboardEvent} event The keydown event.
   */
  function handleKeydown(event: KeyboardEvent) {
    if (event.key !== "Tab") return;

    if (event.shiftKey) {
      if (document.activeElement === firstElement) {
        lastElement.focus();
        event.preventDefault();
      }
    } else {
      if (document.activeElement === lastElement) {
        firstElement.focus();
        event.preventDefault();
      }
    }
  }

  document.addEventListener("keydown", handleKeydown);

  return {
    /**
     * Destroys the focus trap by removing the event listener.
     */
    destroy() {
      document.removeEventListener("keydown", handleKeydown);
    },
  };
}
