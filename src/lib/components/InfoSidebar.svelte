<script lang="ts">
  import { isExifSidebarVisible } from "$lib/stores/appState";
  import ExifDisplay from "./ExifDisplay.svelte";
  import ImageDetails from "./ImageDetails.svelte";
</script>

<div class="exif-sidebar-overlay" class:visible={$isExifSidebarVisible}>
  <div class="sidebar-content">
    <ImageDetails />
    <ExifDisplay />
  </div>
</div>

<style>
  .exif-sidebar-overlay {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    width: 25%;
    max-width: 30ch;
    background-color: var(--color-background);
    transform: translateX(100%);
    transition: transform 0.3s ease-in-out;
    z-index: 20;
    border-left: 0.2rem solid var(--color-outline);
  }

  .exif-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .sidebar-content {
    height: 100%;
    overflow-y: auto;
  }
</style>
