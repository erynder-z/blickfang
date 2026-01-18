<script lang="ts">
  import { isInfoSidebarVisible, imageFormat } from "$lib/stores";
  import ExifDisplay from "$lib/components/ExifDisplay.svelte";
  import AiDetection from "$lib/components/AiDetection.svelte";
  import ImageDetails from "$lib/components/ImageDetails.svelte";
  import InfoSidebarPlaceholder from "$lib/components/InfoSidebarPlaceholder.svelte";
</script>

<div class="info-sidebar-overlay" class:visible={$isInfoSidebarVisible}>
  <div class="sidebar-content">
    {#if $imageFormat}
      <ImageDetails />
      <ExifDisplay />
      <AiDetection />
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
    opacity: 0.9;
  }

  .info-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .sidebar-content {
    height: 100%;
    overflow-y: auto;
  }
</style>
