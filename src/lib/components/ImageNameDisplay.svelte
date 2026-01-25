<script lang="ts">
  import { imagePath, appConfig, isZenModeActive } from "$lib/stores";
  import { quintOut } from "svelte/easing";
  import { fade } from "svelte/transition";

  let timeoutId: number | undefined;
  let isVisible: boolean = true;

  $: imageName = $imagePath ? $imagePath.split(/[/\\]/).pop() : undefined;

  $: {
    const timeoutInMilliseconds = 3000;

    if ($appConfig.imageNameDisplayMode === "hide" || $isZenModeActive) {
      isVisible = false;
    } else if ($appConfig.imageNameDisplayMode === "show") {
      isVisible = true;
    } else if ($appConfig.imageNameDisplayMode === "fade") {
      isVisible = true;
      clearTimeout(timeoutId);
      if (imageName) {
        timeoutId = setTimeout(() => {
          isVisible = false;
        }, timeoutInMilliseconds);
      }
    }
  }
</script>

{#if imageName && isVisible && $appConfig.imageNameDisplayMode !== "hide"}
  <div in:fade={{ duration: 200, easing: quintOut }} out:fade={{ duration: 500, easing: quintOut }}>
    <p>{imageName}</p>
  </div>
{/if}

<style>
  div {
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    text-align: center;
    background-color: transparent;
    color: white;
    z-index: 10;
    display: flex;
    justify-content: center;
  }

  p {
    background-color: var(--color-shadow);
    color: white;
    padding: 0.5rem 0.75rem;
    border-radius: 9999px;
    max-width: 66vw;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
