<script lang="ts">
  import {
    activeActions,
    appConfig,
    imageUrl,
    isSaveAsMenuVisible,
    isZoomModifierUpActive,
    isZoomModifierDownActive,
  } from "$lib/stores";
  import {
    nextImage,
    openFile,
    previousImage,
    startZoomIn,
    startZoomOut,
    stopContinuousZoom,
    toggleExif,
    toggleFullscreen,
  } from "$lib/core/commands";

  const buttonSizes: Record<string, string> = {
    large: "2.5rem",
    small: "2rem",
  };

  $: size = buttonSizes[$appConfig.toolbarButtonSize];

  const showSaveAsMenu = () => {
    isSaveAsMenuVisible.update((v) => !v);
  };
</script>

<div
  class="controls-container"
  style:display={$appConfig.toolbarButtonSize === "hide" ? "none" : "flex"}
>
  <!-- Open File -->
  <button
    on:click={openFile}
    aria-label="Open File"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("openFile")}
  >
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" fill="currentColor"
      ><path
        d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v240h-80v-200H520v-200H240v640h360v80H240Zm638 15L760-183v89h-80v-226h226v80h-90l118 118-56 57Zm-638-95v-640 640Z"
      /></svg
    >
  </button>

  <!-- Save As -->
  <button
    on:click={showSaveAsMenu}
    aria-label="Save As"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("saveImageAs")}
    disabled={!$imageUrl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path
        d="M840-680v480q0 33-23.5 56.5T760-120H200q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h480l160 160Zm-80 34L646-760H200v560h560v-446ZM480-240q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35ZM240-560h360v-160H240v160Zm-40-86v446-560 114Z"
      /></svg
    >
  </button>

  <!-- Previous -->
  <button
    on:click={previousImage}
    aria-label="Previous Image"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("previousImage")}
    disabled={!$imageUrl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path d="m313-440 224 224-57 56-320-320 320-320 57 56-224 224h487v80H313Z" /></svg
    >
  </button>

  <!-- Next -->
  <button
    on:click={nextImage}
    aria-label="Next Image"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("nextImage")}
    disabled={!$imageUrl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path d="M647-440H160v-80h487L423-744l57-56 320 320-320 320-57-56 224-224Z" /></svg
    >
  </button>

  <!-- Zoom In -->
  <button
    on:mousedown={startZoomIn}
    on:mouseup={stopContinuousZoom}
    on:mouseleave={stopContinuousZoom}
    aria-label="Zoom In"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("zoomIn")}
    class:stronger={$isZoomModifierUpActive}
    class:subdued={$isZoomModifierDownActive}
    disabled={!$imageUrl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path
        d="M784-120 532-372q-30 24-69 38t-83 14q-109 0-184.5-75.5T120-580q0-109 75.5-184.5T380-840q109 0 184.5 75.5T640-580q0 44-14 83t-38 69l252 252-56 56ZM380-400q75 0 127.5-52.5T560-580q0-75-52.5-127.5T380-760q-75 0-127.5 52.5T200-580q0 75 52.5 127.5T380-400Zm-40-60v-80h-80v-80h80v-80h80v80h80v80h-80v80h-80Z"
      /></svg
    >
  </button>

  <!-- Zoom Out -->
  <button
    on:mousedown={startZoomOut}
    on:mouseup={stopContinuousZoom}
    on:mouseleave={stopContinuousZoom}
    aria-label="Zoom Out"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("zoomOut")}
    class:stronger={$isZoomModifierUpActive}
    class:subdued={$isZoomModifierDownActive}
    disabled={!$imageUrl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path
        d="M784-120 532-372q-30 24-69 38t-83 14q-109 0-184.5-75.5T120-580q0-109 75.5-184.5T380-840q109 0 184.5 75.5T640-580q0 44-14 83t-38 69l252 252-56 56ZM380-400q75 0 127.5-52.5T560-580q0-75-52.5-127.5T380-760q-75 0-127.5 52.5T200-580q0 75 52.5 127.5T380-400ZM280-540v-80h200v80H280Z"
      /></svg
    >
  </button>

  <!-- Fullscreen -->
  <button
    on:click={toggleFullscreen}
    aria-label="Toggle Fullscreen"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("toggleFullscreen")}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path
        d="M120-120v-200h80v120h120v80H120Zm520 0v-80h120v-120h80v200H640ZM120-640v-200h200v80H200v120h-80Zm640 0v-120H640v-80h200v200h-80Z"
      /></svg
    >
  </button>

  <!-- Info -->
  <button
    on:click={toggleExif}
    aria-label="Toggle Info"
    style="--btn-size: {size}"
    class:active={$activeActions.includes("toggleExif")}
    disabled={!$imageUrl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="fit-content"
      viewBox="0 -960 960 960"
      width="fit-content"
      fill="currentColor"
      ><path
        d="M440-280h80v-240h-80v240Zm40-320q17 0 28.5-11.5T520-640q0-17-11.5-28.5T480-680q-17 0-28.5 11.5T440-640q0 17 11.5 28.5T480-600Zm0 520q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"
      /></svg
    >
  </button>
</div>

<style>
  .controls-container {
    position: absolute;
    top: 0;
    width: 100%;
    display: flex;
    justify-content: center;
    gap: 1rem;
    padding: 1rem;
    background-color: transparent;
    z-index: 10;
  }

  button {
    width: var(--btn-size);
    height: var(--btn-size);
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background-color: rgb(from var(--color-background) r g b / 0.5);
    color: var(--color-text-primary);
    border: none;
    transition: all 0.2s ease-in-out;
  }

  button.active {
    background-color: var(--color-accent);
    color: var(--color-text-tertiary);
  }

  button.active.subdued {
    filter: brightness(0.7);
  }

  button.active.stronger {
    filter: brightness(1.5);
  }

  button:hover {
    filter: brightness(1.3);
    transform: scale(1.05);
  }

  button:disabled {
    opacity: 0.4;
    filter: none;
    transform: none;
  }

  button svg {
    width: 100%;
    height: auto;
  }
</style>
