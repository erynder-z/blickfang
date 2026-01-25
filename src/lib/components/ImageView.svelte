<script lang="ts">
  import { imageUrl, zoomLevel, isZenModeActive } from "$lib/stores";
  import { imageViewport } from "$lib/actions/imageViewport";
  import EdgeIndicators from "$lib/components/EdgeIndicators.svelte";
  import GridOverlay from "$lib/components/GridOverlay.svelte";
  import { t } from "$lib/utils/i18n";
  import { openFile } from "$lib/core/commands";
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";

  let showSvg = false;
  let canvasOpacity = 0;

  onMount(() => {
    showSvg = true;
  });

  $: if ($imageUrl) {
    canvasOpacity = 0;
  }
</script>

<div class="image-view-container" style={$isZenModeActive ? "background-color: black;" : ""}>
  <EdgeIndicators />
  {#if $imageUrl}
    {#key $imageUrl}
      <div class="image-wrapper">
        <canvas
          use:imageViewport={{
            imageUrlStore: imageUrl,
            zoomLevelStore: zoomLevel,
            onImageDrawn: () => (canvasOpacity = 1),
          }}
          style="opacity: {canvasOpacity}; transition: opacity 150ms linear;"
        ></canvas>
        <GridOverlay />
      </div>
    {/key}
  {:else}
    <div class="empty-canvas">
      <button on:click={openFile}>
        {#if showSvg}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            class="icon"
            in:fade={{ duration: 100 }}
          >
            <path
              d="M21 19V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2Zm-2 0H5V5h14v14ZM8.9 13.98l2.1 2.52 3-3.76 3.9 5.26H6l2.9-4.02Z"
              fill="currentColor"
            />
          </svg>
        {/if}

        {$t["image-view.placeholder"]}
      </button>
    </div>
  {/if}
</div>

<style>
  .image-view-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background-color: var(--color-background);
  }

  .image-view-container:active {
    cursor: grabbing;
  }

  .image-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .image-wrapper canvas {
    width: 100%;
    height: 100%;
  }

  .empty-canvas button {
    background: none;
    outline: none;
    border: none;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    color: var(--color-text-secondary);
    font: inherit;
    cursor: pointer;
    text-shadow: 1px 1px 2px var(--color-shadow);
  }

  svg {
    width: 3rem;
    height: 3rem;
  }
</style>
