<script lang="ts">
  import { imageFormat, imageResolution, imageAspectRatio } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
</script>

<div class="info-container">
  <h1>{$t["image.info"]}</h1>
  <div class="info-grid">
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
    {#if $imageAspectRatio}
      <div class="info-item">
        <span class="info-tag">{$t["image.aspectRatio"]}</span>
        <span class="info-value">{$imageAspectRatio}</span>
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
    border-radius: 0.5rem;
    box-shadow:
      0.2rem 0.2rem 0 0 var(--color-outline),
      0.4rem 0.4rem 0 0 var(--color-shadow);
    transition: all 0.15s ease;
  }

  .info-item:hover {
    transform: translateY(-1px);
    box-shadow:
      0.3rem 0.3rem 0 0 var(--color-outline),
      0.5rem 0.5rem 0 0 var(--color-shadow);
  }

  .info-item:active {
    transform: translateY(1px);
    box-shadow: inset 0.1rem 0.1rem 0 0 var(--color-outline);
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
