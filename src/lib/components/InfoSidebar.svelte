<script lang="ts">
  import { isInfoSidebarVisible, imageFormat } from "$lib/stores/appState";
  import ExifDisplay from "./ExifDisplay.svelte";
  import ImageDetails from "./ImageDetails.svelte";
  import InfoSidebarPlaceholder from "./InfoSidebarPlaceholder.svelte";
</script>

<div class="info-sidebar-overlay" class:visible={$isInfoSidebarVisible}>
  <div class="sidebar-content">
    {#if $imageFormat}
      <ImageDetails />
      <ExifDisplay />
    {:else}
      <InfoSidebarPlaceholder />
    {/if}
  </div>
</div>

<style>
  .info-sidebar-overlay {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    width: 25%;
    max-width: 30ch;
    background-color: var(--color-background);
    transform: translateX(100%);
    transition: transform 0.2s ease-in-out;
    z-index: 20;
    border-left: 0.2rem solid var(--color-outline);
  }

  .info-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .sidebar-content {
    height: 100%;
    overflow-y: auto;
  }
</style>
