import { zoomLevel, imageUrl } from "$lib/store";
import type { Writable } from "svelte/store";

type ZoomPanOptions = {
  zoomLevelStore: Writable<number>;
  imageUrlStore: Writable<string | null>;
};

export function zoomPan(canvas: HTMLCanvasElement, options: ZoomPanOptions) {
  const { zoomLevelStore, imageUrlStore } = options;
  const container = canvas.parentElement;
  if (!container) return;

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

  // --- Drawing & Transformations ---
  function draw() {
    animationFrameId = requestAnimationFrame(draw);
    if (!ctx || !canvas) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    if (image && image.complete) {
      ctx.save();
      ctx.translate(offsetX, offsetY);
      ctx.scale(displayScale, displayScale);
      ctx.drawImage(image, 0, 0);
      ctx.restore();
    }
  }

  function setInitialTransform() {
    if (!image || !canvas || canvas.width === 0) return;

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
  }

  function animateZoomTo(targetZoomLevel: number) {
    if (isAnimating) return;
    isAnimating = true;

    const targetDisplayScale = baseScale * targetZoomLevel;
    const startDisplayScale = displayScale;
    const duration = 150; // duration in ms
    let startTime: number | null = null;

    function frame(time: number) {
      if (startTime === null) startTime = time;
      const elapsed = time - startTime;
      const progress = Math.min(elapsed / duration, 1);

      const newDisplayScale =
        startDisplayScale + (targetDisplayScale - startDisplayScale) * progress;

      const rect = canvas.getBoundingClientRect();
      const centerX = rect.width / 2;
      const centerY = rect.height / 2;
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
    }
    requestAnimationFrame(frame);
  }

  // --- Event Handlers ---
  function onMouseDown(event: MouseEvent) {
    if (!image) return;
    isAnimating = false;
    isDragging = true;
    startX = event.clientX - offsetX;
    startY = event.clientY - offsetY;
  }

  function onMouseMove(event: MouseEvent) {
    if (isDragging) {
      offsetX = event.clientX - startX;
      offsetY = event.clientY - startY;
    }
  }

  function onMouseUp() {
    isDragging = false;
  }

  function onWheel(event: WheelEvent) {
    if (!image) return;
    event.preventDefault();
    isAnimating = false;
    lastWheelTime = Date.now();

    const rect = canvas.getBoundingClientRect();
    const mouseX = event.clientX - rect.left;
    const mouseY = event.clientY - rect.top;

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
  }

  // --- Setup & Teardown ---
  container.addEventListener("mousedown", onMouseDown);
  container.addEventListener("mousemove", onMouseMove);
  container.addEventListener("mouseup", onMouseUp);
  container.addEventListener("mouseleave", onMouseUp);
  container.addEventListener("wheel", onWheel);

  const resizeObserver = new ResizeObserver((entries) => {
    // Defer the execution to prevent ResizeObserver loop errors
    requestAnimationFrame(() => {
      if (!ctx || !canvas) return;
      const entry = entries[0];
      if (!entry) return;
      const { width, height } = entry.contentRect;

      const oldCanvasWidth = canvas.width;
      const oldCanvasHeight = canvas.height;

      if (oldCanvasWidth !== width || oldCanvasHeight !== height) {
        const worldCenterX = (oldCanvasWidth / 2 - offsetX) / displayScale;
        const worldCenterY = (oldCanvasHeight / 2 - offsetY) / displayScale;

        canvas.width = width;
        canvas.height = height;

        offsetX = width / 2 - worldCenterX * displayScale;
        offsetY = height / 2 - worldCenterY * displayScale;
      }
    });
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
}
