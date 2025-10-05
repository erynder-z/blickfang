<script lang="ts">
  import { appConfig, type Shortcuts, isRemapping } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  let dialog: HTMLDialogElement;

  let remapStep = 0;
  let actionsToRemap: (keyof Shortcuts)[] = [];
  let tempShortcuts: Shortcuts;
  let errorMessage: string | null = null;

  $: if (dialog && $isRemapping) {
    dialog.showModal();
    errorMessage = null;
    remapStep = 0;
    actionsToRemap = Object.keys($appConfig.shortcuts) as (keyof Shortcuts)[];
    tempShortcuts = JSON.parse(JSON.stringify($appConfig.shortcuts));
  } else if (dialog && !$isRemapping) {
    dialog.close();
  }

  const handleClose = () => {
    isRemapping.set(false);
  };

  const handleCancelRemap = () => {
    handleClose();
  };

  const handleSaveRemap = async () => {
    const newConfig = { ...$appConfig, shortcuts: tempShortcuts };
    appConfig.set(newConfig);
    await invoke("write_config_command", { content: JSON.stringify(newConfig) });
    handleClose();
  };

  const handleKeydown = (event: KeyboardEvent) => {
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

  onMount(() => {
    window.addEventListener("keydown", handleKeydown, true);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown, true);
  });
</script>

<dialog bind:this={dialog} on:close={handleClose}>
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
        <button on:click={handleSaveRemap} class="remap-button">{$t["hotkeys.button.save"]}</button>
      {/if}
      <button on:click={handleCancelRemap} class="close-button"
        >{$t["hotkeys.button.cancel"]}</button
      >
    </div>
  {/if}
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
    min-width: 300px;
    text-align: center;
  }

  h1 {
    margin: 0 0 0.5rem 0;
    color: #e3e3e3;
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
    border: none;
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
  }

  p {
    color: var(--color-text-primary);
  }

  .remap-indicator {
    padding: 0.5rem;
    border: 1px dashed var(--color-accent);
    border-radius: 4px;
    min-height: 1.5rem;
    text-align: center;
  }

  .error {
    color: var(--color-error);
    margin-top: -0.5rem;
    margin-bottom: -0.5rem;
  }
</style>
