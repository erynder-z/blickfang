<script lang="ts">
  import { imageUrl, zoomLevel } from "$lib/store";
  import Placeholder from "./Placeholder.svelte";
  import { zoomPan } from "$lib/actions/zoompan";
  import EdgeIndicators from "./EdgeIndicators.svelte";

  let resizing = false;
</script>

<div class="image-view-container">
  <EdgeIndicators />
  {#if $imageUrl}
    <canvas
      use:zoomPan={{ imageUrlStore: imageUrl, zoomLevelStore: zoomLevel }}
      on:resizing={(e) => (resizing = e.detail)}
    ></canvas>
    <Placeholder {resizing} />
  {:else}
    <p>Select an image to view</p>
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
    cursor: grab;
  }
  .image-view-container:active {
    cursor: grabbing;
  }
  canvas {
    width: 100%;
    height: 100%;
  }
</style>
