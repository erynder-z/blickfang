<script lang="ts">
  import { imageExif } from "$lib/store";

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
    <h1>Exif Data</h1>
    <div class="exif-grid">
      {#each Object.entries(exifData) as [tag, value]}
        <div class="exif-item">
          <span class="exif-tag">{tag}</span>
          <span class="exif-value">{value}</span>
        </div>
      {/each}
    </div>
  {:else}
    <p>No Exif data!</p>
  {/if}
</div>

<style>
  .exif-container {
    padding: 1rem;
    color: #e3e3e3;
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
    background-color: rgba(0, 0, 0, 0.2);
    padding: 0.75rem;
    border-radius: 4px;
  }

  .exif-tag {
    font-weight: bold;
    font-size: 0.9rem;
    color: #aaa;
    margin-bottom: 0.25rem;
    word-break: break-all;
  }

  .exif-value {
    font-size: 1rem;
    word-break: break-all;
  }

  p {
    text-align: center;
  }
</style>
