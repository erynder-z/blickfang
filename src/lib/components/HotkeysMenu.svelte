<script lang="ts">
  import { isHotkeysMenuVisible, appConfig, type Shortcuts, isRemapping } from "$lib/store";
  import { t } from "$lib/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import RemapDialog from './RemapDialog.svelte';

  let dialog: HTMLDialogElement;

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

  const handleRestoreDefaults = async () => {
    const defaultShortcuts = await invoke<Shortcuts>("get_default_shortcuts_command");
    const newConfig = { ...$appConfig, shortcuts: defaultShortcuts };
    appConfig.set(newConfig);
    await invoke("write_config_command", { content: JSON.stringify(newConfig) });
  };
</script>

<RemapDialog />

<dialog bind:this={dialog} on:close={handleClose}>
  <div class="menu-content">
    <h1>{$t["hotkeys.heading"]}</h1>
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
    <button on:click={handleRestoreDefaults} class="restore-button"
      >{$t["hotkeys.button.restore_defaults"]}</button
    >
    <button on:click={handleClose} class="close-button">{$t["hotkeys.button.close"]}</button>
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
    min-width: 300px;
    text-align: center;
  }

  h1 {
    margin: 0 0 0.5rem 0;
    color: #e3e3e3;
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

  button.remap-button,
  button.restore-button,
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
</style>