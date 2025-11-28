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
import type { ViewportOptions } from "$lib/types/viewport";
import { get } from "svelte/store";
import type { Unsubscriber } from "svelte/store";

class ImageViewport {
  private canvas: HTMLCanvasElement;
  private container: HTMLElement;
  private ctx: CanvasRenderingContext2D;
  private options: ViewportOptions;

  // --- State ---
  private image: HTMLImageElement | null = null;
  private isDragging = false;
  private lastWheelTime = 0;
  private isAnimating = false;

  // --- Transformation State ---
  private baseScale = 1;
  private displayScale = 1;
  private offsetX = 0;
  private offsetY = 0;
  private startX = 0;
  private startY = 0;
  private currentRotation = 0;

  // --- Lifecycle ---
  private animationFrameId: number;
  private interactionTimeoutId: ReturnType<typeof setTimeout> | null = null;
  private unsubscribers: Unsubscriber[] = [];

  constructor(canvas: HTMLCanvasElement, options: ViewportOptions) {
    this.canvas = canvas;
    this.options = options;
    this.container = canvas.parentElement as HTMLElement;
    this.ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

    this.canvas.style.willChange = "transform";

    this.setupSubscriptions();
    this.setupEventListeners();

    if (this.container) {
      this.canvas.width = this.container.clientWidth;
      this.canvas.height = this.container.clientHeight;
    }

    this.animationFrameId = requestAnimationFrame(() => this.draw());
  }

  private setupSubscriptions() {
    const { zoomLevelStore, imageUrlStore } = this.options;

    const unSubImageUrl = imageUrlStore.subscribe((url) => {
      if (url) {
        this.image = new Image();
        this.image.src = url;
        this.image.onload = () => {
          this.setInitialTransform();
          if (this.options.onImageDrawn) this.options.onImageDrawn();
        };
      } else {
        this.image = null;
      }
    });

    const unSubZoomLevel = zoomLevelStore.subscribe((newZoom) => {
      const targetDisplayScale = this.baseScale * newZoom;
      if (Math.abs(this.displayScale - targetDisplayScale) > 0.001) {
        if (Date.now() - this.lastWheelTime > 300) {
          this.animateZoomTo(newZoom);
        }
      }
    });

    const unSubRotation = rotation.subscribe((r) => {
      if (!this.image) return;
      const oldRotation = this.currentRotation;
      this.currentRotation = r;
      if (oldRotation !== this.currentRotation) {
        this.setInitialTransform();
      }
    });

    this.unsubscribers.push(unSubImageUrl, unSubZoomLevel, unSubRotation);
  }

  private setupEventListeners() {
    this.container.addEventListener("mousedown", this.onMouseDown);
    this.container.addEventListener("mousemove", this.onMouseMove);
    this.container.addEventListener("mouseup", this.onMouseUp);
    this.container.addEventListener("mouseleave", this.onMouseUp);
    this.container.addEventListener("wheel", this.onWheel);

    const resizeObserver = new ResizeObserver((entries) => {
      if (!this.ctx || !this.canvas || !this.image || !this.image.complete) return;
      const entry = entries[0];
      if (!entry) return;
      const { width, height } = entry.contentRect;

      if (this.canvas.width !== width || this.canvas.height !== height) {
        if (get(isRefittingOnResize)) {
          this.canvas.width = width;
          this.canvas.height = height;
          this.setInitialTransform();
        } else {
          // Preserve the view by scaling the offset proportionally
          const oldW = this.canvas.width;
          const oldH = this.canvas.height;

          this.canvas.width = width;
          this.canvas.height = height;

          if (oldW > 0 && oldH > 0) {
            const xRatio = width / oldW;
            const yRatio = height / oldH;
            this.offsetX *= xRatio;
            this.offsetY *= yRatio;
          }
        }

        this.updateEdgeIndicators();
        this.draw();
      }
    });
    resizeObserver.observe(this.container);
    this.unsubscribers.push(() => resizeObserver.disconnect());
  }

