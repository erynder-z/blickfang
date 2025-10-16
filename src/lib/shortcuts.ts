import { get } from "svelte/store";
import { appConfig } from "$lib/store";
import * as appActions from "$lib/actions";

const singleShotActions: Record<string, () => void> = {
  openFile: appActions.openFile,
  previousImage: appActions.previousImage,
  nextImage: appActions.nextImage,
  toggleExif: appActions.toggleExif,
  toggleOptions: appActions.toggleOptions,
};

let activeContinuousKey: string | null = null;

export const handleKeyDown = (event: KeyboardEvent) => {
  if (event.repeat || activeContinuousKey) return;

  const shortcuts = get(appConfig).shortcuts;
  for (const actionName in shortcuts) {
    const shortcut = shortcuts[actionName as keyof typeof shortcuts];
    if (shortcut.keys.includes(event.key)) {
      if (actionName === "zoomIn") {
        appActions.startZoomIn();
        activeContinuousKey = event.key;
      } else if (actionName === "zoomOut") {
        appActions.startZoomOut();
        activeContinuousKey = event.key;
      } else {
        const action = singleShotActions[actionName];
        if (action) action();
      }
      event.preventDefault();
      return;
    }
  }
};

export const handleKeyUp = (event: KeyboardEvent) => {
  if (event.key === activeContinuousKey) {
    appActions.stopContinuousZoom();
    activeContinuousKey = null;
    event.preventDefault();
  }
};
