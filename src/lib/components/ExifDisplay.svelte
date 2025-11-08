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
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  h1 {
    text-align: center;
    font-size: 1.5rem;
    color: var(--color-text-primary);
    line-height: 1.2;
  }

  .exif-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .exif-item {
    display: flex;
    flex-direction: column;
    background-color: var(--color-button);
    padding: 0.75rem;
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.5rem;
    box-shadow:
      0.2rem 0.2rem 0 0 var(--color-outline),
      0.4rem 0.4rem 0 0 var(--color-shadow);
    transition: all 0.15s ease;
  }

  .exif-item:hover {
    transform: translateY(-1px);
    box-shadow:
      0.3rem 0.3rem 0 0 var(--color-outline),
      0.5rem 0.5rem 0 0 var(--color-shadow);
  }

  .exif-item:active {
    transform: translateY(1px);
    box-shadow: inset 0.1rem 0.1rem 0 0 var(--color-outline);
  }

  .exif-tag {
    font-weight: bold;
    font-size: 0.8rem;
    color: var(--color-text-secondary);
    margin-bottom: 0.25rem;
    text-align: end;
    word-break: break-word;
  }

  .exif-value {
    font-size: 1rem;
    color: var(--color-text-primary);
    word-break: break-word;
  }

  .empty-exif {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    color: var(--color-text-secondary);
    height: 100%;
    text-align: center;
  }

  .empty-exif svg {
    width: 3rem;
    height: 3rem;
    fill: var(--color-text-secondary);
  }
</style>
