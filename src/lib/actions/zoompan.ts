import { triggerWheelZoom } from "$lib/commands";
import { edgeIndicators, indicatorsVisible } from "$lib/store";
import type { Writable } from "svelte/store";

type ZoomPanOptions = {
  zoomLevelStore: Writable<number>;
  imageUrlStore: Writable<string | null>;
};

export const zoomPan = (canvas: HTMLCanvasElement, options: ZoomPanOptions) => {
  const { zoomLevelStore, imageUrlStore } = options;
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

  // --- Lifecycle ---
  let animationFrameId: number;
  let interactionTimeoutId: ReturnType<typeof setTimeout> | null = null;

  // --- Interaction Handling ---
  const debounceIndicatorVisibility = () => {
    if (interactionTimeoutId) {
      clearTimeout(interactionTimeoutId);
    }
    indicatorsVisible.set(true);
    interactionTimeoutId = setTimeout(() => {
      indicatorsVisible.set(false);
    }, 100);
  };

  // --- Drawing & Transformations ---
  const draw = () => {
    animationFrameId = requestAnimationFrame(draw);
    if (!ctx || !canvas) return;

    // --- Interaction check ---
    const isInteracting = isDragging || isAnimating || Date.now() - lastWheelTime < 100;
    if (isInteracting) debounceIndicatorVisibility();

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    if (image && image.complete) {
      ctx.save();
      ctx.translate(offsetX, offsetY);
      ctx.scale(displayScale, displayScale);
      ctx.drawImage(image, 0, 0);
      ctx.restore();
    }

    updateEdgeIndicators();
  };

  const updateEdgeIndicators = () => {
    if (!image || !canvas) return;

    const imageWidth = image.naturalWidth * displayScale;
    const imageHeight = image.naturalHeight * displayScale;

    edgeIndicators.update((indicators) => ({
      ...indicators,
      left: offsetX < 0,
      right: offsetX + imageWidth > canvas.width,
      top: offsetY < 0,
      bottom: offsetY + imageHeight > canvas.height,
    }));
  };

  const setInitialTransform = () => {
    if (!image || !image.complete || !canvas || canvas.width === 0) return;

    const canvasAspect = canvas.width / canvas.height;
    const imageAspect = image.naturalWidth / image.naturalHeight;

    if (imageAspect > canvasAspect) {
      baseScale = canvas.width / image.naturalWidth;
    } else {
      baseScale = canvas.height / image.naturalHeight;
    }
    displayScale = baseScale;
    offsetX = (canvas.width - image.naturalWidth * displayScale) / 2;
    offsetY = (canvas.height - image.naturalHeight * displayScale) / 2;

    zoomLevelStore.set(1);
    updateEdgeIndicators();
  };

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

  // --- Event Handlers ---
  const onMouseDown = (event: MouseEvent) => {
    if (!image) return;
    isAnimating = false;
    isDragging = true;
    startX = event.clientX - offsetX;
    startY = event.clientY - offsetY;
  };

  const onMouseMove = (event: MouseEvent) => {
    if (isDragging) {
      offsetX = event.clientX - startX;
      offsetY = event.clientY - startY;
    }
  };

  const onMouseUp = () => {
    isDragging = false;
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

    const zoomSpeed = 0.001;
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
      canvas.width = width;
      canvas.height = height;
      setInitialTransform();
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
      image.onload = () => setInitialTransform();
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

  return {
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
    },
  };
};
