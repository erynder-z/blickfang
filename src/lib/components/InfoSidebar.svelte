<script lang="ts">
  import { isExifSidebarVisible } from "$lib/store";
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
    box-shadow: -2px 0 5px rgba(0, 0, 0, 0.5);
    border-left: 1px solid var(--color-accent);
  }

  .exif-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .sidebar-content {
    height: 100%;
    overflow-y: auto;
  }
</style>
