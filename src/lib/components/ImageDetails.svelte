<script lang="ts">
  import {
    imageFormat,
    imageResolution,
    imageAspectRatio,
    imageColorDepth,
    imageFileSize,
  } from "$lib/stores";
  import { t } from "$lib/utils/i18n";
  import { prettySize } from "$lib/utils/prettySize";
</script>

<div class="info-container">
  <h1>{$t["image.info"]}</h1>
  <div class="info-grid">
    {#if $imageAspectRatio}
      <div class="info-item">
        <span class="info-tag">{$t["image.aspectRatio"]}</span>
        <span class="info-value">{$imageAspectRatio}</span>
      </div>
    {/if}
    {#if $imageColorDepth}
      <div class="info-item">
        <span class="info-tag">{$t["image.colorDepth"]}</span>
        <span class="info-value">{$imageColorDepth} bit</span>
      </div>
    {/if}
    {#if $imageFileSize}
      <div class="info-item">
        <span class="info-tag">{$t["image.fileSize"]}</span>
        <span class="info-value">{prettySize($imageFileSize)}</span>
      </div>
    {/if}
    {#if $imageFormat}
      <div class="info-item">
        <span class="info-tag">{$t["image.format"]}</span>
        <span class="info-value">{$imageFormat}</span>
      </div>
    {/if}
    {#if $imageResolution}
      <div class="info-item">
        <span class="info-tag">{$t["image.resolution"]}</span>
        <span class="info-value">{$imageResolution.width} x {$imageResolution.height}</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .info-container {
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
    margin-bottom: 0;
  }

  .info-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    justify-content: center;
  }

  .info-item {
    flex: 1 1 10rem;
    display: flex;
    flex-direction: column;
    background-color: var(--color-button);
    padding: 0.75rem;
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.1rem;
    transition: all 0.15s ease;
  }

  .info-tag {
    font-weight: bold;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    margin-bottom: 0.25rem;
    text-align: end;
    word-break: break-word;
  }

  .info-value {
    font-size: 1rem;
    color: var(--color-text-primary);
    word-break: break-word;
  }
</style>
