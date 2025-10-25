<script lang="ts">
  import { isLanguageMenuVisible } from "$lib/stores/appState";
  import { locale, setLocale, t, locales } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";
  import { blur } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  type i18nLanguage = [string, Record<string, string>];

  const languages: i18nLanguage[] = Object.entries(locales);
  const langCodes = languages.map(([langCode]) => langCode);

  let buttons: HTMLButtonElement[] = [];

  $: if ($isLanguageMenuVisible) {
    tick().then(() => {
      const currentIndex = langCodes.indexOf($locale);
      if (currentIndex !== -1) buttons[currentIndex]?.focus();
    });
  }

  const saveLanguage = async (lang: string) => {
    try {
      await invoke("update_language_command", { language: lang });
    } catch (error) {
      console.error("Failed to save language:", error);
    }
  };

  const handleButtonClick = (lang: string) => {
    setLocale(lang);
    saveLanguage(lang);
    handleClose();
  };

  const handleClose = () => isLanguageMenuVisible.set(false);

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isLanguageMenuVisible) return;
    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isLanguageMenuVisible}
  <!-- svelte-ignore a11y-no-static-element-interactions, a11y-click-events-have-key-events -->
  <div class="backdrop" on:click={handleClose} transition:blur={{ duration: 100 }}></div>
  <div
    use:focusTrap
    class="menu-dialog"
    role="dialog"
    aria-modal="true"
    transition:blur={{ duration: 100 }}
  >
    <div class="menu-content">
      <h1>{$t["options.language.heading"]}</h1>

      {#each languages as [lang, translations], i}
        <button bind:this={buttons[i]} on:click={() => handleButtonClick(lang)}>
          {translations["language.name"]}
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
    border: none;
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
  }
  button:focus {
    outline: none;
    background-color: var(--color-accent);
  }
  .close-button {
    margin-top: 1rem;
  }
</style>
