<script lang="ts">
  import { imageUrl, zoomLevel } from "$lib/store";
  import { zoomPan } from "$lib/actions/zoompan";
  import EdgeIndicators from "./EdgeIndicators.svelte";
  import { t } from "$lib/i18n";
  import { openFile } from "$lib/commands";
</script>

<div class="image-view-container">
  <EdgeIndicators />
  {#if $imageUrl}
    <canvas use:zoomPan={{ imageUrlStore: imageUrl, zoomLevelStore: zoomLevel }}></canvas>
  {:else}
    <div class="empty-canvas">
      <button on:click={openFile}>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="icon">
          <path
            d="M21 19V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2Zm-2 0H5V5h14v14ZM8.9 13.98l2.1 2.52 3-3.76 3.9 5.26H6l2.9-4.02Z"
            fill="currentColor"
          />
        </svg>

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
  }
  .image-view-container:active {
    cursor: grabbing;
  }
  canvas {
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
  }

  svg {
    width: 3rem;
    height: 3rem;
  }
</style>
