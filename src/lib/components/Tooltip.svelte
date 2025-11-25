<script lang="ts">
  export let text = "";
  export let visible = false;
  export let targetRect: DOMRect | null = null;

  let tooltipEl: HTMLDivElement;
  let top = 0;
  let left = 0;

  $: if (visible && targetRect && tooltipEl) {
    const { height: tooltipHeight, width: tooltipWidth } = tooltipEl.getBoundingClientRect();
    const windowHeight = window.innerHeight;
    const windowWidth = window.innerWidth;

    let newTop = targetRect.bottom + 8;

    // If default position is off-screen, place it above
    if (newTop + tooltipHeight > windowHeight) newTop = targetRect.top - tooltipHeight - 8;

    // Horizontal centering
    let newLeft = targetRect.left + targetRect.width / 2 - tooltipWidth / 2;

    // Clamp to window edges
    if (newLeft < 8) newLeft = 8;
    if (newLeft + tooltipWidth > windowWidth) newLeft = windowWidth - tooltipWidth - 8;

    top = newTop;
    left = newLeft;
  }
</script>

{#if visible}
  <div class="tooltip" bind:this={tooltipEl} style="top: {top}px; left: {left}px;">
    {text}
  </div>
{/if}

<style>
  .tooltip {
    position: fixed;
    background-color: var(--color-tooltip-background);
    color: var(--color-tooltip-text);
    padding: 5px 10px;
    border-radius: 5px;
    z-index: 9999;
    font-size: 0.875rem;
    pointer-events: none;
    white-space: nowrap;
  }
</style>
