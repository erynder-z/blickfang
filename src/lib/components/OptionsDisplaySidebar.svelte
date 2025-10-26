<script lang="ts">
  import { t } from "$lib/utils/i18n";
  import {
    isLanguageMenuVisible,
    isOptionsSidebarVisible,
    isThemeMenuVisible,
    isHotkeysMenuVisible,
    isButtonMenuVisible,
    isImageNameDisplayMenuVisible,
    isEdgeIndicatorMenuVisible,
    isAppWindowMenuVisible,
  } from "$lib/stores/appState";

  const showLanguageSelectOverlay = () => {
    isLanguageMenuVisible.set(true);
  };

  const showThemeSelectOverlay = () => {
    isThemeMenuVisible.set(true);
  };

  const showHotkeysOverlay = () => {
    isHotkeysMenuVisible.set(true);
  };

  const showUIButtonOptionsOverlay = () => {
    isButtonMenuVisible.set(true);
  };

  const showImageNameDisplayMenu = () => {
    isImageNameDisplayMenuVisible.set(true);
  };

  const showEdgeIndicatorMenu = () => {
    isEdgeIndicatorMenuVisible.set(true);
  };

  const showAppWindowMenu = () => {
    isAppWindowMenuVisible.set(true);
  };
</script>

<div class="options-sidebar-overlay" class:visible={$isOptionsSidebarVisible}>
  <h1>{$t["options.title"]}</h1>
  <div class="options-container">
    <button class="hotkeys-button" on:click={showHotkeysOverlay}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-hotkeys-icon)"
        ><path
          d="M160-200q-33 0-56.5-23.5T80-280v-400q0-33 23.5-56.5T160-760h640q33 0 56.5 23.5T880-680v400q0 33-23.5 56.5T800-200H160Zm0-80h640v-400H160v400Zm160-40h320v-80H320v80ZM200-440h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM200-560h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM160-280v-400 400Z"
        /></svg
      >
      {$t["options.button.hotkeys"]}</button
    >
    <button class="language-button" on:click={showLanguageSelectOverlay}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-language-icon)"
        ><path
          d="M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 31.5-155.5t86-127Q252-817 325-848.5T480-880q83 0 155.5 31.5t127 86q54.5 54.5 86 127T880-480q0 82-31.5 155t-86 127.5q-54.5 54.5-127 86T480-80Zm0-82q26-36 45-75t31-83H404q12 44 31 83t45 75Zm-104-16q-18-33-31.5-68.5T322-320H204q29 50 72.5 87t99.5 55Zm208 0q56-18 99.5-55t72.5-87H638q-9 38-22.5 73.5T584-178ZM170-400h136q-3-20-4.5-39.5T300-480q0-21 1.5-40.5T306-560H170q-5 20-7.5 39.5T160-480q0 21 2.5 40.5T170-400Zm216 0h188q3-20 4.5-39.5T580-480q0-21-1.5-40.5T574-560H386q-3 20-4.5 39.5T380-480q0 21 1.5 40.5T386-400Zm268 0h136q5-20 7.5-39.5T800-480q0-21-2.5-40.5T790-560H654q3 20 4.5 39.5T660-480q0 21-1.5 40.5T654-400Zm-16-240h118q-29-50-72.5-87T584-782q18 33 31.5 68.5T638-640Zm-234 0h152q-12-44-31-83t-45-75q-26 36-45 75t-31 83Zm-200 0h118q9-38 22.5-73.5T376-782q-56 18-99.5 55T204-640Z"
        /></svg
      >{$t["options.button.language"]}</button
    >
    <button class="theme-button" on:click={showThemeSelectOverlay}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-theme-icon)"
        ><path
          d="M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 32.5-156t88-127Q256-817 330-848.5T488-880q80 0 151 27.5t124.5 76q53.5 48.5 85 115T880-518q0 115-70 176.5T640-280h-74q-9 0-12.5 5t-3.5 11q0 12 15 34.5t15 51.5q0 50-27.5 74T480-80Zm0-400Zm-220 40q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17Zm120-160q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17Zm200 0q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17Zm120 160q26 0 43-17t17-43q0-26-17-43t-43-17q-26 0-43 17t-17 43q0 26 17 43t43 17ZM480-160q9 0 14.5-5t5.5-13q0-14-15-33t-15-57q0-42 29-67t71-25h70q66 0 113-38.5T800-518q0-121-92.5-201.5T488-800q-136 0-232 93t-96 227q0 133 93.5 226.5T480-160Z"
        /></svg
      >{$t["options.button.theme"]}</button
    >
    <button class="ui-buttons-button" on:click={showUIButtonOptionsOverlay}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-buttons-icon)"
        ><path
          d="M160-400q-33 0-56.5-23.5T80-480q0-33 23.5-56.5T160-560q33 0 56.5 23.5T240-480q0 33-23.5 56.5T160-400Zm0-54q11 0 18.5-7.5T186-480q0-11-7.5-18.5T160-506q-11 0-18.5 7.5T134-480q0 11 7.5 18.5T160-454Zm214 54q-33 0-56.5-23.5T294-480q0-33 23.5-56.5T374-560q33 0 56.5 23.5T454-480q0 33-23.5 56.5T374-400Zm0-54q11 0 18.5-7.5T400-480q0-11-7.5-18.5T374-506q-11 0-18.5 7.5T348-480q0 11 7.5 18.5T374-454Zm212 54q-33 0-56.5-23.5T506-480q0-33 23.5-56.5T586-560q33 0 56.5 23.5T666-480q0 33-23.5 56.5T586-400Zm0-54q11 0 18.5-7.5T612-480q0-11-7.5-18.5T586-506q-11 0-18.5 7.5T560-480q0 11 7.5 18.5T586-454Zm214 54q-33 0-56.5-23.5T720-480q0-33 23.5-56.5T800-560q33 0 56.5 23.5T880-480q0 33-23.5 56.5T800-400Z"
        /></svg
      >{$t["options.button.UI_buttons"]}</button
    >
    <button class="image-name-button" on:click={showImageNameDisplayMenu}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-image-name-display-icon)"
        ><path
          d="M680-360q-17 0-28.5-11.5T640-400v-160q0-17 11.5-28.5T680-600h120q17 0 28.5 11.5T840-560v40h-60v-20h-80v120h80v-20h60v40q0 17-11.5 28.5T800-360H680Zm-300 0v-240h160q17 0 28.5 11.5T580-560v40q0 17-11.5 28.5T540-480q17 0 28.5 11.5T580-440v40q0 17-11.5 28.5T540-360H380Zm60-150h80v-30h-80v30Zm0 90h80v-30h-80v30Zm-320 60v-200q0-17 11.5-28.5T160-600h120q17 0 28.5 11.5T320-560v200h-60v-60h-80v60h-60Zm60-120h80v-60h-80v60Z"
        /></svg
      >{$t["options.button.image_name_display"]}</button
    >
    <button class="edge-indicators-button" on:click={showEdgeIndicatorMenu}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-edge-indicators-icon)"
        ><path
          d="m480-193 85-85 57 56L480-80 338-222l57-56 85 85ZM193-480l85 85-56 57L80-480l142-142 56 57-85 85Zm574 0-85-85 56-57 142 142-142 142-56-57 85-85ZM480-767l-85 85-57-56 142-142 142 142-57 56-85-85Z"
        /></svg
      >{$t["options.button.edge_indicators"]}</button
    >
    <button class="app-window-button" on:click={showAppWindowMenu}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="1.25rem"
        viewBox="0 -960 960 960"
        width="1.25rem"
        fill="var(--color-app-window-icon)"
        ><path
          d="M160-80q-33 0-56.5-23.5T80-160v-360q0-33 23.5-56.5T160-600h80v-200q0-33 23.5-56.5T320-880h480q33 0 56.5 23.5T880-800v360q0 33-23.5 56.5T800-360h-80v200q0 33-23.5 56.5T640-80H160Zm0-80h480v-280H160v280Zm560-280h80v-280H320v120h320q33 0 56.5 23.5T720-520v80Z"
        /></svg
      >{$t["options.button.app_window"]}</button
    >
  </div>
