<script lang="ts">
  import { isEdgeIndicatorMenuVisible, appConfig } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  let showButton: HTMLButtonElement;
  let hideButton: HTMLButtonElement;

  $: if ($isEdgeIndicatorMenuVisible) {
    tick().then(() => {
      const targetButton = $appConfig.edgeIndicatorsVisible ? showButton : hideButton;
      targetButton?.focus();
    });
  }

  const saveEdgeIndicatorMode = async (visible: boolean) => {
    try {
      await invoke("update_edge_indicators_visible_command", { visible });
    } catch (error) {
      console.error("Failed to save edge indicator mode:", error);
    }
  };

  const handleButtonClick = (visible: boolean) => saveEdgeIndicatorMode(visible);

  const handleClose = () => isEdgeIndicatorMenuVisible.set(false);

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isEdgeIndicatorMenuVisible) return;
    if (event.key === "Escape") handleClose();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isEdgeIndicatorMenuVisible}
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
      <h1>{$t["options.button.edge_indicators"]}</h1>

      <button
        bind:this={showButton}
        on:click={() => handleButtonClick(true)}
        class:active={$appConfig.edgeIndicatorsVisible}
      >
        {$t["general.show"]}
      </button>

      <button
        bind:this={hideButton}
        on:click={() => handleButtonClick(false)}
        class:active={!$appConfig.edgeIndicatorsVisible}
      >
        {$t["general.hide"]}
      </button>

      <button on:click={handleClose} class="close-button">
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
    width: clamp(40ch, 45ch, 90vw);
    min-height: 25rem;
    padding: 4rem;
    background-color: var(--color-background);
    border: 0.15rem solid var(--color-outline);
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
    border: 0.2rem solid var(--color-outline);
    background-color: var(--color-button);
    color: var(--color-text-primary);
    font-weight: 700;
    border-radius: 0.25rem;
    padding: 0.75rem 1.25rem;
    font-size: 1rem;
    cursor: pointer;
    box-shadow: 0.25rem 0.25rem 0 0 var(--color-outline);
    transition:
      transform 0.1s ease-out,
      box-shadow 0.1s ease-out,
      background-color 0.1s ease-out;
  }

  button:hover {
    transform: translate(-0.1rem, -0.1rem);
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
  }

  button:active {
    transform: translate(0.1rem, 0.1rem);
  }

  button:focus {
    outline: none;
    background-color: var(--color-accent);
    color: var(--color-text-tertiary);
  }

  .close-button {
    margin-top: 2rem;
    color: var(--color-close-button);
  }
</style>
