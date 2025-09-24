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
    background-color: rgba(255, 0, 255, 0.55);
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
    width: 50%;
    height: 0.5ch;
    left: 25%;
    top: 0;
  }

  .indicator.bottom {
    width: 50%;
    height: 0.5ch;
    left: 25%;
    bottom: 0;
  }

  .indicator.left {
    width: 0.5ch;
    height: 33%;
    top: 33.33%;
    left: 0;
  }

  .indicator.right {
    width: 0.5ch;
    height: 33%;
    top: 33.33%;
    right: 0;
  }
</style>
