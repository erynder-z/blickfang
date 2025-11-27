import { triggerWheelZoom } from "$lib/core/commands";
import {
  edgeIndicators,
  indicatorsVisible,
  appConfig,
  isRefittingOnResize,
  isZoomModifierUpActive,
  isZoomModifierDownActive,
  rotation,
} from "$lib/stores";
import type { Writable } from "svelte/store";
import { get } from "svelte/store";

type ZoomPanOptions = {
  zoomLevelStore: Writable<number>;
  imageUrlStore: Writable<string | null>;
  onImageDrawn?: () => void;
};

/**
 * Attaches a zoom and pan functionality to a given canvas element.
 *
 * @param {HTMLCanvasElement} canvas - The canvas element to attach the functionality to.
 * @param {ZoomPanOptions} options - Options for the zoom and pan functionality.
 * @param {Writable<number>} options.zoomLevelStore - The writable store for the current zoom level.
 * @param {Writable<string | null>} options.imageUrlStore - The writable store for the current image URL.
 * @param {() => void} [options.onImageDrawn] - Optional callback to be called when the image is finished drawing.
 * @returns {Object | undefined}  Undefined if the canvas element is not found. Otherwise, returns an object with a single property, "destroy", which is a function that can be used to destroy the zoom and pan functionality.
 */
