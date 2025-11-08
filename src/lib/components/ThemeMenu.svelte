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

  const handleButtonClick = (theme: string) => saveTheme(theme);

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

    display: flex;
    flex-direction: column;
    align-items: center;

    width: clamp(30ch, 35ch, 90vw);
    min-height: 25rem;
    padding: 1.5rem;

    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    box-shadow: 0 4px 12px var(--color-shadow);

    transition: height 0.2s ease;
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    justify-content: center;
    gap: 1rem;
    text-align: center;
    width: 100%;
    max-width: 100%;
    overflow-wrap: break-word;
  }

  h1 {
    color: var(--color-text-primary);
    line-height: 1.2;
    text-wrap: balance;
    min-height: 2.5em;
    margin: 0;
  }

  button {
    appearance: none;
    border: 2px solid var(--color-outline);
    background-color: var(--color-button);
    color: var(--color-text-primary);
    font-weight: 700;
    border-radius: 0.25rem;
    padding: 0.75rem 1.25rem;
    font-size: 0.95rem;
    cursor: pointer;
    box-shadow: 0.2rem 0.2rem 0 var(--color-outline);
    transition:
      transform 0.1s ease-out,
      box-shadow 0.1s ease-out,
      background-color 0.1s ease-out;
  }

  button:hover {
    background-color: color-mix(in srgb, var(--color-button) 85%, var(--color-accent));
    transform: translate(-0.1rem, -0.1rem);
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
  }

  button:active {
    transform: translate(0.1rem, 0.1rem);
    box-shadow: 1px 1px 0 var(--color-outline);
  }

  button:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .close-button {
    margin-top: 1rem;
    background-color: var(--color-close-button);
  }
</style>
