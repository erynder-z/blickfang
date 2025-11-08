<script lang="ts">
  import { isSaveAsMenuVisible } from "$lib/stores/appState";
  import { saveImageAs } from "$lib/core/commands";
  import { t } from "$lib/utils/i18n";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let losslessFormats: string[] = [];
  let lossyFormats: string[] = [];
  let quality: number = 75;

  onMount(async () => {
    try {
      const formats = await invoke<string[]>("get_supported_image_formats");
      const lossy = ["jpeg", "webp"];
      lossyFormats = formats.filter((f) => lossy.includes(f.toLowerCase()));
      losslessFormats = formats.filter((f) => !lossy.includes(f.toLowerCase()));
    } catch (error) {
      console.error("Failed to get supported image formats:", error);
    }
  });

  const handleFormatClick = (format: string) => {
    const formatLower = format.toLowerCase();
    if (lossyFormats.map((f) => f.toLowerCase()).includes(formatLower)) {
      saveImageAs(formatLower, quality);
    } else {
      saveImageAs(formatLower, undefined);
    }
    handleClose();
  };

  const handleClose = () => {
    isSaveAsMenuVisible.set(false);
  };

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

      <div class="format-group-lossless">
        <h2 class="format-type-heading">Lossless</h2>
        <div class="formats-grid">
          {#each losslessFormats as format}
            <button on:click={() => handleFormatClick(format)}>
              {format}
            </button>
          {/each}
        </div>
      </div>

      <div class="format-group-lossy">
        <h2 class="format-type-heading">Lossy</h2>
        <div class="formats-grid">
          {#each lossyFormats as format}
            <button on:click={() => handleFormatClick(format)}>
              {format}
            </button>
          {/each}
        </div>

        <div class="quality-slider">
          <label for="quality">{$t["saveAs.quality"]} ({quality}%)</label>
          <div class="input-container">
            <span>{$t["general.smaller_filesize"]}</span>
            <input
              class="slider"
              type="range"
              id="quality"
              name="quality"
              min="1"
              max="100"
              bind:value={quality}
            />
            <span>{$t["general.better_quality"]}</span>
          </div>
        </div>
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

  .format-group-lossless {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .format-group-lossy {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .format-type-heading {
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    margin: 0;
    text-align: left;
    padding-left: 0.25rem;
  }

  .formats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
    gap: 1rem;
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

  .close-button:focus {
    outline: solid 0.15rem var(--color-accent);
    background-color: var(--color-close-button);
    outline-offset: 0.15rem;
  }

  .quality-slider {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .quality-slider .slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 0.25rem;
    background: var(--color-outline);
    outline: none;
  }

  .quality-slider .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 1rem;
    height: 1rem;
    background: var(--color-accent);
    outline: solid 0.2ch var(--color-outline);
    cursor: pointer;
  }

  .quality-slider label {
    font-size: 0.9rem;
    color: var(--color-text-secondary);
  }

  .quality-slider .input-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.75rem;
    color: var(--color-text-secondary);
    line-height: 1.2;
  }

  .quality-slider input[type="range"] {
    width: 100%;
  }
</style>
