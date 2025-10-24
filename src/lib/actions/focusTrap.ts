export function focusTrap(node: HTMLElement) {
  const focusableElements = node.querySelectorAll<HTMLElement>(
    'a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), details, [tabindex]:not([tabindex="-1"])'
  );
  const firstElement = focusableElements[0];
  const lastElement = focusableElements[focusableElements.length - 1];

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
    destroy() {
      document.removeEventListener("keydown", handleKeydown);
    },
  };
}
