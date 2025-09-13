<script lang="ts">
  import { imageUrl, zoomLevel } from "$lib/store";
  import { zoomPan } from "$lib/actions/zoompan";
</script>

<div class="image-view-container">
  {#if $imageUrl}
    <canvas use:zoomPan={{ imageUrlStore: imageUrl, zoomLevelStore: zoomLevel }}></canvas>
  {:else}
    <p>Select an image to view</p>
  {/if}
</div>

<style>
  .image-view-container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-grow: 1;
    min-height: 0;
    max-height: 80%;
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