</div>

<style>
  .options-sidebar-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 1rem;
    background-color: var(--color-background);
    transform: translateY(100%);
    transition: transform 0.3s ease-in-out;
    z-index: 20;
  }

  .options-sidebar-overlay.visible {
    transform: translateX(0);
  }

  .options-container {
    display: grid;
    grid-template-columns:
      minmax(10rem, max-content)
      repeat(auto-fill, 10rem) 20%;
    gap: 1rem;
    padding: 1rem;
  }

  h1 {
    color: var(--color-text-primary);
    text-align: center;
  }

  button {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: fit-content;
    gap: 0.5rem;
    padding: 0.5rem 1.25rem;
    border: none;
    border-radius: 9999px;
    background-color: var(--color-button);
    color: var(--color-text-primary);
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: bold;
    transition: all 0.2s ease;
  }

  .hotkeys-button {
    background-color: var(--color-hotkeys-icon-background);
  }

  .hotkeys-button:hover {
    background-color: rgb(from var(--color-hotkeys-icon) r g b / 0.5);
  }

  .language-button {
    background-color: var(--color-language-icon-background);
  }

  .language-button:hover {
    background-color: rgb(from var(--color-language-icon) r g b / 0.5);
  }

  .theme-button {
    background-color: var(--color-theme-icon-background);
  }

  .theme-button:hover {
    background-color: rgb(from var(--color-theme-icon) r g b / 0.5);
  }

  .ui-buttons-button {
    background-color: var(--color-buttons-icon-background);
  }

  .ui-buttons-button:hover {
    background-color: rgb(from var(--color-buttons-icon) r g b / 0.5);
  }

  .edge-indicators-button {
    background-color: var(--color-edge-indicators-icon-background);
  }

  .edge-indicators-button:hover {
    background-color: rgb(from var(--color-edge-indicators-icon) r g b / 0.5);
  }

  .image-name-button {
    background-color: var(--color-image-name-display-icon-background);
  }

  .image-name-button:hover {
    background-color: rgb(from var(--color-image-name-display-icon) r g b / 0.5);
  }

  .app-window-button {
    background-color: var(--color-app-window-icon-background);
  }

  .app-window-button:hover {
    background-color: rgb(from var(--color-app-window-icon) r g b / 0.5);
  }
</style>
