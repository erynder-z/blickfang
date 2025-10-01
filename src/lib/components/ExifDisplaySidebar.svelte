<script lang="ts">
  import { imageExif, isExifSidebarVisible } from "$lib/store";
  import { t } from "$lib/i18n";

  let exifData: Record<string, string> | null = null;

  $: {
    if ($imageExif) {
      try {
        exifData = JSON.parse($imageExif);
      } catch (e) {
        console.error("Failed to parse EXIF data:", e);
        exifData = null;
      }
    } else {
      exifData = null;
    }
  }
</script>

<div class="exif-sidebar-overlay" class:visible={$isExifSidebarVisible}>
  <div class="exif-container">
    {#if exifData && Object.keys(exifData).length > 0}
      <h1>{$t["exif.title"]}</h1>
      <div class="exif-grid">
        {#each Object.entries(exifData) as [tag, value]}
          <div class="exif-item">
            <span class="exif-tag">{tag}</span>
            <span class="exif-value">{value}</span>
          </div>
        {/each}
      </div>
    {:else}
      <p>{$t["exif.no-data"]}</p>
    {/if}
  </div>
</div>

<style>
  .exif-sidebar-overlay {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    width: 25%;
    max-width: 33%;
    min-width: 25%;
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

  .exif-container {
    padding: 1rem;
    color: var(--color-text-primary);
    height: 100%;
    overflow-y: auto;
  }

  .exif-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .exif-item {
    flex: 1 1 10rem;
    display: flex;
    flex-direction: column;
    background-color: var(--color-accent);
    padding: 0.75rem;
    border-radius: 4px;
  }

  .exif-tag {
    font-weight: bold;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    margin-bottom: 0.25rem;
    word-break: break-all;
  }

  .exif-value {
    font-size: 1rem;
    color: var(--color-text-primary);
    word-break: break-all;
  }

  p {
    text-align: center;
  }
</style>
