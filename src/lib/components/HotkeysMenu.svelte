<script lang="ts">
  import { isHotkeysMenuVisible, appConfig, isRemapping } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import RemapDialog from "./RemapDialog.svelte";
  import type { Shortcuts } from "$lib/store";
  import { onMount } from "svelte";

  let dialog: HTMLDialogElement;
  let defaultShortcuts: Shortcuts | null = null;

  onMount(async () => {
    defaultShortcuts = await invoke<Shortcuts>("get_default_shortcuts_command");
  });

  $: isUsingDefault =
    defaultShortcuts &&
    $appConfig.shortcuts &&
    JSON.stringify($appConfig.shortcuts) === JSON.stringify(defaultShortcuts);

  isHotkeysMenuVisible.subscribe((visible) => {
    if (visible) {
      dialog?.showModal();
    } else {
      dialog?.close();
    }
  });

  const handleRemap = () => {
    isRemapping.set(true);
  };

  const handleClose = () => {
    isHotkeysMenuVisible.set(false);
  };

  const handleSetDefault = () => {
    invoke("set_active_shortcuts_to_default");
  };

  const handleSetCustom = () => {
    invoke("set_active_shortcuts_to_custom");
  };
</script>

<RemapDialog />

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["hotkeys.heading"]}</h1>

    <div class="toggle-buttons">
      <button class:selected={isUsingDefault} on:click={handleSetDefault}>
        {$t["hotkeys.button.default_hotkeys"]}
      </button>
      <button class:selected={!isUsingDefault} on:click={handleSetCustom}>
        {$t["hotkeys.button.custom_hotkeys"]}
      </button>
    </div>

    <div class="hotkeys-grid">
      {#if $appConfig.shortcuts}
        {#each Object.keys($appConfig.shortcuts) as action}
          <div class="hotkey-action">{$t[`hotkeys.${action}`]}</div>
          <div class="hotkey-key">
            {$appConfig.shortcuts[action as keyof Shortcuts].label.toLocaleUpperCase()}
          </div>
        {/each}
      {/if}
    </div>

    <button on:click={handleRemap} class="remap-button">{$t["hotkeys.button.remap"]}</button>

    <button on:click={handleClose} class="close-button">{$t["general.close"]}</button>
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
    gap: 1.5rem;
    min-width: 300px;
    text-align: center;
  }

  h1 {
    margin: 0;
    color: #e3e3e3;
  }

  .toggle-buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    border: 1px solid var(--color-accent);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .toggle-buttons button {
    padding: 0.5rem;
    border: none;
    background-color: transparent;
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
    transition: background-color 0.2s ease;
  }

  .toggle-buttons button:first-child {
    border-right: 1px solid var(--color-accent);
  }

  .toggle-buttons button.selected {
    background-color: var(--color-accent);
  }

  .toggle-buttons button:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
  }

  .hotkeys-grid {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.5rem 1.5rem;
    text-align: left;
  }

  .hotkey-action {
    color: var(--color-text-primary);
  }

  .hotkey-key {
    font-weight: bold;
    color: var(--color-text-secondary);
    justify-self: end;
  }

  .remap-button,
  .close-button {
    margin-top: 1rem;
    padding: 0.5rem;
    border: none;
    border-radius: 0.5rem;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: bold;
  }
</style>
