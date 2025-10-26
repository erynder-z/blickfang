<script lang="ts">
  import { isThemeMenuVisible, appConfig } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import themes from "$lib/theme/themes.json";
  import { tick } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  const themeNames = Object.keys(themes);

  let buttons: HTMLButtonElement[] = [];

  $: if ($isThemeMenuVisible) {
    tick().then(() => {
      const currentIndex = themeNames.indexOf($appConfig.theme);
      if (currentIndex !== -1) buttons[currentIndex]?.focus();
    });
  }

  const saveTheme = async (theme: string) => {
    try {
      await invoke("update_theme_command", { theme: theme });
    } catch (error) {
      console.error("Failed to save theme:", error);
    }
  };

  const handleButtonClick = (theme: string) => {
    saveTheme(theme);
    handleClose();
  };

  const handleClose = () => isThemeMenuVisible.set(false);

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isThemeMenuVisible) return;
    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isThemeMenuVisible}
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
      <h1>{$t["options.theme.heading"]}</h1>

      {#each themeNames as theme, i}
        <button
          bind:this={buttons[i]}
          on:click={() => handleButtonClick(theme)}
          class:active={$appConfig.theme === theme}
        >
          {theme}
        </button>
      {/each}

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
    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px var(--color-shadow);
  }
  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-width: 200px;
    text-align: center;
  }
  h1 {
    margin: 0 0 0.5rem 0;
    color: #e3e3e3;
  }
  button {
    padding: 0.5rem;
    border: solid 0.15rem var(--color-outline);
    border-radius: 0.5rem;
    background-color: transparent;
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
  }
  button:focus {
    outline: solid 0.15rem var(--color-accent);
    outline-offset: 0.15rem;
  }

  button.active {
    background-color: var(--color-accent);
  }
  .close-button {
    margin-top: 1rem;
    background-color: var(--color-close-button);
  }
</style>
