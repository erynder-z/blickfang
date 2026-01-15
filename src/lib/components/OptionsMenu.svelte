<script lang="ts">
  import { t } from "$lib/utils/i18n";
  import {
    isLanguageMenuVisible,
    isOptionsMenuVisible,
    isThemeMenuVisible,
    isHotkeysMenuVisible,
    isToolbarMenuVisible,
    isImageNameDisplayMenuVisible,
    isEdgeIndicatorMenuVisible,
    isAppWindowMenuVisible,
    isAboutMenuVisible,
    isAsciiCharsMenuVisible,
    isGridOverlayMenuVisible,
    tooltip,
  } from "$lib/stores";
  import { toggleOptions } from "$lib/core/commands";
  import type { Writable } from "svelte/store";

  const openMenu = (store: Writable<boolean>) => store.set(true);
</script>

<div class="options-sidebar-overlay" class:visible={$isOptionsMenuVisible}>
  <div class="options-container">
    <button
      class="hotkeys-button"
      on:click={() => openMenu(isHotkeysMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.hotkeys"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.hotkeys"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M160-200q-33 0-56.5-23.5T80-280v-400q0-33 23.5-56.5T160-760h640q33 0 56.5 23.5T880-680v400q0 33-23.5 56.5T800-200H160Zm0-80h640v-400H160v400Zm160-40h320v-80H320v80ZM200-440h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM200-560h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM160-280v-400 400Z"
        /></svg
      >
    </button>
    <button
      class="language-button"
      on:click={() => openMenu(isLanguageMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.language"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.language"]}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 31.5-155.5t86-127Q252-817 325-848.5T480-880q83 0 155.5 31.5t127 86q54.5 54.5 86 127T880-480q0 82-31.5 155t-86 127.5q-54.5 54.5-127 86T480-80Zm0-82q26-36 45-75t31-83H404q12 44 31 83t45 75Zm-104-16q-18-33-31.5-68.5T322-320H204q29 50 72.5 87t99.5 55Zm208 0q56-18 99.5-55t72.5-87H638q-9 38-22.5 73.5T584-178ZM170-400h136q-3-20-4.5-39.5T300-480q0-21 1.5-40.5T306-560H170q-5 20-7.5 39.5T160-480q0 21 2.5 40.5T170-400Zm216 0h188q3-20 4.5-39.5T580-480q0-21-1.5-40.5T574-560H386q-3 20-4.5 39.5T380-480q0 21 1.5 40.5T386-400Zm268 0h136q5-20 7.5-39.5T800-480q0-21-2.5-40.5T790-560H654q3 20 4.5 39.5T660-480q0 21-1.5 40.5T654-400Zm-16-240h118q-29-50-72.5-87T584-782q18 33 31.5 68.5T638-640Zm-234 0h152q-12-44-31-83t-45-75q-26 36-45 75t-31 83Zm-200 0h118q9-38 22.5-73.5T376-782q-56 18-99.5 55T204-640Z"
        /></svg
      ></button
    >
    <button
      class="theme-button"
      on:click={() => openMenu(isThemeMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.theme"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.theme"]}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 32.5-156t88-127Q256-817 330-848.5T488-880q80 0 151 27.5t124.5 76q53.5 48.5 85 115T880-518q0 115-70 176.5T640-280h-74q-9 0-12.5 5t-3.5 11q0 12 15 34.5t15 51.5q0 50-27.5 74T480-80Zm0-400Zm-220 40q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17Zm120-160q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17Zm200 0q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17Zm120 160q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17ZM480-160q9 0 14.5-5t5.5-13q0-14-15-33t-15-57q0-42 29-67t71-25h70q66 0 113-38.5T800-518q0-121-92.5-201.5T488-800q-136 0-232 93t-96 227q0 133 93.5 226.5T480-160Z"
        /></svg
      ></button
    >
    <button
      class="ascii-chars-button"
      on:click={() => openMenu(isAsciiCharsMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.asciiChars"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.asciiChars"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M224-878h78l135 360h-75l-30-86H194l-30 86H89l135-360Zm5 402q42 0 71 29t29 71q0 20-7.5 38.5T300-305l-15 14 28 28 57-56 57 57-56 56 56 57-57 56-56-56-42 42q-15 15-34 22.5T198-77q-42 0-70-29.5T100-178q0-20 8-38.5t22-32.5l42-42-14-14q-14-14-22-32t-8-38q0-42 29.5-71.5T229-476Zm0 242-43 42q-3 3-4 6.5t-1 7.5q0 8 5.5 14t13.5 6q4 0 7.5-1.5t6.5-4.5l43-42-28-28Zm-1-162q-8 0-13.5 6t-5.5 14q0 4 1 7.5t4 6.5l15 14 13-13q3-3 4.5-6.5t1.5-7.5q0-8-6-14.5t-14-6.5Zm33-404-46 134h96l-46-134h-4Zm438-105q8 13 13.5 27t9.5 28l-43 13h201v80h-67q-11 33-28.5 62.5T744-640q27 16 56 26.5t60 18.5l-19 77q-43-11-83.5-27T680-586q-37 24-77.5 40.5T519-518l-18-77q30-8 59-18.5t56-26.5q-23-25-40.5-54.5T548-757h-68v-80h176q-3-13-8-25.5T638-887l61-18Zm104 444 57 56L549-94l-57-56 311-311Zm-223 23q25 0 42.5 17.5T640-378q0 25-17.5 42.5T580-318q-25 0-42.5-17.5T520-378q0-25 17.5-42.5T580-438Zm53-319q8 19 20 36t27 32q15-15 27-32t21-36h-95Zm147 519q25 0 42.5 17.5T840-178q0 25-17.5 42.5T780-118q-25 0-42.5-17.5T720-178q0-25 17.5-42.5T780-238Z"
        /></svg
      ></button
    >
    <button
      class="grid-overlay-button"
      on:click={() => openMenu(isGridOverlayMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.gridOverlay"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.gridOverlay"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M360-80v-280H80v-80h280v-80H80v-80h280v-280h80v280h80v-280h80v280h280v80H600v80h280v80H600v280h-80v-280h-80v280h-80Zm80-360h80v-80h-80v80Z"
        /></svg
      ></button
    >
    <button
      class="toolbar-button"
      on:click={() => openMenu(isToolbarMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.toolbar"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.toolbar"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M160-400q-33 0-56.5-23.5T80-480q0-33 23.5-56.5T160-560q33 0 56.5 23.5T240-480q0 33-23.5 56.5T160-400Zm0-54q11 0 18.5-7.5T186-480q0-11-7.5-18.5T160-506q-11 0-18.5 7.5T134-480q0 11 7.5 18.5T160-454Zm214 54q-33 0-56.5-23.5T294-480q0-33 23.5-56.5T374-560q33 0 56.5 23.5T454-480q0 33-23.5 56.5T374-400Zm0-54q11 0 18.5-7.5T400-480q0-11-7.5-18.5T374-506q-11 0-18.5 7.5T348-480q0 11 7.5 18.5T374-454Zm212 54q-33 0-56.5-23.5T506-480q0-33 23.5-56.5T586-560q33 0 56.5 23.5T666-480q0 33-23.5 56.5T586-400Zm0-54q11 0 18.5-7.5T612-480q0-11-7.5-18.5T586-506q-11 0-18.5 7.5T560-480q0 11 7.5 18.5T586-454Zm214 54q-33 0-56.5-23.5T720-480q0-33 23.5-56.5T800-560q33 0 56.5 23.5T880-480q0 33-23.5 56.5T800-400Z"
        /></svg
      >
    </button>
    <button
      class="image-name-button"
      on:click={() => openMenu(isImageNameDisplayMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.imageName"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.imageName"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M680-360q-17 0-28.5-11.5T640-400v-160q0-17 11.5-28.5T680-600h120q17 0 28.5 11.5T840-560v40h-60v-20h-80v120h80v-20h60v40q0 17-11.5 28.5T800-360H680Zm-300 0v-240h160q17 0 28.5 11.5T580-560v40q0 17-11.5 28.5T540-480q17 0 28.5 11.5T580-440v40q0 17-11.5 28.5T540-360H380Zm60-150h80v-30h-80v30Zm0 90h80v-30h-80v30Zm-320 60v-200q0-17 11.5-28.5T160-600h120q17 0 28.5 11.5T320-560v200h-60v-60h-80v60h-60Zm60-120h80v-60h-80v60Z"
        /></svg
      ></button
    >
    <button
      class="edge-indicators-button"
      on:click={() => openMenu(isEdgeIndicatorMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.edgeIndicators"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.edgeIndicators"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="m480-193 85-85 57 56L480-80 338-222l57-56 85 85ZM193-480l85 85-56 57L80-480l142-142 56 57-85 85Zm574 0-85-85 56-57 142 142-142 142-56-57 85-85ZM480-767l-85 85-57-56 142-142 142 142-57 56-85-85Z"
        /></svg
      ></button
    >
    <button
      class="app-window-button"
      on:click={() => openMenu(isAppWindowMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.appWindow"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.appWindow"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-text-primary)"
        ><path
          d="M160-80q-33 0-56.5-23.5T80-160v-360q0-33 23.5-56.5T160-600h80v-200q0-33 23.5-56.5T320-880h480q33 0 56.5 23.5T880-800v360q0 33-23.5 56.5T800-360h-80v200q0 33-23.5 56.5T640-80H160Zm0-80h480v-280H160v280Zm560-280h80v-280H320v120h320q33 0 56.5 23.5T720-520v80Z"
        /></svg
      ></button
    >
    <button
      class="about-button"
      on:click={() => openMenu(isAboutMenuVisible)}
      on:mouseenter={(e) => tooltip.show($t["tooltip.about"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.about"]}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="24px"
        fill="var(--color-text-primary)"
        ><path
          d="M478-240q21 0 35.5-14.5T528-290q0-21-14.5-35.5T478-340q-21 0-35.5 14.5T428-290q0 21 14.5 35.5T478-240Zm-36-154h74q0-33 7.5-52t42.5-52q26-26 41-49.5t15-56.5q0-56-41-86t-97-30q-57 0-92.5 30T342-618l66 26q5-18 22.5-39t53.5-21q32 0 48 17.5t16 38.5q0 20-12 37.5T506-526q-44 39-54 59t-10 73Zm38 314q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"
        /></svg
      >
    </button>
    <button
      class="close-options-button"
      on:click={toggleOptions}
      on:mouseenter={(e) => tooltip.show($t["tooltip.closeMenu"], e.currentTarget)}
      on:mouseleave={tooltip.hide}
      aria-label={$t["tooltip.closeMenu"]}
    >
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
    </button>
  </div>
</div>

<style>
  .options-sidebar-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 2rem;
    background-color: var(--color-background);
    transform: translateY(100%);
    transition: transform 0.2s ease-in-out;
    z-index: 20;
    border-top: 0.2rem solid var(--color-outline);
    opacity: 0.9;
  }

  .options-sidebar-overlay.visible {
    transform: translateY(0);
  }

  .options-container {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  button {
    appearance: none;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: fit-content;
    height: 3rem;
    border: 0.15rem solid var(--color-outline);
    padding: 0.5rem 1rem;
    border-radius: 0.1rem;
    color: var(--color-text-primary);
    background-color: var(--color-button);
    box-shadow: 0.25rem 0.25rem 0 var(--color-outline);
    font-size: 0.9rem;
    font-weight: 600;
    text-wrap: nowrap;
    text-overflow: ellipsis;
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

  button svg {
    width: 1.3rem;
    height: 1.3rem;
    stroke-width: 0.5;
  }

  .close-options-button {
    font-size: 0.9rem;
    padding: 0.5rem 1rem;
    width: auto;
    color: var(--color-close-button);
  }
</style>
