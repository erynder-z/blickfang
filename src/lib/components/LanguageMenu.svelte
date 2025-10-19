<script lang="ts">
  import { isLanguageMenuVisible } from "$lib/store";
  import { locale, setLocale, t, locales } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";

  type i18nLanguage = [string, Record<string, string>];

  const languages: i18nLanguage[] = Object.entries(locales);

  let dialog: HTMLDialogElement;

  isLanguageMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
    } else {
      dialog?.close();
    }
  });

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

  const handleClose = () => {
    isLanguageMenuVisible.set(false);
  };
</script>

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["options.language.heading"]}</h1>

    {#each languages as [lang, translations]}
      <!-- svelte-ignore a11y_autofocus -->
      <button on:click={() => handleButtonClick(lang)} autofocus={$locale === lang}>
        {translations["language.name"]}
      </button>
    {/each}

    <button on:click={handleClose} class="close-button">{$t["options.button.close"]}</button>
  </div>
</dialog>

<style>
  dialog {
    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px var(--color-shadow);
  }

  dialog::backdrop {
    background: var(--color-accent);
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

  .close-button {
    margin-top: 1rem;
  }
</style>
