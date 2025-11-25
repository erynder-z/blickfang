import { writable } from "svelte/store";

/**
 * Creates a store for managing a tooltip.
 *
 * The store has three properties: `text`, `visible`, and `targetRect`.
 * `text` is the text to display in the tooltip.
 * `visible` is a boolean indicating whether the tooltip is currently visible.
 * `targetRect` is the bounding rectangle of the element that the tooltip should be displayed next to.
 *
 * The store provides three methods: `show`, `hide`, and `subscribe`.
 * `show` takes a text and an element and displays the tooltip with the given text next to the element after a short delay.
 * `hide` hides the tooltip.
 * `subscribe` returns a subscription to the store that can be used to observe changes to the store's state.
 */
export const createTooltipStore = () => {
  const { subscribe, set } = writable<{
    text: string;
    visible: boolean;
    targetRect: DOMRect | null;
  }>({
    text: "",
    visible: false,
    targetRect: null,
  });

  let timeout: number;

  return {
    subscribe,
    show: (text: string, target: HTMLElement) => {
      clearTimeout(timeout);
      timeout = window.setTimeout(() => {
        const rect = target.getBoundingClientRect();
        set({ text, visible: true, targetRect: rect });
      }, 1500);
    },
    hide: () => {
      clearTimeout(timeout);
      set({ text: "", visible: false, targetRect: null });
    },
  };
};
