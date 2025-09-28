<script lang="ts">
  import { isLanguageMenuVisible } from "$lib/store";
  import { locale, setLocale, t, locales } from "$lib/i18n";

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

  const handleButtonClick = (lang: string) => {
    setLocale(lang);
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

    <button on:click={handleClose} class="close-button">Close</button>
  </div>
</dialog>

<style>
  dialog {
    background-color: #372e49;
    border: 1px solid #ccc;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  dialog::backdrop {
    background: rgba(0, 0, 0, 0.5);
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
    background-color: rgba(255, 255, 255, 0.2);
    color: #e3e3e3;
    cursor: pointer;
    font-weight: bold;
  }

  .close-button {
    margin-top: 1rem;
  }
</style>
