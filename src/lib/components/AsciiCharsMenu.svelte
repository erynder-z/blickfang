<script lang="ts">
  import { isAsciiCharsMenuVisible } from "$lib/stores";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { tick, onMount } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";
  import { appConfig } from "$lib/stores/config";
  import type { AsciiCharSetId, AsciiCharSet } from "$lib/types/ascii_art";

  let asciiCharSets: AsciiCharSet[] = [];

  onMount(async () => {
    try {
      const charSets: AsciiCharSet[] = await invoke("get_available_ascii_char_sets");
      asciiCharSets = charSets;
    } catch (error) {
      console.error("Failed to fetch ASCII character sets:", error);
    }
  });

  let buttons: HTMLButtonElement[] = [];
  let backgroundColor: string = $appConfig.asciiBackgroundColor || "#000000";
  let autoBackground: boolean = $appConfig.asciiAutoBackground || false;

  $: if ($isAsciiCharsMenuVisible) {
    tick().then(() => {
      if (buttons.length > 0) {
        const activeIndex = asciiCharSets.findIndex(
          (charSet) => charSet.id === $appConfig.asciiChars
        );
        const buttonToFocus = buttons[activeIndex !== -1 ? activeIndex : 0];
        buttonToFocus?.focus();
      }
    });
  }

  /**
   * Saves the selected ASCII character set to the application configuration.
   *
   * @param {string} charSet - The ID of the ASCII character set to save.
   *
   * @returns {Promise<void>} - A promise that resolves when the character set is saved.
   */
  const saveAsciiChars = async (charSet: AsciiCharSetId) => {
    try {
      await invoke("update_ascii_chars_command", { asciiChars: charSet });
      appConfig.update((config) => ({ ...config, asciiChars: charSet }));
    } catch (error) {
      console.error("Failed to save ASCII character set:", error);
    }
  };

  /**
   * Saves the selected ASCII background color to the application configuration.
   *
   * @returns {Promise<void>} - A promise that resolves when the background color is saved.
   */
  const saveBackgroundColor = async () => {
    try {
      await invoke("update_ascii_background_color_command", { backgroundColor });
      appConfig.update((config) => ({ ...config, asciiBackgroundColor: backgroundColor }));
    } catch (error) {
      console.error("Failed to save ASCII background color:", error);
    }
  };

  /**
   * Saves the selected ASCII auto background setting to the application configuration.
   *
   * @returns {Promise<void>} - A promise that resolves when the auto background setting is saved.
   */
  const saveAutoBackground = async () => {
    try {
      await invoke("update_ascii_auto_background_command", { enabled: autoBackground });
      appConfig.update((config) => ({ ...config, asciiAutoBackground: autoBackground }));
    } catch (error) {
      console.error("Failed to save ASCII auto background setting:", error);
    }
  };

  /**
   * Handles the click event on an ASCII character set button.
   *
   * Saves the selected ASCII character set to the application configuration
   * and closes the ASCII character set menu.
   *
   * @param {string} charSet - The ID of the ASCII character set to save.
   */
  const handleButtonClick = (charSet: AsciiCharSetId) => {
    saveAsciiChars(charSet);
    handleClose();
  };

  /**
   * Closes the ASCII character set menu.
   */
  const handleClose = () => isAsciiCharsMenuVisible.set(false);
  /**
   * Handles the keydown event on the ASCII character set menu.
   *
   * If the key is "Escape", closes the ASCII character set menu.
   *
   * @param {KeyboardEvent} event - The keydown event.
   */
  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isAsciiCharsMenuVisible) return;
    if (event.key === "Escape") {
      event.stopPropagation();
      handleClose();
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isAsciiCharsMenuVisible}
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
      <h1>{$t["options.asciiChars.heading"]}</h1>
      <div class="option-buttons">
        {#each asciiCharSets as { id, label, chars }, i}
          <button
            bind:this={buttons[i]}
            on:click={() => handleButtonClick(id)}
            class:active={$appConfig.asciiChars === id}
          >
            <div class="charset-label">{label}</div>
            <div class="charset-preview">{chars}</div>
          </button>
        {/each}
      </div>

      <h2>{$t["options.asciiBackgroundColor.heading"]}</h2>
      <div class="background-color-section">
        <div class="auto-background-section">
          <label class="checkbox-label">
            <input
              type="checkbox"
              bind:checked={autoBackground}
              on:change={saveAutoBackground}
              class="checkbox-input"
            />
            <span class="checkbox-text">{$t["options.asciiAutoBackground.label"]}</span>
          </label>
        </div>

        <div class="color-picker-section" class:disabled={autoBackground}>
          <div class="color-picker-container">
            <input
              type="color"
              id="background-color"
              bind:value={backgroundColor}
              on:change={saveBackgroundColor}
              class="color-picker"
              disabled={autoBackground}
            />
            <span class="color-value">{backgroundColor}</span>
          </div>
        </div>
      </div>

      <button on:click={handleClose} class="close-button">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          height="1.25rem"
          viewBox="0 -960 960 960"
          width="1.25rem"
          fill="var(--color-close-button)"
          ><path
            d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
          /></svg
        >
        {$t["general.close"]}
      </button>
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
    width: clamp(50ch, 55ch, 90vw);
    padding: 2.5rem;
    min-height: auto;
    background-color: var(--color-background);
    border: 0.2rem solid var(--color-outline);
    border-radius: 0.1rem;
    box-shadow: 0 2px 8px var(--color-shadow);
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    align-items: center;
  }

  h1 {
    margin: 0 0 1rem 0;
    color: var(--color-text-primary);
    line-height: 1.2;
    font-size: 1.25rem;
    text-wrap: balance;
    text-align: center;
  }

  h2 {
    margin: 2rem 0 1rem 0;
    color: var(--color-text-primary);
    line-height: 1.2;
    font-size: 1.25rem;
    text-wrap: balance;
    text-align: center;
  }

  .option-buttons {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    align-items: center;
  }

  button {
    width: 80%;
    border: 0.15rem solid var(--color-outline);
    padding: 0.75rem 1rem;
    border-radius: 0.1rem;
    color: var(--color-text-primary);
    background-color: var(--color-button);
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
    font-size: 0.9rem;
    font-weight: 600;
    transition:
      transform 150ms ease,
      box-shadow 150ms ease,
      background-color 150ms ease,
      color 150ms ease;
  }

  .charset-label {
    font-weight: 600;
    font-size: 0.9rem;
  }

  .charset-preview {
    font-family: monospace;
    font-size: 0.8rem;
    opacity: 0.8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--color-text-secondary);
  }

  button:hover {
    transform: translate(0.15rem, 0.15rem);
    box-shadow: 0.1rem 0.1rem 0 var(--color-outline);
  }

  button:active {
    transform: translate(0.35rem, 0.35rem);
    box-shadow: 0 0 0 var(--color-outline);
  }

  button:focus {
    outline: none;
    background-color: var(--color-accent);
    color: var(--color-text-tertiary);
  }

  button.active {
    background-color: var(--color-accent);
    color: var(--color-text-tertiary);
  }

  button.active .charset-preview {
    color: var(--color-text-tertiary);
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1.5rem;
    font-size: 0.9rem;
    padding: 0.5rem 1rem;
    min-width: 0;
    color: var(--color-close-button);
  }

  .background-color-section {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .auto-background-section {
    display: flex;
    justify-content: center;
    width: 80%;
    margin-top: 1rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .checkbox-input {
    width: 1.25rem;
    height: 1.25rem;
    cursor: pointer;
  }

  .checkbox-text {
    font-size: 0.9rem;
    color: var(--color-text-primary);
  }

  .color-picker-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 80%;
    margin-top: 1rem;
    opacity: 1;
    transition: opacity 0.2s ease;
  }

  .color-picker-section.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .color-picker-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .color-picker {
    width: 2.5rem;
    height: 2.5rem;
    padding: 0;
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.25rem;
    background-color: transparent;
    cursor: pointer;
  }

  .color-value {
    font-size: 0.9rem;
    font-family: monospace;
    color: var(--color-text-secondary);
    padding: 0.25rem 0.5rem;
    background-color: var(--color-button);
    border: 0.1rem solid var(--color-outline);
    border-radius: 0.2rem;
  }
</style>
