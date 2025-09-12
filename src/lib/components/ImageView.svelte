<script lang="ts">
  import { imageUrl, zoomLevel } from "$lib/store";
</script>

<div class="image-view-container">
  {#if $imageUrl}
    <img src={$imageUrl} alt="Selected" style="transform: scale({$zoomLevel});" />
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
  }
  img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    transition: transform 0.2s ease-in-out;
  }
</style>