export const zoomPan = (canvas: HTMLCanvasElement, options: ZoomPanOptions): object | undefined => {
  const { zoomLevelStore, imageUrlStore, onImageDrawn } = options;
  const container = canvas.parentElement;
  if (!container) return;

  canvas.style.willChange = "transform";
  let ctx = canvas.getContext("2d");

  // --- State ---
  let image: HTMLImageElement | null = null;
  let isDragging = false;
  let lastWheelTime = 0;
  let isAnimating = false;

  // --- Transformation State ---
  let baseScale = 1;
  let displayScale = 1;
  let offsetX = 0;
  let offsetY = 0;
  let startX = 0;
  let startY = 0;
  let currentRotation = 0;

  // --- Lifecycle ---
  let animationFrameId: number;
  let interactionTimeoutId: ReturnType<typeof setTimeout> | null = null;

  /**
   * Returns the dimensions of the rotated image.
   *
   * If the image is rotated by 90 or 270 degrees, the width and height
   * of the image are swapped.
   *
   * @returns { width: number; height: number } An object with the width and height of the rotated image.
   */
  const getRotatedImageDimensions = (): { width: number; height: number } => {
    if (!image) return { width: 0, height: 0 };
    const isSideways = currentRotation === 90 || currentRotation === 270;
    return {
      width: isSideways ? image.naturalHeight : image.naturalWidth,
      height: isSideways ? image.naturalWidth : image.naturalHeight,
    };
  };

  /**
   * Debounces the visibility of the edge indicators.
   *
   * If the edge indicators are enabled, this function will set the
   * visibility of the indicators to true and then set it back to false
   * after a timeout of 100ms.
   *
   * The function will only run if the edge indicators are enabled.
   */
  const debounceIndicatorVisibility = () => {
    const timeoutInMs = 100;
    if (!get(appConfig).edgeIndicatorsVisible) return;

    if (interactionTimeoutId) clearTimeout(interactionTimeoutId);

    indicatorsVisible.set(true);
    interactionTimeoutId = setTimeout(() => {
      indicatorsVisible.set(false);
    }, timeoutInMs);
  };

  /**
   * Draws the current image on the canvas.
   *
   * This function is responsible for updating the canvas image when the user
   * interacts with the image (e.g. zooms or pans the image).
   *
   * If the user is currently interacting with the image, the function will also
   * trigger the edge indicators to be visible, if applicable.
   *
   * If the image is not fully loaded, the function will return early.
   */
  const draw = () => {
    animationFrameId = requestAnimationFrame(draw);
    if (!ctx || !canvas) return;

    const isInteracting = isDragging || isAnimating || Date.now() - lastWheelTime < 100;
    if (isInteracting) debounceIndicatorVisibility();

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    if (image && image.complete) {
      ctx.save();
      const w = image.naturalWidth;
      const h = image.naturalHeight;

      // Translate to the final position
      ctx.translate(offsetX, offsetY);

      // Then rotate around the center of the unscaled, unrotated image
      if (currentRotation > 0) {
        const rad = (currentRotation * Math.PI) / 180;
        ctx.rotate(rad);
      }

      // Scale relative to the center
      ctx.scale(displayScale, displayScale);

      // And finally draw the image centered at the origin
      ctx.drawImage(image, -w / 2, -h / 2);

      ctx.restore();
    }

    updateEdgeIndicators();
  };

  /**
   * Updates the edge indicators with the current image offset and dimensions.
   *
   * If the image offset is negative or exceeds the canvas dimensions, the
   * corresponding edge indicator will be set to true.
   */
  const updateEdgeIndicators = () => {
    if (!image || !canvas) return;

    const { width: rotatedWidth, height: rotatedHeight } = getRotatedImageDimensions();
    const imageWidth = rotatedWidth * displayScale;
    const imageHeight = rotatedHeight * displayScale;

    const imageLeft = offsetX - imageWidth / 2;
    const imageRight = offsetX + imageWidth / 2;
    const imageTop = offsetY - imageHeight / 2;
    const imageBottom = offsetY + imageHeight / 2;

    edgeIndicators.update((indicators) => ({
      ...indicators,
      left: imageLeft < 0,
      right: imageRight > canvas.width,
      top: imageTop < 0,
      bottom: imageBottom > canvas.height,
    }));
  };

  /**
   * Sets the initial transformation of the image on the canvas.
   *
   * The initial transformation is calculated based on the aspect ratio of the canvas and the image.
   * The image is scaled to fit within the canvas while maintaining its aspect ratio.
   * The offset of the image is then calculated based on the scaled image dimensions and the canvas dimensions.
   * The zoom level is set to 1.
   * The edge indicators are updated based on the new image offset and dimensions.
   */
  const setInitialTransform = () => {
    if (!image || !image.complete || !canvas || canvas.width === 0) return;

    const { width: rotatedWidth, height: rotatedHeight } = getRotatedImageDimensions();

    const canvasAspect = canvas.width / canvas.height;
    const imageAspect = rotatedWidth / rotatedHeight;

    if (imageAspect > canvasAspect) {
      baseScale = canvas.width / rotatedWidth;
    } else {
      baseScale = canvas.height / rotatedHeight;
    }
    displayScale = baseScale;

    offsetX = canvas.width / 2;
    offsetY = canvas.height / 2;

    zoomLevelStore.set(1);
    updateEdgeIndicators();
  };

  /**
   * Animates the zoom level to the given target zoom level.
   *
   * The animation is done by smoothly changing the display scale of the image over a given duration.
   * The offset of the image is also updated based on the new display scale and the canvas dimensions.
   * The edge indicators are updated based on the new image offset and dimensions.
   *
   * If the function is called while an animation is already in progress, it will return immediately without doing anything.
   *
   * @param {number} targetZoomLevel - The target zoom level of the animation.
   */
  const animateZoomTo = (targetZoomLevel: number) => {
    if (isAnimating) return;
    isAnimating = true;

    const targetDisplayScale = baseScale * targetZoomLevel;
    const startDisplayScale = displayScale;
    const duration = 150; // duration in ms
    let startTime: number | null = null;

    const frame = (time: number) => {
      if (startTime === null) startTime = time;
      const elapsed = time - startTime;
      const progress = Math.min(elapsed / duration, 1);

      const newDisplayScale =
        startDisplayScale + (targetDisplayScale - startDisplayScale) * progress;

      const centerX = canvas.width / 2;
      const centerY = canvas.height / 2;
      const worldX = (centerX - offsetX) / displayScale;
      const worldY = (centerY - offsetY) / displayScale;

      offsetX = centerX - worldX * newDisplayScale;
      offsetY = centerY - worldY * newDisplayScale;
      displayScale = newDisplayScale;

      if (progress < 1) {
        requestAnimationFrame(frame);
      } else {
        displayScale = targetDisplayScale;
        zoomLevelStore.set(targetZoomLevel);
        isAnimating = false;
      }
      updateEdgeIndicators();
    };
    requestAnimationFrame(frame);
  };

  /**
   * Handle mouse down event on the image.
   * Start dragging the image when mouse is down.
   * @param {MouseEvent} event - The mouse down event.
   */
  const onMouseDown = (event: MouseEvent) => {
    if (!image) return;
    isAnimating = false;
    isDragging = true;
    startX = event.clientX - offsetX;
    startY = event.clientY - offsetY;
  };

  /**
   * Handles mouse move event on the image.
   * Updates the offset of the image when the mouse is dragged.
   * @param {MouseEvent} event - The mouse move event.
   */
  const onMouseMove = (event: MouseEvent) => {
    if (isDragging) {
      offsetX = event.clientX - startX;
      offsetY = event.clientY - startY;
    }
  };

  /**
   * Handles mouse up event on the image.
   * Sets isDragging to false when the mouse is released.
   */
  const onMouseUp = () => {
    isDragging = false;
  };

  /**
   * Gets the speed of the wheel zoom based on the current zoom modifier state.
   * If the zoom modifier up is active, returns 0.003.
   * If the zoom modifier down is active, returns 0.0002.
   * Otherwise returns 0.001.
   * @returns {number} The speed of the wheel zoom.
   */
  const getWheelZoomSpeed = (): number => {
    if (get(isZoomModifierUpActive)) {
      return 0.003;
    }
    if (get(isZoomModifierDownActive)) {
      return 0.0002;
    }
    return 0.001;
  };

  const onWheel = (event: WheelEvent) => {
    if (!image) return;
    event.preventDefault();
    isAnimating = false;
    lastWheelTime = Date.now();

    triggerWheelZoom(event.deltaY < 0 ? "in" : "out");

    const mouseX = event.offsetX;
    const mouseY = event.offsetY;

    const worldX = (mouseX - offsetX) / displayScale;
    const worldY = (mouseY - offsetY) / displayScale;

    const zoomSpeed = getWheelZoomSpeed();
    const zoomAmount = event.deltaY * zoomSpeed;
    const newDisplayScale = Math.max(
      0.1 * baseScale,
      Math.min(10 * baseScale, displayScale * (1 - zoomAmount))
    );

    offsetX = mouseX - worldX * newDisplayScale;
    offsetY = mouseY - worldY * newDisplayScale;
    displayScale = newDisplayScale;

    zoomLevelStore.set(displayScale / baseScale);
  };

  // --- Setup & Teardown ---
  container.addEventListener("mousedown", onMouseDown);
  container.addEventListener("mousemove", onMouseMove);
  container.addEventListener("mouseup", onMouseUp);
  container.addEventListener("mouseleave", onMouseUp);
  container.addEventListener("wheel", onWheel);

  const resizeObserver = new ResizeObserver((entries) => {
    if (!ctx || !canvas || !image || !image.complete) return;
    const entry = entries[0];
    if (!entry) return;
    const { width, height } = entry.contentRect;

    if (canvas.width !== width || canvas.height !== height) {
      if (get(isRefittingOnResize)) {
        canvas.width = width;
        canvas.height = height;
        setInitialTransform();
      } else {
        // Preserve the view by scaling the offset proportionally
        const oldW = canvas.width;
        const oldH = canvas.height;

        canvas.width = width;
        canvas.height = height;

        if (oldW > 0 && oldH > 0) {
          const xRatio = width / oldW;
          const yRatio = height / oldH;
          offsetX *= xRatio;
          offsetY *= yRatio;
        }
      }

      updateEdgeIndicators();
      draw();
    }
  });

  resizeObserver.observe(container);
  if (container) {
    canvas.width = container.clientWidth;
    canvas.height = container.clientHeight;
  }
  draw();

  const unSubImageUrl = imageUrlStore.subscribe((url) => {
    if (url) {
      image = new Image();
      image.src = url;
      image.onload = () => {
        setInitialTransform();
        if (onImageDrawn) onImageDrawn();
      };
    } else {
      image = null;
    }
  });

  const unSubZoomLevel = zoomLevelStore.subscribe((newZoom) => {
    const targetDisplayScale = baseScale * newZoom;
    if (Math.abs(displayScale - targetDisplayScale) > 0.001) {
      if (Date.now() - lastWheelTime > 300) {
        animateZoomTo(newZoom);
      }
    }
  });

  const unSubRotation = rotation.subscribe((r) => {
    if (!image) return;
    const oldRotation = currentRotation;
    currentRotation = r;
    if (oldRotation !== currentRotation) {
      setInitialTransform();
    }
  });

  return {
    /**
     * Destroys the zoompan action and releases all resources.
     */
    destroy() {
      canvas.style.willChange = "auto";
      container.removeEventListener("mousedown", onMouseDown);
      container.removeEventListener("mousemove", onMouseMove);
      container.removeEventListener("mouseup", onMouseUp);
      container.removeEventListener("mouseleave", onMouseUp);
      container.removeEventListener("wheel", onWheel);
      resizeObserver.disconnect();
      cancelAnimationFrame(animationFrameId);
      unSubImageUrl();
      unSubZoomLevel();
      unSubRotation();
    },
  };
};
