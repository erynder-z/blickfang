<script lang="ts">
  import { aiDetectionResult } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
</script>

<div class="ai-detection-container">
  {#if $aiDetectionResult}
    <h1>{$t["ai_checker.heading"]}</h1>
    <div class="ai-detection-grid">
      <div class="ai-detection-item">
        <h1 class="ai-detection-tag">{$t["ai_checker.result_tag"]}</h1>
        <span class="ai-detection-value"
          >{$aiDetectionResult.is_ai_generated
            ? $t["ai_checker.positive_result"]
            : $t["ai_checker.negative_result"]}</span
        >
      </div>
    </div>
  {/if}
</div>

<style>
  .ai-detection-container {
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

  .ai-detection-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .ai-detection-item {
    display: flex;
    flex-direction: column;
    background-color: var(--color-button);
    padding: 0.75rem;
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.1rem;
    transition: all 0.15s ease;
  }

  .ai-detection-tag {
    font-weight: bold;
    font-size: 0.8rem;
    color: var(--color-text-secondary);
    margin-bottom: 0.25rem;
    text-align: end;
    word-break: break-word;
  }

  .ai-detection-value {
    font-size: 1rem;
    color: var(--color-text-primary);
    word-break: break-word;
  }
</style>
