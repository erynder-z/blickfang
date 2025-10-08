<script lang="ts">
  import { get } from "svelte/store";
  import {
    imagePath,
    imageUrl,
    imageExif,
    zoomLevel,
    isExifSidebarVisible,
    isOptionsSidebarVisible,
  } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  onMount(async () => {
    await listen<string>("new_image_path", (event) => {
      imagePath.set(event.payload);
    });
  });

  const openFile = async () => {
    try {
      const [newImageData, newImageExif, newImagePath] =
        await invoke<[string, string, string]>("open_and_read_file");
      if (newImageData) {
        imageUrl.set(newImageData);
        imageExif.set(newImageExif);
        imagePath.set(newImagePath);
      }
    } catch (error) {
      console.error("Failed to open and read file:", error);
    }
  };

  const changeImage = async (direction: "next" | "previous") => {
    const currentPath = get(imagePath);
    if (!currentPath) return;

    try {
      const [newImageData, newImagePath, newImageExif] = await invoke<[string, string, string]>(
        "change_image",
        {
          currentPath,
          direction,
        }
      );
      imageUrl.set(newImageData);
      imagePath.set(newImagePath);
      imageExif.set(newImageExif);
    } catch (error) {
      console.error("Failed to change image:", error);
    }
  };

  const previous = () => changeImage("previous");
  const next = () => changeImage("next");

  const zoomIn = () => {
    zoomLevel.update((level) => level + 0.25);
  };

  const zoomOut = () => {
    zoomLevel.update((level) => {
      const newLevel = level - 0.25;
      return newLevel < 0.1 ? 0.1 : newLevel;
    });
  };

  const toggleExifInfo = () => {
    isExifSidebarVisible.update((isOpen) => !isOpen);
  };

  const toggleOptopns = () => {
    isOptionsSidebarVisible.update((isOpen) => !isOpen);
  };
</script>

<div class="controls-container">
  <!-- Menu -->
  <button on:click={toggleOptopns} aria-label="Toggle Options">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 0 24 24"
      width="1.75rem"
      fill="currentColor"
      ><path d="M0 0h24v24H0V0z" fill="none" /><path
        d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"
      /></svg
    >
  </button>

  <!-- Open File -->
  <button on:click={openFile} aria-label="Open File">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 -960 960 960"
      width="1.75rem"
      fill="currentColor"
      ><path
        d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v240h-80v-200H520v-200H240v640h360v80H240Zm638 15L760-183v89h-80v-226h226v80h-90l118 118-56 57Zm-638-95v-640 640Z"
      /></svg
    >
  </button>

  <!-- Previous -->
  <button on:click={previous} aria-label="Previous Image">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 -960 960 960"
      width="1.75rem"
      fill="currentColor"
      ><path d="m313-440 224 224-57 56-320-320 320-320 57 56-224 224h487v80H313Z" /></svg
    >
  </button>

  <!-- Next -->
  <button on:click={next} aria-label="Next Image">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 -960 960 960"
      width="1.75rem"
      fill="currentColor"
      ><path d="M647-440H160v-80h487L423-744l57-56 320 320-320 320-57-56 224-224Z" /></svg
    >
  </button>

  <!-- Zoom In -->
  <button on:click={zoomIn} aria-label="Zoom In">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 -960 960 960"
      width="1.75rem"
      fill="currentColor"
      ><path
        d="M784-120 532-372q-30 24-69 38t-83 14q-109 0-184.5-75.5T120-580q0-109 75.5-184.5T380-840q109 0 184.5 75.5T640-580q0 44-14 83t-38 69l252 252-56 56ZM380-400q75 0 127.5-52.5T560-580q0-75-52.5-127.5T380-760q-75 0-127.5 52.5T200-580q0 75 52.5 127.5T380-400Zm-40-60v-80h-80v-80h80v-80h80v80h80v80h-80v80h-80Z"
      /></svg
    >
  </button>

  <!-- Zoom Out -->
  <button on:click={zoomOut} aria-label="Zoom Out">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 -960 960 960"
      width="1.75rem"
      fill="currentColor"
      ><path
        d="M784-120 532-372q-30 24-69 38t-83 14q-109 0-184.5-75.5T120-580q0-109 75.5-184.5T380-840q109 0 184.5 75.5T640-580q0 44-14 83t-38 69l252 252-56 56ZM380-400q75 0 127.5-52.5T560-580q0-75-52.5-127.5T380-760q-75 0-127.5 52.5T200-580q0 75 52.5 127.5T380-400ZM280-540v-80h200v80H280Z"
      /></svg
    >
  </button>

  <!-- Info -->
  <button on:click={toggleExifInfo} aria-label="Toggle Info">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="1.75rem"
      viewBox="0 -960 960 960"
      width="1.75rem"
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
    background-color: rgb(from var(--color-background) r g b / 0.5);
    color: var(--color-text-primary);
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 50%;
    transition: all 0.2s ease-in-out;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  button:hover {
    filter: brightness(1.3);
    transform: scale(1.05);
  }
</style>
