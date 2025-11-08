<script lang="ts">
  import { appConfig, type Shortcuts, isRemapping } from "$lib/stores/appState";
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";

  let remapStep = 0;
  let actionsToRemap: (keyof Shortcuts)[] = [];
  let tempShortcuts: Shortcuts;
  let errorMessage: string | null = null;

  $: if ($isRemapping) {
    errorMessage = null;
    remapStep = 0;
    actionsToRemap = Object.keys($appConfig.shortcuts) as (keyof Shortcuts)[];
    tempShortcuts = JSON.parse(JSON.stringify($appConfig.shortcuts));
  }

  const handleClose = () => isRemapping.set(false);

  const handleCancelRemap = () => handleClose();

  const handleSaveRemap = async () => {
    await invoke("update_custom_shortcuts_command", { newShortcuts: tempShortcuts });
    handleClose();
  };

  const handleRemappingKeydown = (event: KeyboardEvent) => {
    if (!$isRemapping || remapStep >= actionsToRemap.length) return;

    event.preventDefault();

    if (event.ctrlKey || event.altKey || event.metaKey) return;

    const currentAction = actionsToRemap[remapStep];
    if (!currentAction) return;

    const newKey = event.key;
    errorMessage = null;

    for (const action in tempShortcuts) {
      if (
        action !== currentAction &&
        tempShortcuts[action as keyof Shortcuts].keys.includes(newKey)
      ) {
        errorMessage = $t["hotkeys.remap.error.key_in_use"].replace("{key}", newKey);
        return;
      }
    }

    tempShortcuts[currentAction] = {
      keys: [newKey],
      label: newKey,
    };

    // Force update of the view
    tempShortcuts = tempShortcuts;

    if (remapStep < actionsToRemap.length - 1) {
      remapStep++;
    } else {
      remapStep++;
    }
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isRemapping) return;
    if (event.key === "Escape") handleClose();
  };

  onMount(() => {
    window.addEventListener("keydown", handleRemappingKeydown, true);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleRemappingKeydown, true);
  });
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isRemapping}
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
    in:fly={{ y: -200, duration: 100 }}
    out:fade={{ duration: 100 }}
  >
    {#if tempShortcuts}
      <div class="menu-content">
        <h1>{$t["hotkeys.remap.title"]}</h1>
        {#if remapStep < actionsToRemap.length}
          <p>
            {$t["hotkeys.remap.press_key_for"]} "{$t[`hotkeys.${actionsToRemap[remapStep]}`]}"
          </p>
          {#if errorMessage}
            <p class="error">{errorMessage}</p>
          {/if}
          <div class="hotkey-key remap-indicator">
            {tempShortcuts[actionsToRemap[remapStep]].label.toLocaleUpperCase()}
          </div>
        {:else}
          <p>{$t["hotkeys.remap.finished"]}</p>
          <button on:click={handleSaveRemap} class="remap-button">{$t["general.save"]}</button>
        {/if}
        <button on:click={handleCancelRemap} class="close-button">{$t["general.cancel"]}</button>
      </div>
    {/if}
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
    z-index: 150;
  }

  .menu-dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 200;
    background-color: var(--color-background);
    border: 0.15rem solid var(--color-accent);
    border-radius: 0.5rem;
    padding: 1.5rem;
    box-shadow:
      0.3rem 0.3rem 0 0 var(--color-outline),
      0.6rem 0.6rem 0 0 var(--color-shadow);
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .menu-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-width: 300px;
    text-align: center;
  }

  h1 {
    margin: 0 0 0.5rem 0;
    color: var(--color-text-primary);
    line-height: 1.2;
  }

  .hotkey-key {
    font-weight: bold;
    color: var(--color-text-secondary);
    justify-self: end;
  }

  button.remap-button,
  button.close-button {
    margin-top: 1rem;
    padding: 0.5rem;
    border: 0.15rem solid var(--color-outline);
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
    min-height: 2.5rem;
    white-space: nowrap;
    text-overflow: ellipsis;
    box-shadow: 0.2rem 0.2rem 0 0 var(--color-outline);
    transition: all 0.15s ease;
  }

  button.remap-button:hover,
  button.close-button:hover {
    transform: translateY(-1px);
    box-shadow: 0.3rem 0.3rem 0 0 var(--color-outline);
  }

  button.remap-button:active,
  button.close-button:active {
    transform: translateY(1px);
    box-shadow: inset 0.1rem 0.1rem 0 0 var(--color-outline);
  }

  button.remap-button:focus,
  button.close-button:focus {
    outline: 0.15rem solid var(--color-accent);
    outline-offset: 0.15rem;
  }

  p {
    color: var(--color-text-primary);
  }

  .remap-indicator {
    padding: 0.5rem;
    border: 0.15rem dashed var(--color-accent);
    border-radius: 0.5rem;
    min-height: 1.5rem;
    text-align: center;
    box-shadow: inset -0.1rem -0.1rem 0 0 var(--color-outline);
  }

  .error {
    color: var(--color-error);
    margin-top: -0.5rem;
    margin-bottom: -0.5rem;
    font-weight: bold;
  }
</style>
