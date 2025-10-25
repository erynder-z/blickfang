<script lang="ts">
  import { imageExif } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";

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
    <div class="empty-exif">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="24px"
        viewBox="0 -960 960 960"
        width="24px"
        fill="currentColor"
        ><path
          d="m792-282-58-56 26-26v-232L596-760H364l-26 26-56-58 48-48h300l210 210v298l-48 50ZM520-552v-128h-80v48l80 80ZM820-28 678-170l-48 50H330L120-332v-298l48-48L28-820l56-56L876-84l-56 56ZM536-536ZM364-200h232l26-26-396-396-26 26v232l164 164Zm116-80q-17 0-28.5-11.5T440-320q0-17 11.5-28.5T480-360q17 0 28.5 11.5T520-320q0 17-11.5 28.5T480-280Zm-56-144Z"
        /></svg
      >

      {$t["exif.no-data"]}
    </div>
  {/if}
</div>

<style>
  .exif-container {
    padding: 1rem;
    color: var(--color-text-primary);
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
    margin-top: 4rem;
  }

  .empty-exif {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    color: var(--color-text-secondary);
    height: 100%;
    text-align: justify;
  }

  svg {
    width: 3rem;
    height: 3rem;
  }
</style>
