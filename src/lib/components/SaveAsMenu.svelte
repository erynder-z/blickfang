<script lang="ts">
  import { isSaveAsMenuVisible } from "$lib/stores/appState";
  import { saveImageAs } from "$lib/core/commands";
  import { t } from "$lib/utils/i18n";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let formats: string[] = [];

  onMount(async () => {
    try {
      formats = await invoke<string[]>("get_supported_image_formats");
    } catch (error) {
      console.error("Failed to get supported image formats:", error);
    }
  });

  const handleFormatClick = (format: string) => {
    saveImageAs(format.toLowerCase());
    handleClose();
  };

  const handleClose = () => isSaveAsMenuVisible.set(false);

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isSaveAsMenuVisible) return;
    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isSaveAsMenuVisible}
  <!-- svelte-ignore a11y-no-static-element-interactions, a11y-click-events-have-key-events -->
  <div
    class="backdrop"
    on:click={handleClose}
    in:fade={{ duration: 100 }}
    out:fade={{ duration: 100 }}
  ></div>
  <div
    use:focusTrap
    class="menu-dialog"
    role="dialog"
    aria-modal="true"
    in:fly={{ y: 200, duration: 100 }}
    out:fade={{ duration: 100 }}
  >
    <div class="menu-content">
      <h1>{$t["saveAs.heading"]}</h1>

      <div class="formats-grid">
        {#each formats as format}
          <button on:click={() => handleFormatClick(format)}>
            {format}
          </button>
        {/each}
      </div>

      <button on:click={handleClose} class="close-button">{$t["general.close"]}</button>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: var(--color-dialog-backdrop);
    z-index: 30;
  }
  .menu-dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 100;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: clamp(40ch, 45ch, 90vw);
    padding: 1.5rem;
    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    box-shadow: 0 4px 12px var(--color-shadow);
  }
  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    min-width: 300px;
    text-align: center;
    width: 100%;
  }

  h1 {
    color: var(--color-text-primary);
    line-height: 1.2;
    text-wrap: balance;
    min-height: 2.5em;
    margin: 0;
  }

  .formats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
    gap: 1rem;
  }

  button {
    padding: 0.5rem;
    border: solid 0.15rem var(--color-outline);
    border-radius: 0.5rem;
    background-color: transparent;
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
    min-height: 2.5rem;
  }

  button:focus,
  button:hover {
    outline: solid 0.15rem var(--color-accent);
    outline-offset: 0.15rem;
    background-color: rgb(from var(--color-accent) r g b / 0.2);
  }

  .close-button {
    margin-top: 1rem;
    background-color: var(--color-close-button);
  }
</style>
