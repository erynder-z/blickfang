<script lang="ts">
  import { appConfig, notification } from "$lib/stores";
  import { quintOut } from "svelte/easing";
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";

  let timeoutId: ReturnType<typeof setTimeout>;

  // Auto-hide after 3 seconds when notification becomes visible
  $: if ($notification.visible) {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => {
      $notification = { ...$notification, visible: false };
    }, 3000);
  }

  onMount(() => {
    return () => clearTimeout(timeoutId);
  });
</script>

{#if $notification.visible && $appConfig.imageNameDisplayMode !== "hide"}
  <div in:fade={{ duration: 200, easing: quintOut }} out:fade={{ duration: 500, easing: quintOut }}>
    <p>{$notification.message}</p>
  </div>
{/if}

<style>
  div {
    position: absolute;
    bottom: 5%;
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