  /**
   * Returns the dimensions of the rotated image.
   * If the image is rotated by 90 or 270 degrees, the width and height are swapped.
   */
  private getRotatedImageDimensions = (): { width: number; height: number } => {
    if (!this.image) return { width: 0, height: 0 };
    const isSideways = this.currentRotation === 90 || this.currentRotation === 270;
    return {
      width: isSideways ? this.image.naturalHeight : this.image.naturalWidth,
      height: isSideways ? this.image.naturalWidth : this.image.naturalHeight,
    };
  };

  /**
   * Shows edge indicators for a short duration if they are enabled.
   */
  private debounceIndicatorVisibility = () => {
    const timeoutInMs = 100;
    if (!get(appConfig).edgeIndicatorsVisible) return;

    if (this.interactionTimeoutId) clearTimeout(this.interactionTimeoutId);

    indicatorsVisible.set(true);
    this.interactionTimeoutId = setTimeout(() => {
      indicatorsVisible.set(false);
    }, timeoutInMs);
  };

  /**
   * The main rendering loop. Clears the canvas and draws the image with all transformations.
   */
  private draw = () => {
    this.animationFrameId = requestAnimationFrame(this.draw);

    const isInteracting =
      this.isDragging || this.isAnimating || Date.now() - this.lastWheelTime < 100;
    if (isInteracting) this.debounceIndicatorVisibility();

    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    if (this.image && this.image.complete) {
      this.ctx.save();
      const w = this.image.naturalWidth;
      const h = this.image.naturalHeight;

      // 1. Translate to the center of the canvas where the image will be panned to.
      this.ctx.translate(this.offsetX, this.offsetY);

      // 2. Rotate the context around this new origin.
      if (this.currentRotation > 0) {
        const rad = (this.currentRotation * Math.PI) / 180;
        this.ctx.rotate(rad);
      }

      // 3. Scale the context from the center.
      this.ctx.scale(this.displayScale, this.displayScale);

      // 4. Draw the image centered at the origin (0,0), which is now translated, rotated, and scaled.
      this.ctx.drawImage(this.image, -w / 2, -h / 2);

      this.ctx.restore();
    }

    this.updateEdgeIndicators();
  };

  /**
   * Updates the visibility of the edge indicators based on whether the image is overflowing the canvas.
   */
  private updateEdgeIndicators = () => {
    if (!this.image) return;

    const { width: rotatedWidth, height: rotatedHeight } = this.getRotatedImageDimensions();
    const imageWidth = rotatedWidth * this.displayScale;
    const imageHeight = rotatedHeight * this.displayScale;

    const imageLeft = this.offsetX - imageWidth / 2;
    const imageRight = this.offsetX + imageWidth / 2;
    const imageTop = this.offsetY - imageHeight / 2;
    const imageBottom = this.offsetY + imageHeight / 2;

    edgeIndicators.update((indicators) => ({
      ...indicators,
      left: imageLeft < 0,
      right: imageRight > this.canvas.width,
      top: imageTop < 0,
      bottom: imageBottom > this.canvas.height,
    }));
  };

  /**
   * Calculates and applies the initial transformation to fit the image onto the canvas.
   */
  private setInitialTransform = () => {
    if (!this.image || !this.image.complete || this.canvas.width === 0) return;

    const { width: rotatedWidth, height: rotatedHeight } = this.getRotatedImageDimensions();
    const canvasAspect = this.canvas.width / this.canvas.height;
    const imageAspect = rotatedWidth / rotatedHeight;

    if (imageAspect > canvasAspect) {
      this.baseScale = this.canvas.width / rotatedWidth;
    } else {
      this.baseScale = this.canvas.height / rotatedHeight;
    }
    this.displayScale = this.baseScale;
    this.offsetX = this.canvas.width / 2;
    this.offsetY = this.canvas.height / 2;

    this.options.zoomLevelStore.set(1);
    this.updateEdgeIndicators();
  };

