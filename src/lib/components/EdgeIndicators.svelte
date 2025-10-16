<script lang="ts">
  import { edgeIndicators, indicatorsVisible } from "$lib/store";
</script>

<div
  class="edge-indicators"
  class:visible={$indicatorsVisible}
  class:show-top={$edgeIndicators.top}
  class:show-bottom={$edgeIndicators.bottom}
  class:show-left={$edgeIndicators.left}
  class:show-right={$edgeIndicators.right}
>
  <div class="indicator top"></div>
  <div class="indicator bottom"></div>
  <div class="indicator left"></div>
  <div class="indicator right"></div>
</div>

<style>
  .edge-indicators {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 10;
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .edge-indicators.visible {
    opacity: 1;
    transition: opacity 0.2s ease;
  }

  .indicator {
    position: absolute;
    width: 0;
    height: 0;
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .show-top .top,
  .show-bottom .bottom,
  .show-left .left,
  .show-right .right {
    opacity: 1;
  }

  .indicator.top {
    left: 50%;
    transform: translateX(-50%);
    top: 0.5ch;
    border-left: 1ch solid transparent;
    border-right: 1ch solid transparent;
    border-bottom: 1ch solid var(--color-indicator);
  }

  .indicator.bottom {
    left: 50%;
    transform: translateX(-50%);
    bottom: 0.5ch;
    border-left: 1ch solid transparent;
    border-right: 1ch solid transparent;
    border-top: 1ch solid var(--color-indicator);
  }

  .indicator.left {
    top: 50%;
    transform: translateY(-50%);
    left: 0.5ch;
    border-top: 1ch solid transparent;
    border-bottom: 1ch solid transparent;
    border-right: 1ch solid var(--color-indicator);
  }

  .indicator.right {
    top: 50%;
    transform: translateY(-50%);
    right: 0.5ch;
    border-top: 1ch solid transparent;
    border-bottom: 1ch solid transparent;
    border-left: 1ch solid var(--color-indicator);
  }
</style>
