<script lang="ts">
  import { t } from "$lib/utils/i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { fly, fade } from "svelte/transition";
  import { focusTrap } from "$lib/actions/focusTrap";
  import { isLinuxDesktopInstallDialogVisible } from "$lib/stores/initialDialog";

  let buttons: HTMLButtonElement[] = [];

  const handleInstallClick = async () => {
    await invoke("install_linux_desktop_file_command");
    await invoke("set_linux_desktop_install_choice_command", { choice: "installed" });
    isLinuxDesktopInstallDialogVisible.set(false);
  };

  const handleSkipClick = async () => {
    await invoke("set_linux_desktop_install_choice_command", { choice: "skipped" });
    isLinuxDesktopInstallDialogVisible.set(false);
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (!$isLinuxDesktopInstallDialogVisible) return;
    if (event.key === "Escape") handleSkipClick();
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $isLinuxDesktopInstallDialogVisible}
  <!-- svelte-ignore a11y-no-static-element-interactions, a11y-click-events-have-key-events -->
  <div class="backdrop" in:fade={{ duration: 100 }} out:fade={{ duration: 100 }}></div>
  <div
    use:focusTrap
    class="menu-dialog"
    role="dialog"
    aria-modal="true"
    in:fly={{ y: 200, duration: 100 }}
    out:fade={{ duration: 100 }}
  >
    <div class="menu-content">
      <h1>{$t["linuxDesktopInstall.heading"]}</h1>
      <p>{$t["linuxDesktopInstall.message"]}</p>

      <div class="option-buttons">
        <button bind:this={buttons[0]} on:click={handleInstallClick}>
          {$t["linuxDesktopInstall.installButton"]}
        </button>
        <button bind:this={buttons[1]} on:click={handleSkipClick}>
          {$t["linuxDesktopInstall.skipButton"]}
        </button>
      </div>
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
    background: black;
    z-index: 130;
  }

  .menu-dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 200;
    display: flex;
    flex-direction: column;
    align-items: center;
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
  }

  p {
    margin: 0 0 1.5rem 0;
    color: var(--color-text-secondary);
    text-align: center;
    text-wrap: balance;
  }

  .option-buttons {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    align-items: center;
  }

  button {
    width: fit-content;
    min-width: 100%;
    border: 0.15rem solid var(--color-outline);
    padding: 0.5rem 1rem;
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
</style>