  /**
   * Smoothly animates the zoom level to a target value.
   */
  private animateZoomTo = (targetZoomLevel: number) => {
    if (this.isAnimating) return;
    this.isAnimating = true;

    const targetDisplayScale = this.baseScale * targetZoomLevel;
    const startDisplayScale = this.displayScale;
    const duration = 150; // ms
    let startTime: number | null = null;

    const frame = (time: number) => {
      if (startTime === null) startTime = time;
      const elapsed = time - startTime;
      const progress = Math.min(elapsed / duration, 1);

      const newDisplayScale =
        startDisplayScale + (targetDisplayScale - startDisplayScale) * progress;
      const centerX = this.canvas.width / 2;
      const centerY = this.canvas.height / 2;
      const worldX = (centerX - this.offsetX) / this.displayScale;
      const worldY = (centerY - this.offsetY) / this.displayScale;

      this.offsetX = centerX - worldX * newDisplayScale;
      this.offsetY = centerY - worldY * newDisplayScale;
      this.displayScale = newDisplayScale;

      if (progress < 1) {
        requestAnimationFrame(frame);
      } else {
        this.displayScale = targetDisplayScale;
        this.options.zoomLevelStore.set(targetZoomLevel);
        this.isAnimating = false;
      }
      this.updateEdgeIndicators();
    };
    requestAnimationFrame(frame);
  };

  // --- Event Handlers ---
  private onMouseDown = (event: MouseEvent) => {
    if (!this.image) return;
    this.isAnimating = false;
    this.isDragging = true;
    this.startX = event.clientX - this.offsetX;
    this.startY = event.clientY - this.offsetY;
  };

  private onMouseMove = (event: MouseEvent) => {
    if (this.isDragging) {
      this.offsetX = event.clientX - this.startX;
      this.offsetY = event.clientY - this.startY;
    }
  };

  private onMouseUp = () => {
    this.isDragging = false;
  };

  private getWheelZoomSpeed = (): number => {
    if (get(isZoomModifierUpActive)) return 0.003;
    if (get(isZoomModifierDownActive)) return 0.0002;
    return 0.001;
  };

  private onWheel = (event: WheelEvent) => {
    if (!this.image) return;
    event.preventDefault();
    this.isAnimating = false;
    this.lastWheelTime = Date.now();

    triggerWheelZoom(event.deltaY < 0 ? "in" : "out");

    const mouseX = event.offsetX;
    const mouseY = event.offsetY;
    const worldX = (mouseX - this.offsetX) / this.displayScale;
    const worldY = (mouseY - this.offsetY) / this.displayScale;
    const zoomSpeed = this.getWheelZoomSpeed();
    const zoomAmount = event.deltaY * zoomSpeed;
    const newDisplayScale = Math.max(
      0.1 * this.baseScale,
      Math.min(10 * this.baseScale, this.displayScale * (1 - zoomAmount))
    );

    this.offsetX = mouseX - worldX * newDisplayScale;
    this.offsetY = mouseY - worldY * newDisplayScale;
    this.displayScale = newDisplayScale;

    this.options.zoomLevelStore.set(this.displayScale / this.baseScale);
  };

  /**
   * Cleans up all event listeners and subscriptions.
   */
  public destroy() {
    this.canvas.style.willChange = "auto";
    this.container.removeEventListener("mousedown", this.onMouseDown);
    this.container.removeEventListener("mousemove", this.onMouseMove);
    this.container.removeEventListener("mouseup", this.onMouseUp);
    this.container.removeEventListener("mouseleave", this.onMouseUp);
    this.container.removeEventListener("wheel", this.onWheel);

    cancelAnimationFrame(this.animationFrameId);
    this.unsubscribers.forEach((unsub) => unsub());
  }
}

/**
 * A Svelte action that attaches zoom, pan, and rotate functionality to a canvas element.
 */
export const imageViewport = (canvas: HTMLCanvasElement, options: ViewportOptions) => {
  const viewport = new ImageViewport(canvas, options);
  return {
    destroy: () => {
      viewport.destroy();
    },
  };
};
