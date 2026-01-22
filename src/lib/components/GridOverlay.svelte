<script lang="ts">
  import { appConfig, isGridOverlayVisible, imageTransform } from "$lib/stores";
  import { hexToRgba } from "$lib/utils/hexToRgba";
  import { onMount, onDestroy } from "svelte";

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let resizeObserver: ResizeObserver;
  let color: string = $appConfig.gridColor || "#000000";

  onMount(() => {
    if (canvas) {
      ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
      resizeObserver = new ResizeObserver(() => {
        if ($isGridOverlayVisible) {
          drawGridOverlay();
        }
      });
      resizeObserver.observe(canvas);
    }
  });

  onDestroy(() => {
    resizeObserver?.disconnect();
  });

  $: if ($isGridOverlayVisible && canvas && ctx && $imageTransform) {
    const { renderedWidth, renderedHeight, offsetX, offsetY } = $imageTransform;

    if (renderedWidth === 0 || renderedHeight === 0) {
      // Skip drawing if dimensions are zero
    } else {
      const imageLeft = offsetX - renderedWidth / 2;
      const imageTop = offsetY - renderedHeight / 2;

      drawGridOverlay();
    }
  }

  const drawGridOverlay = () => {
    if (!canvas || !ctx || !$imageTransform) return;

    const { renderedWidth, renderedHeight, offsetX, offsetY } = $imageTransform;

    if (renderedWidth === 0 || renderedHeight === 0) return;

    // Setup canvas
    const canvasWidth = canvas.clientWidth;
    const canvasHeight = canvas.clientHeight;

    if (canvas.width !== canvasWidth || canvas.height !== canvasHeight) {
      canvas.width = canvasWidth;
      canvas.height = canvasHeight;
    }

    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    // Draw grid
    const imageLeft = offsetX - renderedWidth / 2;
    const imageTop = offsetY - renderedHeight / 2;
    const defaultColor = "#000000";
    const defaultStrength = 2;

    ctx.strokeStyle = hexToRgba($appConfig.gridColor || defaultColor, 0.5);
    ctx.lineWidth = $appConfig.gridLineStrength || defaultStrength;
    ctx.beginPath();

    switch ($appConfig.gridOverlayMode) {
      case "golden-ratio":
        drawGoldenRatioGrid(imageLeft, imageTop, renderedWidth, renderedHeight);
        break;
      case "rule-of-thirds":
        drawRuleOfThirdsGrid(imageLeft, imageTop, renderedWidth, renderedHeight);
        break;
      case "grid":
        drawRegularGrid(imageLeft, imageTop, renderedWidth, renderedHeight);
        break;
    }

    ctx.stroke();
  };

  const drawGoldenRatioGrid = (left: number, top: number, width: number, height: number) => {
    const goldenRatio = 0.618;

    // Vertical lines
    ctx.moveTo(left + width * goldenRatio, top);
    ctx.lineTo(left + width * goldenRatio, top + height);
    ctx.moveTo(left + width * (1 - goldenRatio), top);
    ctx.lineTo(left + width * (1 - goldenRatio), top + height);

    // Horizontal lines
    ctx.moveTo(left, top + height * goldenRatio);
    ctx.lineTo(left + width, top + height * goldenRatio);
    ctx.moveTo(left, top + height * (1 - goldenRatio));
    ctx.lineTo(left + width, top + height * (1 - goldenRatio));
  };

  const drawRuleOfThirdsGrid = (left: number, top: number, width: number, height: number) => {
    const thirdWidth = width / 3;
    const thirdHeight = height / 3;

    // Vertical lines
    ctx.moveTo(left + thirdWidth, top);
    ctx.lineTo(left + thirdWidth, top + height);
    ctx.moveTo(left + thirdWidth * 2, top);
    ctx.lineTo(left + thirdWidth * 2, top + height);

    // Horizontal lines
    ctx.moveTo(left, top + thirdHeight);
    ctx.lineTo(left + width, top + thirdHeight);
    ctx.moveTo(left, top + thirdHeight * 2);
    ctx.lineTo(left + width, top + thirdHeight * 2);
  };

  const drawRegularGrid = (left: number, top: number, width: number, height: number) => {
    const cols = 4;
    const rows = 4;
    const colWidth = width / cols;
    const rowHeight = height / rows;

    // Vertical lines
    for (let i = 1; i < cols; i++) {
      ctx.moveTo(left + colWidth * i, top);
      ctx.lineTo(left + colWidth * i, top + height);
    }

    // Horizontal lines
    for (let i = 1; i < rows; i++) {
      ctx.moveTo(left, top + rowHeight * i);
      ctx.lineTo(left + width, top + rowHeight * i);
    }
  };
</script>

<canvas
  bind:this={canvas}
  class="grid-overlay"
  style:display={$isGridOverlayVisible ? "block" : "none"}
></canvas>

<style>
  .grid-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 5;
  }
</style>
