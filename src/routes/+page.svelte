<script lang="ts">
  import ImageView from "../lib/components/ImageView.svelte";
  import Controls from "../lib/components/Controls.svelte";
  import ImageNameDisplay from "$lib/components/ImageNameDisplay.svelte";
  import { isExifSidebarVisible, isOptionsSidebarVisible } from "$lib/store";
  import ExifDisplaySidebar from "$lib/components/ExifDisplaySidebar.svelte";
  import OptionsDisplaySidebar from "$lib/components/OptionsDisplaySidebar.svelte";
</script>

<main>
  <div class="image-section">
    <ImageView />
    <Controls />
    <ImageNameDisplay />
  </div>
  <div class="exif-sidebar-overlay" class:visible={$isExifSidebarVisible}>
    <ExifDisplaySidebar />
  </div>
  <div class="options-sidebar-overlay" class:visible={$isOptionsSidebarVisible}>
    <OptionsDisplaySidebar />
  </div>
</main>

<style>
  main {
    height: 100vh;
    position: relative;
    overflow: hidden;
  }

  .image-section {
    position: relative;
    height: 100%;
    width: 100%;
  }

  .exif-sidebar-overlay {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    width: 25%;
    max-width: 350px;
    min-width: 250px;
    background-color: #372e49;
    transform: translateX(100%);
    transition: transform 0.3s ease-in-out;
    z-index: 20;
    box-shadow: -2px 0 5px rgba(0, 0, 0, 0.5);
    border-left: 1px solid #444;
  }

  .options-sidebar-overlay {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 25%;
    max-width: 350px;
    min-width: 250px;
    background-color: #372e49;
    transform: translateX(-100%);
    transition: transform 0.3s ease-in-out;
    z-index: 20;
    box-shadow: -2px 0 5px rgba(0, 0, 0, 0.5);
    border-left: 1px solid #444;
  }

  .exif-sidebar-overlay.visible,
  .options-sidebar-overlay.visible {
    transform: translateX(0);
  }

  :global(*, *:before, *:after) {
    box-sizing: border-box;
  }

  :global(:root) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #747474;
    background-color: #241f2e;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  :global(body) {
    margin: 0;
  }
</style>
